use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use bevy::prelude::*;

use crate::*;
use game_types::*;
use helper_functions::{u128_to_f32_u8, f32_u8_to_u128};

//TODO: StopAfterDistance
//TODO: Molotovs
//TODO: Warping, Ghosting
//TODO: PulseWave
//TODO: Send damage net events
pub fn move_objects(mut commands: Commands, mut physics_pipeline: ResMut<PhysicsPipeline>, mut island_manager: ResMut<IslandManager>, mut broad_phase: ResMut<BroadPhase>, mut narrow_phase: ResMut<NarrowPhase>, mut joint_set: ResMut<JointSet>, mut ccd_solver: ResMut<CCDSolver>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, mut movable_objects: Query<(Entity, &RigidBodyHandle, &ColliderHandle, &mut Sprite, &mut Transform, Option<&mut Health>, Option<&ProjectileIdent>, Option<&PlayerID>, Option<&mut DamageSource>)>, mut deathmatch_score: ResMut<DeathmatchScore>, mut death_event: EventWriter<DeathEvent>, my_player_id: Res<MyPlayerID>) {
    // Update the locations of all rigid bodies to their graphics counterparts
    movable_objects.iter_mut().for_each(|(entity, rigid_body_handle, collider_handle, mut sprite, mut transform, mut health, shot_from, player_id, mut damage_source)| {
        if let Some(rigid_body) = rigid_body_set.get(*rigid_body_handle) {
            // Update the rigid body's sprite to the correct translation            
            let rigid_body_translation = rigid_body.translation().component_mul(&Vector2::new(250.0, 250.0));
            transform.translation = Vec3::new(rigid_body_translation.x, rigid_body_translation.y, transform.translation.z);

            let contacts = narrow_phase.contacts_with(*collider_handle);

            // Increase the size of speedballs
            // Only speedballs have a negative linear damping, meaning they increase in speed over time
            // TODO: Replace this with projectile_type == Speedball
            if rigid_body.linear_damping() < 0.0 {
                let mut linvel = rigid_body.linvel().abs().amax() * 25.0;
                linvel = match linvel > 50.0 {
                    true => 50.0,
                    false => linvel,
                };

                sprite.size = Vec2::splat(linvel);

                let collider = collider_set.get_mut(*collider_handle).unwrap();
                collider.set_shape(SharedShape::cuboid(linvel / 500.0, linvel / 500.0));

                let (_damage, shot_from) = u128_to_f32_u8(collider.user_data);
                // Speedballs do more damage as their velocity increases
                let new_damage = linvel * 1.5;
                collider.user_data = f32_u8_to_u128(new_damage, shot_from);


            }

            contacts.for_each(|contact_pair| {
                // Finds the collider handle that isn't equal to the current collider handle, and then grabs a reference to the actual collider object
                let other_collider_handle = match contact_pair.collider1 != *collider_handle {
                    true => contact_pair.collider1,
                    false => contact_pair.collider2,

                };

                if let Some(other_collider) = collider_set.get(other_collider_handle) {
                    // Deal damage to objects that can take damage
                    if let Some(health) = &mut health {

                        if other_collider.user_data != 0 {
                            let (damage, shot_from) = u128_to_f32_u8(other_collider.user_data);
                            let player_id = player_id.unwrap().0; 

                            if shot_from != player_id {

                                let new_health = health.0 - damage;
                                let player_died = new_health <= 0.0;

                                // Only directly edit the local health of our player, other players send their health over the net
                                if health.0 > 0.0 && my_player_id.0.as_ref().unwrap().0 == player_id {
                                    damage_source.as_mut().unwrap().0 = Some(shot_from);

                                    if player_died {
                                        health.0 = 0.0;
                                        death_event.send(DeathEvent(player_id));
                                        // The player who shot the bullet has their score increased 
                                        *deathmatch_score.0.get_mut(&shot_from).unwrap() += 1;

                                    } else {
                                        health.0 = new_health;

                                    }
                                }

                            }
                        }

                    // Destroy any projectiles
                    } else if shot_from.is_some() {
                        // Projectiles upon collision with any object destroy themselves, except for collisions with other bullets TODO
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
