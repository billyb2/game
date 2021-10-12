use std::intrinsics::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use bevy::prelude::*;

use crate::*;
use game_types::*;
use game_types::player_attr::DEFAULT_PLAYER_SPEED;
use helper_functions::{u128_to_f32_u8, f32_u8_to_u128};

//TODO: Molotovs
pub fn move_objects(mut commands: Commands, mut physics_pipeline: ResMut<PhysicsPipeline>, mut island_manager: ResMut<IslandManager>, mut broad_phase: ResMut<BroadPhase>, mut narrow_phase: ResMut<NarrowPhase>, mut joint_set: ResMut<JointSet>, mut ccd_solver: ResMut<CCDSolver>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, mut movable_objects: Query<(Entity, &RigidBodyHandle, &ColliderHandle, &mut Sprite, &mut Transform, Option<&mut Health>, Option<&ProjectileIdent>, Option<&PlayerID>, Option<&mut DamageSource>, Option<&ProjectileType>, Option<&mut PlayerSpeed>, Option<&Speed>, Option<&mut DistanceTraveled>, Option<&MaxDistance>, &mut Handle<ColorMaterial>)>, mut deathmatch_score: ResMut<DeathmatchScore>, mut death_event: EventWriter<DeathEvent>, my_player_id: Res<MyPlayerID>) {
    movable_objects.iter_mut().for_each(|(entity, rigid_body_handle, collider_handle, mut sprite, mut transform, mut health, shot_from, player_id, mut damage_source, projectile_type, mut p_speed, speed, mut distance_traveled, max_distance, mut color_material)| {

        if let Some(rigid_body) = rigid_body_set.get_mut(*rigid_body_handle) {
            // Update the rigid body's sprite to the correct translation            
            let rigid_body_translation = rigid_body.translation().component_mul(&Vector2::new(250.0, 250.0));
            transform.translation = Vec3::new(rigid_body_translation.x, rigid_body_translation.y, transform.translation.z);

            let contacts = narrow_phase.contacts_with(*collider_handle);

            // Increase the size of speedballs
            // Only speedballs have a negative linear damping, meaning they increase in speed over time
            // TODO: Replace this with projectile_type == Speedball
            if rigid_body.linear_damping() < 0.0 {
                let mut linvel = rigid_body.linvel().abs().amax() * 25.0;
                // The maximum speed of Speedball projectiles is 50, so that they aren't horribly difficult to doge
                linvel = match linvel > 50.0 {
                    true => 50.0,
                    false => linvel,
                };

                sprite.size = Vec2::splat(linvel);

                let collider = collider_set.get_mut(*collider_handle).unwrap();
                collider.set_shape(SharedShape::cuboid(linvel / 500.0, linvel / 500.0));

                let (_damage, proj_info) = u128_to_f32_u8(collider.user_data);                
                // Speedballs do more damage as their velocity increases
                let new_damage = linvel * 1.5;
                collider.user_data = f32_u8_to_u128(new_damage, proj_info);

            }

            if let Some(max_distance) = max_distance {
                if let Some(distance_traveled) = distance_traveled.as_mut() {
                    let speed = speed.as_ref().unwrap().0;
                    distance_traveled.0 += speed;

                    if distance_traveled.0 >= max_distance.0 {
                        let projectile_type = projectile_type.as_ref().unwrap();

                        if **projectile_type != ProjectileType::Molotov {
                            rigid_body_set.remove(*rigid_body_handle, &mut island_manager, &mut collider_set, &mut joint_set);
                            commands.entity(entity).despawn_recursive();

                        } else {
                            rigid_body.set_linvel(Vector2::new(0.0, 0.0), false);

                        }
                    }

                }
            }

            contacts.for_each(|contact_pair| {
                // Finds the collider handle that isn't equal to the current collider handle, and then grabs a reference to the actual collider object
                let other_collider_handle = match contact_pair.collider1 != *collider_handle {
                    true => contact_pair.collider1,
                    false => contact_pair.collider2,

                };

                if let Some(other_collider) = collider_set.get(other_collider_handle) {
                    let hit_player = other_collider.user_data == u128::MAX;
                    let hit_map_object = other_collider.user_data == 0;

                    // Deal damage to objects that can take damage
                    if let Some(health) = &mut health {
                        if other_collider.user_data != 0  && other_collider.user_data != u128::MAX{
                            let (damage, (shot_from, projectile_type)) = u128_to_f32_u8(other_collider.user_data);
                            let player_id = player_id.unwrap().0; 

                            if shot_from != player_id {
                                let new_health = health.0 - damage;
                                let player_died = new_health <= 0.0;

                                // Only directly edit the local health of our player, other players send their health over the net
                                if health.0 > 0.0 && my_player_id.0.as_ref().unwrap().0 == player_id {
                                    damage_source.as_mut().unwrap().0 = Some(shot_from);

                                    health.0 = match player_died {
                                        true => {
                                            death_event.send(DeathEvent(player_id));
                                            // The player who shot the bullet has their score increased 
                                            *deathmatch_score.0.get_mut(&shot_from).unwrap() += 1;
                                            0.0
                                        },
                                        false => {
                                            let speed = p_speed.as_mut().unwrap();
                                            // Only do the conversion if the game needs to figure out the projectile type
                                            let projectile_type: ProjectileType = projectile_type.into();          

                                            // Slow down players for X amount of seconds
                                            if projectile_type == ProjectileType::PulseWave {
                                                speed.0 =  unsafe { fmul_fast(speed.0, 0.25) };
                                                commands.entity(entity).insert(SlowedDown(Timer::from_seconds(2.5, false)));


                                            } else if projectile_type == ProjectileType::MolotovLiquid && speed.0 >= DEFAULT_PLAYER_SPEED {
                                                speed.0 = unsafe { fmul_fast(speed.0, 0.65) };
                                                commands.entity(entity).insert(SlowedDown(Timer::from_seconds(2.0, false)));

                                            }

                                            new_health

                                        },
                                    };

                                }

                            }
                        }

                    // Destroy any projectiles
                    } else if shot_from.is_some() && (hit_map_object || hit_player) && (*projectile_type.unwrap() != ProjectileType::PulseWave || hit_player) {
                        // Projectiles upon collision with any object destroy themselves, except for collisions with other bullets
                        rigid_body_set.remove(*rigid_body_handle, &mut island_manager, &mut collider_set, &mut joint_set);
                        commands.entity(entity).despawn_recursive();

                    }

                }

            });

        }
    });

    const GRAVITY: Vector2<f32> = Vector2::new(0.0, 0.0);
    const INTEGRATION_PARAMETERS: IntegrationParameters = IntegrationParameters {
        dt: 1.0 / 60.0,
        min_ccd_dt: 1.0 / 60.0 / 100.0,
        erp: 0.2,
        joint_erp: 0.2,
        velocity_solve_fraction: 1.0,
        velocity_based_erp: 0.0,
        warmstart_coeff: 1.0,
        warmstart_correction_slope: 10.0,
        allowed_linear_error: 0.005,
        prediction_distance: 0.002,
        allowed_angular_error: 0.001,
        max_linear_correction: 0.2,
        max_angular_correction: 0.2,
        max_velocity_iterations: 4,
        max_position_iterations: 1,
        min_island_size: 128,
        max_ccd_substeps: 20,
    };

    physics_pipeline.step(
        &GRAVITY,
        &INTEGRATION_PARAMETERS,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
        &mut ccd_solver,
        &(),
        &()
    );

}
