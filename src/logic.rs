use std::intrinsics::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use bevy::prelude::*;

use game_types::*;
use game_types::player_attr::DEFAULT_PLAYER_SPEED;
use helper_functions::{u128_to_f32_u8, f32_u8_to_u128};

pub fn move_objects(mut commands: Commands, mut physics_pipeline: ResMut<PhysicsPipeline>, mut island_manager: ResMut<IslandManager>, mut broad_phase: ResMut<BroadPhase>, mut narrow_phase: ResMut<NarrowPhase>, mut joint_set: ResMut<JointSet>, mut ccd_solver: ResMut<CCDSolver>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, mut movable_objects: Query<(Entity, &RigidBodyHandle, &ColliderHandle, &mut Sprite, &mut Transform, Option<&mut Health>, Option<&ProjectileIdent>, Option<&PlayerID>, Option<&mut DamageSource>, Option<&mut ProjectileType>, Option<&mut PlayerSpeed>, Option<&Speed>, Option<&mut DistanceTraveled>, Option<&MaxDistance>, &mut Handle<ColorMaterial>)>, mut deathmatch_score: ResMut<DeathmatchScore>, mut death_event: EventWriter<DeathEvent>, proj_materials: Res<ProjectileMaterials>, mut widow_maker_heals: ResMut<WidowMakerHeals>, local_players: Res<LocalPlayers>) {
    movable_objects.iter_mut().for_each(|(entity, rigid_body_handle, collider_handle, mut sprite, mut transform, mut health, shot_from, player_id, mut damage_source, mut projectile_type, mut p_speed, speed, mut distance_traveled, max_distance, mut material)| {

        if let Some(player_id) = player_id.as_ref() {
            if let Some(health_to_heal) = widow_maker_heals.0.remove(&player_id.0) {
                let health = health.as_mut().unwrap();
                // The health can only go as high as 150.0
                let new_health = health.0 + health_to_heal;

                if new_health <= 150.0 {
                    health.0 = new_health;
                }

            }
        }


        let mut should_remove_rigid_body = false;

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
                        let projectile_type = projectile_type.as_mut().unwrap();

                        if !rigid_body.is_static() && **projectile_type != ProjectileType::Molotov {
                            should_remove_rigid_body = true;

                        } else if **projectile_type == ProjectileType::Molotov {
                            let collider = collider_set.get_mut(*collider_handle).unwrap();
                            *material = proj_materials.molotov_liquid.clone();
                            **projectile_type = ProjectileType::MolotovLiquid;
                            sprite.size = Vec2::splat(200.0);
                            collider.set_shape(SharedShape::ball(200.0 / 500.0));
                            rigid_body.set_linvel(Vector2::new(0.0, 0.0), false);
                            rigid_body.set_body_type(RigidBodyType::Static);
                            collider.set_collision_groups(InteractionGroups::new(0b0010, 0b0100));

                            commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(45.0, false)));

                            let (_damage, (shot_from, _proj_type)) = u128_to_f32_u8(rigid_body.user_data);

                            rigid_body.user_data = f32_u8_to_u128(0.0, (shot_from, ProjectileType::MolotovLiquid.into()));
                            collider.user_data = f32_u8_to_u128(0.0, (shot_from, ProjectileType::MolotovLiquid.into()));

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
                                
                                let projectile_type: ProjectileType = projectile_type.into();          

                                let player_is_local = local_players.0.contains(&player_id);

                                if projectile_type == ProjectileType::WidowMaker {
                                    if let Some(health_to_heal) = widow_maker_heals.0.get_mut(&shot_from) {
                                        *health_to_heal += damage * 1.5;

                                    } else {
                                        widow_maker_heals.0.insert(shot_from, damage * 1.5);

                                    }

                                }
                                // Only directly edit the local health of our player, other players send their health over the net
                                if health.0 > 0.0 && player_is_local {
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
                    } else if shot_from.is_some() {
                        let projectile_type_ref = **projectile_type.as_ref().unwrap();
                        if 
                        // None of the molov type should be destroyed when hit
                        (projectile_type_ref != ProjectileType::MolotovLiquid && projectile_type_ref != ProjectileType::Molotov && projectile_type_ref != ProjectileType::MolotovFire)
                        // the projecitle hit a wall or a player
                         && (hit_map_object || hit_player) 
                         // If it's a pulsewave, it has to have hit a player to dissapear
                         && (projectile_type_ref != ProjectileType::PulseWave || hit_player) {
                                // Projectiles upon collision with any object destroy themselves, except for collisions with other bullets
                                should_remove_rigid_body = true;

                        // Molotov liquid only becomes molotov fire when it hits something other than a player or a map object (almost always a projectile)
                        } else if projectile_type_ref == ProjectileType::MolotovLiquid && !(hit_player || hit_map_object) {
                            let collider = collider_set.get_mut(*collider_handle).unwrap();

                           let (_damage, (shot_from, _proj_type)) = u128_to_f32_u8(rigid_body.user_data);

                            // 75.0 / 60.0 * 60 FPS = 75 damage per second
                            const MOLOTOV_FIRE_DAMAGE: f32 = 75.0 / 60.0;

                            rigid_body.user_data = f32_u8_to_u128(MOLOTOV_FIRE_DAMAGE, (shot_from, ProjectileType::MolotovFire.into()));
                            collider.user_data = f32_u8_to_u128(MOLOTOV_FIRE_DAMAGE, (shot_from, ProjectileType::MolotovFire.into()));

                            *material = proj_materials.molotov_fire.clone();
                            **projectile_type.as_mut().unwrap() = ProjectileType::MolotovFire;
                            sprite.size = Vec2::splat(400.0);
                            collider.set_shape(SharedShape::ball(400.0 / 500.0));

                            commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(5.0, false)));
                            
                        }

                    }

                }

            });

        }

        if should_remove_rigid_body {
            rigid_body_set.remove(*rigid_body_handle, &mut island_manager, &mut collider_set, &mut joint_set);
            commands.entity(entity).despawn_recursive();
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
        max_position_iterations: 4,
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

pub fn destruction_timer(mut commands: Commands, q: Query<(Entity, &DestructionTimer, &RigidBodyHandle)>, mut rigid_body_set: ResMut<RigidBodySet>, mut island_manager: ResMut<IslandManager>, mut collider_set: ResMut<ColliderSet>, mut joint_set: ResMut<JointSet>) {
    q.for_each(|(e, d_timer, rigid_body_handle)| {
        if d_timer.0.finished() {
            rigid_body_set.remove(*rigid_body_handle, &mut island_manager, &mut collider_set, &mut joint_set);
            commands.entity(e).despawn_recursive();
        }

    });

}


//TODO: have different player shaders and set them on ability change in this fn
pub fn set_player_materials(mut players: Query<(&Model, &mut Handle<ColorMaterial>, &mut Sprite), Changed<Model>>, player_materials: Res<Skin>) {
    players.for_each_mut(|(model, mut skin, mut sprite)| {
        let model_u8: u8 = (*model).into();
        let (handle, size) = player_materials.player[model_u8 as usize].clone();

        *skin = handle;
        sprite.size = size;

    });

}
