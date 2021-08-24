// Game math logic (basically move_objects)
use std::intrinsics::*;
use std::ops::DerefMut;

use crate::*;
use bevy::prelude::*;
use helper_functions::{collide, collide_rect_circle, out_of_bounds};

const DESIRED_TICKS_PER_SECOND: f32 = 60.0;

//TODO: Turn RequestedMovement into an event
//TODO: Maybe make all the bullet collisions into its own seperate system? (for readability and maybe performance)
//TODO: Make it so molotovs are map objects and not bullets
// Move objects will first validate whether a movement can be done, and if so move them
// Probably the biggest function in the entire project, since it's a frankenstein amalgamation of multiple different functions from the original ggez version. It basically does damage for bullets, and moves any object that requested to be moved
#[allow(clippy::too_many_arguments)]
pub fn move_objects(mut commands: Commands, mut player_movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, Option<&mut DistanceTraveled>, &Sprite, &PlayerID, &mut Health, &Ability, &mut Visible, &mut PlayerSpeed, &Phasing, &mut Alpha), Without<ProjectileIdent>>, mut projectile_movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, Option<&mut DistanceTraveled>, &mut Sprite, &mut ProjectileType, &ProjectileIdent, &mut Damage, &mut Handle<ColorMaterial>, Option<&DestructionTimer>), (Without<PlayerID>, With<ProjectileIdent>)>, mut maps: ResMut<Maps>, map_crc32: Res<MapCRC32>, time: Res<Time>, mut death_event: EventWriter<DeathEvent>, materials: Res<ProjectileMaterials>, mut wall_event: EventWriter<DespawnWhenDead>, mut deathmatch_score: ResMut<DeathmatchScore>, my_player_id: Res<MyPlayerID>, mut net: ResMut<NetworkResource>, player_entity: Res<HashMap<u8, Entity>>, asset_server: Res<AssetServer>) {

    let mut liquid_molotovs: Vec<(Vec2, f32)> = Vec::with_capacity(5);

    let map = maps.0.get_mut(&map_crc32.0).unwrap();

    let stop_after_distance = 
    #[inline(always)]
    |movement_speed: &mut f32, distance_traveled: &mut f32, distance_to_stop_at: f32| {
        *distance_traveled = unsafe { fadd_fast(movement_speed.abs(), *distance_traveled) };

        if *distance_traveled >= distance_to_stop_at {
            *movement_speed = 0.0;

        }
    };

    player_movements.for_each_mut(|(_entity, mut object, mut movement, movement_type, mut distance_traveled, sprite, _player_id, health, _ability, _visible, _player_speed, phasing, _alpha)| {
        if movement.speed != 0.0 && health.0 != 0.0 {
            // The next potential movement is multipled by the amount of time that's passed since the last frame times how fast I want the game to be, so that the game doesn't run slower even with lag or very fast PC's, so the game moves at the same frame rate no matter the power of each device
            let mut lag_compensation = unsafe { fmul_fast(DESIRED_TICKS_PER_SECOND, time.delta_seconds()) };


            if lag_compensation > 4.0 {
                lag_compensation = 4.0;

            }

            let speed = unsafe { fmul_fast(movement.speed, lag_compensation) };

            let angle_trig = Vec2::from_slice(&[movement.angle.cos(), movement.angle.sin()]);
            let speed_simd = Vec2::splat(speed);

            let translation = object.translation.truncate();

            let next_potential_pos = speed_simd * angle_trig + translation;

            if phasing.0 || (!out_of_bounds(next_potential_pos, sprite.size, map.size)) {
                let collision = map.collision_no_damage(translation, sprite.size, speed, angle_trig);

                let next_potential_pos = next_potential_pos.to_array();

                if !collision.0 {
                    object.translation.x = next_potential_pos[0];

                }

                if !collision.1 {
                    object.translation.y = next_potential_pos[1];

                }

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        stop_after_distance(&mut movement.speed, unsafe { &mut distance_traveled.as_mut().unwrap_unchecked().0 }, *distance_to_stop_at);

                    },
                }

            } else {
                movement.speed = 0.0;

            }
        }
    });

    projectile_movements.for_each_mut(|(_, mut object, mut movement, movement_type, mut distance_traveled, mut sprite, projectile_type, shot_from, mut damage, _, _)| {
        if movement.speed != 0.0 || *projectile_type == ProjectileType::MolotovFire || *projectile_type == ProjectileType::MolotovLiquid {
            if *projectile_type == ProjectileType::MolotovLiquid {
                liquid_molotovs.push((object.translation.truncate(), sprite.size.x));

            }

            let lag_compensation = unsafe { fmul_fast(DESIRED_TICKS_PER_SECOND, time.delta_seconds()) };

            let speed = unsafe { fmul_fast(movement.speed, lag_compensation) };


            let translation = object.translation.truncate();
            let angle_trig = Vec2::new(movement.angle.cos(), movement.angle.sin());
            let speed_simd = Vec2::splat(speed);

            let next_potential_pos = speed_simd * angle_trig + translation;
            let mut player_collision = false;

            // Check to see if a player-projectile collision takes place
            player_movements.for_each_mut(|(entity, player, mut player_movement, _, _, player_sprite, player_id, mut health, ability, _visible, mut player_speed, _phasing, mut alpha) |{
                // Player bullets cannot collide with the player who shot them (thanks @Susorodni for the idea)
                // Checks that players aren't already dead as well lol
                // Check to see if a player-projectile collision takes place                
                let collision = {
                    let collision = collide(translation, sprite.size, player.translation.truncate(), player_sprite.size, movement.speed, angle_trig);

                    collision.0 || collision.1

                };

                if health.0 > 0.0 && ((*projectile_type != ProjectileType::MolotovFire && *projectile_type != ProjectileType::MolotovLiquid && collision) || (*projectile_type == ProjectileType::MolotovFire && collide_rect_circle(player.translation.truncate(), player_sprite.size, next_potential_pos, sprite.size.x))) && (player_id.0 != shot_from.0 || *projectile_type == ProjectileType::MolotovFire) {
                    if *projectile_type == ProjectileType::TractorBeam {
                        const BEAM_STRENGTH: f32 = 6.5;
                        let angle_add = Vec2::new(player_movement.angle.cos(), player_movement.angle.sin()) + Vec2::new(movement.angle.cos(), movement.angle.sin());
                        player_movement.angle = angle_add.y.atan2(angle_add.x);

                        player_movement.speed += BEAM_STRENGTH * -speed.signum();

                    }

                    
                    if *ability == Ability::Cloak && (alpha.value - 1.0).abs() > f32::EPSILON {
                        alpha.value = 1.0;

                    }

                    // Players can only do damage to other players if they receive a network event about it, if they don't, then damage can only happen to themselves
                    if let Some(my_player_id) = &my_player_id.0 {
                        let player_died = (health.0 - damage.0) <= 0.0;

                        commands.spawn_bundle(Text2dBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: format!("{:.0}", damage.0),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 11.0,
                                            color: match player_died {
                                                false => Color::WHITE,
                                                true => Color::RED,
    
                                            },
                                        },
                                    },
                                ],
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec2::from_slice(&next_potential_pos.to_array()).extend(5.0)),
                            ..Default::default()
    
                        })
                        .insert(DamageTextTimer(Timer::from_seconds(2.0, false)));
    
                        
                        if my_player_id.0 == player_id.0 {
                            net.broadcast_message(([my_player_id.0, shot_from.0], damage.0));
                            
                            if player_died {
                                health.0 = 0.0;
                                death_event.send(DeathEvent(player_id.0));
                                // The player who shot the bullet has their score increased 
                                *deathmatch_score.0.get_mut(&shot_from.0).unwrap() += 1;

                            } else {
                                health.0 -= damage.0;

                                if *projectile_type == ProjectileType::PulseWave {
                                    player_speed.0 =  unsafe { fmul_fast(player_speed.0, 0.25) };

                                    commands.entity(entity).insert(SlowedDown(Timer::from_seconds(2.0, false)));

                                }

                            }

                        }
                    }

                    player_collision = true;

                }

            });

            let (wall_collision, health_and_coords) = match *projectile_type {
                // Pulsewaves and tractor beams move through walls
                ProjectileType::PulseWave | ProjectileType::TractorBeam => (false, None),
                _ =>  map.collision(translation, sprite.size, damage.0, speed, Vec2::new(movement.angle.cos(), movement.angle.sin())),

            };

            if let Some((health, coords)) = health_and_coords {
                wall_event.send(DespawnWhenDead {
                    health,
                    coords,

                });

            }

            // Pulsewaves move through walls, but not players
            if !(player_collision || wall_collision) {
                object.translation = Vec2::from_slice(&next_potential_pos.to_array()).extend(3.0);

                // Gotta make sure that it's both a projectile and has a projectile type, since guns also have a projectile type
                // If you don't do the is_projectile bit, you get a great bug where a player's size will increase as it moves (if they're using the speedball weapon)
                // The speedball's weapon speeds up and gets bigger
                if *projectile_type == ProjectileType::Speedball {
                    movement.speed = unsafe { fmul_fast(movement.speed, 1.1) };
                    sprite.size = unsafe { Vec2::new(fmul_fast(sprite.size.x, 1.03), fmul_fast(sprite.size.y, 1.03))};

                    if damage.0 <= 80.0 {
                        damage.0 += distance_traveled.as_ref().unwrap().0  / 60.0;

                    }

                } else if *projectile_type == ProjectileType::Flame && sprite.size.x <= 20.0 {
                    sprite.size *= 1.3;

                }

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        stop_after_distance(&mut movement.speed, unsafe { &mut distance_traveled.as_mut().unwrap_unchecked().0 }, *distance_to_stop_at)

                    },
                }

            } else {
                // Stop any bullets that hit players or walls
                movement.speed = 0.0;

            }

        }
    });


    // Remove all stopped bullets
    projectile_movements.for_each_mut(|(entity, _, req_mov, _, _, mut sprite, mut projectile_type, _, _, mut material, destruction_timer)| {
        if req_mov.speed == 0.0 {
            if *projectile_type == ProjectileType::Molotov {
                // Once the molotov reaches it's destination, or hits a player, it becomes molotov liquid, waiting to be lit by an Inferno player
                *projectile_type.deref_mut() = ProjectileType::MolotovLiquid;
                *material.deref_mut() = materials.molotov_liquid.clone();
                sprite.deref_mut().size = Vec2::new(175.0, 175.0);
                // Molotov liquid disappears after a little while
                commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(45.0, false)));

            } else if likely(*projectile_type != ProjectileType::MolotovLiquid && *projectile_type != ProjectileType::MolotovFire || ((*projectile_type == ProjectileType::MolotovLiquid || *projectile_type == ProjectileType::MolotovFire) && destruction_timer.unwrap().0.finished())) {
                commands.entity(entity).despawn_recursive();

            }
        }
    });

    let mut molotovs_to_be_lit_on_fire: Vec<(Vec2, f32)> = Vec::new();

    // Find molotovs that are to be lit on fire
    projectile_movements.for_each_mut(|(_, proj_coords, _, _, _, sprite, projectile_type, shot_from, _, _, _) |{
        if *projectile_type != ProjectileType::MolotovFire && *projectile_type != ProjectileType::MolotovLiquid {
            // Firstly, find if the player ID is that of an inferno
            let (_entity, _, _, _, _, _, _, _, _ability, _, _player_speed, _phasing, _alpha) = player_movements.get_mut(*player_entity.get(&shot_from.0).unwrap()).unwrap();

            for (coords, radius) in liquid_molotovs.iter() {
                if collide_rect_circle(proj_coords.translation.truncate(), sprite.size, *coords, *radius) {
                    molotovs_to_be_lit_on_fire.push((*coords, *radius));

                }

            }

        }

    });

    // Finally, light any molotovs on fire that need to be lit
    projectile_movements.for_each_mut(|(entity, proj_coords, _, _, _, mut sprite, mut projectile_type, _, mut damage, mut material, _) |{
        if *projectile_type == ProjectileType::MolotovLiquid {

            molotovs_to_be_lit_on_fire.drain_filter(|potential_molotov| {
                let should_light_molotov = proj_coords.translation.truncate() == potential_molotov.0 && (sprite.size.x - potential_molotov.1).abs() < f32::EPSILON;

                if should_light_molotov {
                    // Does 75 damage every second (since there are 60 frames per second)
                    // This might seem excessive, but most players have the sense to run if they catch on fire, so the high damage done forces them to take the fire as a threat instead of just running through it to engage the slow and weak Inferno
                    // Once the molotov is hit by a bullet, it becomes molotov fire

                    *projectile_type.deref_mut() = ProjectileType::MolotovFire;
                    *material.deref_mut() = materials.molotov_fire.clone();
                    damage.deref_mut().0 = 75.0 / 60.0;
                    sprite.deref_mut().size = Vec2::new(250.0, 250.0);
                    commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(5.0, false)));

                }

                // Remove any lit molotovs w. drain filter
                should_light_molotov

            });

        }

    });
}
