use std::intrinsics::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

use bevy::prelude::*;
use bevy::ecs::component::Component;
 
use single_byte_hashmap::HashMap;

use game_lib::{GameLog, Logs};
use game_types::*;
use helper_functions::{u128_to_f32_u8, f32_u8_to_u128, get_angle};

use map::MapHealth;

//TODO: Damage numbers
pub fn move_objects(mut commands: Commands, mut physics_pipeline: ResMut<PhysicsPipeline>, mut island_manager: ResMut<IslandManager>, mut broad_phase: ResMut<BroadPhase>, mut narrow_phase: ResMut<NarrowPhase>, mut joint_set: ResMut<JointSet>, mut ccd_solver: ResMut<CCDSolver>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, mut movable_objects: Query<(Entity, &RigidBodyHandleWrapper, &ColliderHandleWrapper, &mut Sprite, Option<&mut Health>, Option<&mut MapHealth>, Option<&ProjectileIdent>, Option<&PlayerID>, Option<&mut DamageSource>, Option<&mut ProjectileType>, Option<&mut PlayerSpeed>, &mut Handle<ColorMaterial>), Without<ExplodeTimer>>, mut deathmatch_score: ResMut<DeathmatchScore>, mut death_event: EventWriter<DeathEvent>, proj_materials: Res<ProjectileMaterials>, local_players: Res<LocalPlayers>, mut widow_maker_heals: ResMut<WidowMakerHeals>, asset_server: Res<AssetServer>) {
    movable_objects.iter_mut().for_each(|(entity, rigid_body_handle, collider_handle, mut sprite, mut health, mut map_health, shot_from, player_id, mut damage_source, mut projectile_type, mut p_speed, mut material)| {
        let mut should_remove_rigid_body = false;

        let rigid_body_handle = &rigid_body_handle.0;
        let collider_handle = &collider_handle.0;

        if let Some(rigid_body) = rigid_body_set.get_mut(*rigid_body_handle) {
            let contacts = narrow_phase.contacts_with(*collider_handle);

            contacts.for_each(|contact_pair| {
                // Finds the collider handle that isn't equal to the current collider handle, and then grabs a reference to the actual collider object
                let other_collider_handle = match contact_pair.collider1 != *collider_handle {
                    true => contact_pair.collider1,
                    false => contact_pair.collider2,

                };

                if let Some(other_collider) = collider_set.get(other_collider_handle) {
                    if other_collider.collision_groups() == InteractionGroups::none() {
                        return;
                        
                    }

                    let hit_player = other_collider.user_data == u128::MAX;
                    let hit_map_object = other_collider.user_data == 0;

                    // Contains a mutable reference to the player or wall's health, and whether or not it's a player or a wall being hit
                    let health: Option<(&mut f32, bool)> = if health.as_ref().is_some() {
                        Some((&mut health.as_mut().unwrap().0, false))

                    } else if map_health.is_some() {
                        match map_health.as_mut().unwrap().0.as_mut() {
                            Some(health) => Some((health, true)),
                            None => None,
                        }

                    } else {
                        None

                    };

                    // Deal damage to objects that can take damage
                    if let Some((health, hit_wall)) = health  {
                        if other_collider.user_data != 0  && other_collider.user_data != u128::MAX{
                            let (damage, (shot_from, projectile_type)) = u128_to_f32_u8(other_collider.user_data);
                            let projectile_type: ProjectileType = projectile_type.into(); 

                            let should_do_damage = match hit_wall {
                                // Walls should always take damage
                                true => true,
                                // Players should only take damage if they aren't hitting themselves
                                false => {
                                    let player_id = player_id.unwrap().0; 
                                    shot_from != player_id

                                },

                            };

                            if should_do_damage {
                                let new_health = *health - damage;
                                let died = new_health <= 0.0;

                                if damage >= 0.0 {
                                    let text_pos = {
                                        let mut pos = *other_collider.translation();
                                        pos = pos.component_mul(&Vector2::new(250.0, 250.0));

                                        Vec3::new(pos.x, pos.y, 1000.0)
                                    };

                                    // Spawn damage text
                                    commands.spawn_bundle(Text2dBundle {
                                        text: Text {
                                            sections: vec![
                                                TextSection {
                                                    value: format!("{:.0}", damage),
                                                    style: TextStyle {
                                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 11.0,
                                                        color: match died {
                                                            false => Color::WHITE,
                                                            true => Color::RED,
                
                                                        },
                                                    },
                                                },
                                            ],
                                            ..Default::default()
                                        },
                                        transform: Transform::from_translation(text_pos),
                                        ..Default::default()
                
                                    })
                                    .insert(DamageTextTimer(Timer::from_seconds(2.0, false)));


                                }         


                                let player_is_local = if !hit_wall {
                                    if projectile_type == ProjectileType::WidowMaker {
                                        if let Some(health_to_heal) = widow_maker_heals.0.get_mut(&shot_from) {
                                            *health_to_heal += damage * 1.5;

                                        } else {
                                            widow_maker_heals.0.insert(shot_from, damage * 1.5);

                                        }

                                    }

                                    local_players.0.contains(&player_id.unwrap().0)

                                } else {
                                    // If you hit a wall, the player probably isn't local lol
                                    false

                                };

                                // Only directly edit the local health of our player, other players send their health over the net
                                if *health > 0.0 && (player_is_local || hit_wall) {
                                    if !hit_wall {
                                        damage_source.as_mut().unwrap().0 = Some(shot_from);

                                        if projectile_type == ProjectileType::TractorBeam {
                                            const FORCE: Vector2<f32> = Vector2::new(250.0, 250.0);

                                            let other_translation = other_collider.translation();
                                            let rigid_body_pos = rigid_body.translation();

                                            let angle_to_move = get_angle(other_translation.x, other_translation.y, rigid_body_pos.x, rigid_body_pos.y);

                                            let force = FORCE.component_mul(&Vector2::new(angle_to_move.cos(), angle_to_move.sin()));

                                            rigid_body.apply_force(force, true);

                                        }

                                    }

                                    *health = match died {
                                        true => {
                                            if !hit_wall {
                                                death_event.send(DeathEvent(player_id.unwrap().0));
                                                // The player who shot the bullet has their score increased 
                                                *deathmatch_score.0.get_mut(&shot_from).unwrap() += 1;

                                            }

                                            0.0
                                        },
                                        false => {
                                            if !hit_wall {
                                                let speed = p_speed.as_mut().unwrap();
                                                // Slow down players for X amount of seconds
                                                if projectile_type == ProjectileType::PulseWave {
                                                    speed.0 =  unsafe { fmul_fast(speed.0, 0.25) };
                                                    commands.entity(entity).insert(SlowedDown(Timer::from_seconds(2.5, false)));


                                                } else if projectile_type == ProjectileType::MolotovLiquid && speed.0 >= DEFAULT_PLAYER_SPEED * 0.5{
                                                    speed.0 = unsafe { fmul_fast(speed.0, 0.65) };
                                                    commands.entity(entity).insert(SlowedDown(Timer::from_seconds(2.0, false)));

                                                }

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
                        // The sticky grenade type shouldn't either
                        && projectile_type_ref != ProjectileType::StickyGrenade
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
                            
                        } else if projectile_type_ref == ProjectileType::StickyGrenade && hit_map_object {
                            commands.entity(entity).insert(ExplodeTimer(Timer::from_seconds(3.0, false)));

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

pub fn sync_physics_pos(mut obj: Query<(&mut Transform, &RigidBodyHandleWrapper)>, rigid_body_set: Res<RigidBodySet>) {
    obj.for_each_mut(|(mut transform, rigid_body_handle)| {
        if let Some(rigid_body) = rigid_body_set.get(rigid_body_handle.0) {
            // Update the rigid body's sprite to the correct translation
            let rigid_body_translation = rigid_body.translation().component_mul(&Vector2::new(250.0, 250.0));
            transform.translation = Vec3::new(rigid_body_translation.x, rigid_body_translation.y, transform.translation.z);

        }

    });
}

pub fn heal_widowmaker_shots(mut widow_maker_heals: ResMut<WidowMakerHeals>, mut players: Query<(&PlayerID, &mut Health)>) {
    players.for_each_mut(|(player_id, mut health)| {
        if let Some(health_to_heal) = widow_maker_heals.0.remove(&player_id.0) {
            // The health can only go as high as 150.0
            let new_health = health.0 + health_to_heal;

            if new_health <= 150.0 {
                health.0 = new_health;

            }

        }

    });


}

pub fn destruction_timer(mut commands: Commands, q: Query<(Entity, &DestructionTimer, &RigidBodyHandleWrapper)>, mut rigid_body_set: ResMut<RigidBodySet>, mut island_manager: ResMut<IslandManager>, mut collider_set: ResMut<ColliderSet>, mut joint_set: ResMut<JointSet>) {
    q.for_each(|(e, d_timer, rigid_body_handle)| {
        if d_timer.0.finished() {
            rigid_body_set.remove(rigid_body_handle.0, &mut island_manager, &mut collider_set, &mut joint_set);
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

pub fn generic_log_system<L: Component + Logs, T: Component, const TEXT_SIZE: Option<f32>, const TEXT_TIME: f32, E: Component + LogEv>(mut logs: ResMut<L>, mut game_log: Query<&mut Text, With<T>>, asset_server: Res<AssetServer>, mut log_event: EventReader<E>) {
    log_event.iter().for_each(|log_text| {
        let mut log_string = log_text.inner().clone();
        log_string.push('\n');

        let game_log = GameLog::new(log_string, TEXT_SIZE, TEXT_TIME, &asset_server);

        match logs.is_full() {
            true => *logs.first_mut().unwrap() = game_log,
            false => logs.push_unchecked(game_log),

        };

    });


    logs.retain(|l| {
        let should_keep = !l.timer.finished();

        if should_keep {
            l.text.style.color.set_a(l.timer.percent_left());

        }

        should_keep

    });

    let mut game_log = game_log.single_mut();

    game_log.sections.clear();
    game_log.sections.extend(logs.iter().rev().map(|l| l.text.clone()));

}


//TODO: Change this to seperate queries using Without
pub fn update_game_ui(query: Query<(&AbilityCharge, &AmmoInMag, &MaxAmmo, &TimeSinceStartReload, &Health), With<Model>>, mut ammo_style: Query<&mut Style, With<AmmoText>>, mut ammo_text: Query<&mut Text, (With<AmmoText>, Without<AbilityChargeText>)>, mut ability_charge_text: Query<&mut Text, (With<AbilityChargeText>, Without<HealthText>)>, mut health_text: Query<&mut Text, (With<HealthText>, Without<AmmoText>)>,
    my_player_id: Res<MyPlayerID>, player_entity: Res<HashMap<u8, Entity>>) {
    if let Some(my_id) = &my_player_id.0 {
        let (ability_charge, player_ammo_count, player_max_ammo, reload_timer, player_health) = query.get(*player_entity.get(&my_id.0).unwrap()).unwrap();

        let ammo_in_mag = (*player_ammo_count).0;
        let max_ammo = (*player_max_ammo).0;

        let ability_charge_percent = ability_charge.0.percent() * 100.0;

        let reloading = reload_timer.reloading;
        let health = player_health.0;

        let mut ammo_text = ammo_text.single_mut();
        let mut ammo_pos = ammo_style.single_mut();

        if !reloading {
            ammo_text.sections[0].value = ammo_in_mag.to_string();
            ammo_text.sections[1].value = " / ".to_string();
            ammo_text.sections[2].value = max_ammo.to_string();

            ammo_pos.position.left = Val::Percent(90.0);

        } else {
            ammo_text.sections[0].value = "Reloading...".to_string();
            ammo_text.sections[1].value = "".to_string();
            ammo_text.sections[2].value = "".to_string();

            // Since the Reloading text is pretty big, I need to shift it left slightly
            ammo_pos.position.left = Val::Percent(83.0);

        }

        let mut ability_charge_text = ability_charge_text.single_mut();
        ability_charge_text.sections[0].value = format!("{:.0}%", ability_charge_percent);

        let ability_charge_percent = ability_charge_percent as u8;


        ability_charge_text.sections[0].style.color = match ability_charge_percent {
            0..=49 => Color::RED,
            50..=99 => Color::YELLOW,
            100.. => Color::GREEN,
        };

        let mut health_text = health_text.single_mut();
        health_text.sections[0].value = format!("Health: {:.0}%", health);

    }
}

pub fn damage_text_system(mut commands: Commands, mut texts: Query<(Entity, &mut Text, &DamageTextTimer)>) {
    texts.for_each_mut(|(entity, mut text, timer)| {
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();

        } else {
            let text = &mut text.sections[0];
            text.style.color.set_a(timer.0.percent_left());

        }

    });
}

pub fn despawn_destroyed_walls(mut commands: Commands, mut walls: Query<(Entity, &MapHealth, &RigidBodyHandleWrapper), Changed<MapHealth>>, mut rigid_body_set: ResMut<RigidBodySet>, mut island_manager: ResMut<IslandManager>, mut joint_set: ResMut<JointSet>, mut collider_set: ResMut<ColliderSet>,) {
    walls.for_each_mut(|(entity, health, rigid_body_handle)| {
        if let Some(health) = health.0.as_ref() {
            if *health <= 0.0 {
                rigid_body_set.remove(rigid_body_handle.0, &mut island_manager, &mut collider_set, &mut joint_set);
                commands.entity(entity).despawn_recursive();

            }
        }


    });
}

pub fn explode_grenades(mut commands: Commands, grenades: Query<(Entity, &ExplodeTimer, &RigidBodyHandleWrapper, &ProjectileIdent)>, mut non_grenade_objects: Query<(&RigidBodyHandleWrapper, Option<&mut Health>, Option<&mut MapHealth>, Option<&PlayerID>), Without<ExplodeTimer>>, mut rigid_body_set: ResMut<RigidBodySet>, mut death_event: EventWriter<DeathEvent>, mut deathmatch_score: ResMut<DeathmatchScore>, mut islands: ResMut<IslandManager>, mut collider_set: ResMut<ColliderSet>, mut joint_set: ResMut<JointSet>, asset_server: Res<AssetServer>) {
    let mut explosion_positions = Vec::new();

    grenades.for_each(|(entity, explode_timer, rigid_body_handle, shot_from)| {
        if explode_timer.0.finished() {
            let rigid_body = rigid_body_set.get(rigid_body_handle.0).unwrap();

            explosion_positions.push((*rigid_body.translation(), shot_from.0));

            commands.entity(entity).despawn_recursive();
            rigid_body_set.remove(rigid_body_handle.0, &mut islands, &mut collider_set, &mut joint_set);

        }

    });

    non_grenade_objects.for_each_mut(|(rigid_body_handle, mut health, mut map_health, player_id)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body_handle.0) {
            let rigid_body_pos = *rigid_body.translation();

            let mut health = if health.is_some() {
                    Some(&mut health.as_mut().unwrap().0)

                } else if map_health.is_some() {
                    Some(map_health.as_mut().unwrap().0.as_mut().unwrap()) 

                } else {
                    None

                };

            explosion_positions.iter().for_each(|(explosion_pos, shot_from)| {
                const MAX_FORCE: f32 = 3200.0;
                // Divided by 250 to adjust for physics coord stuff
                const EXPLOSION_RADIUS: f32 = 600.0 / 250.0;
                const MAX_DAMAGE: f32 = 120.0;


                let explosion_angle = get_angle(rigid_body_pos.x, rigid_body_pos.y, explosion_pos.x, explosion_pos.y);

                let distance = rigid_body_pos.metric_distance(explosion_pos);

                let percent_of_explosion_radius = {
                    let distance_clamped = 
                        match distance > EXPLOSION_RADIUS {
                            true => EXPLOSION_RADIUS.copysign(distance),
                            false => distance,
                        };

                    Vector2::new(1.0, 1.0) - (Vector2::new(distance_clamped, distance_clamped).component_div(&Vector2::new(EXPLOSION_RADIUS, EXPLOSION_RADIUS)))

                };

                let force = {
                    let adj_force = Vector2::new(MAX_FORCE, MAX_FORCE).component_mul(&percent_of_explosion_radius);
                    adj_force.component_mul(&Vector2::new(explosion_angle.cos(), explosion_angle.sin()))

                };

                rigid_body.apply_force(force, true);

                let damage = percent_of_explosion_radius.amax().powi(3) * MAX_DAMAGE;

                // Health stuff
                if let Some(health) = health.as_mut() {
                    let new_health = **health - damage;
                    let died = new_health <= 0.0;

                    if **health > 0.0 {
                        **health = match died {
                            true => {
                                if let Some(player_id) = player_id {
                                    death_event.send(DeathEvent(player_id.0));
                                    *deathmatch_score.0.get_mut(&shot_from).unwrap() += 1;


                                }

                                0.0
                            },
                            false => new_health,

                        };

                        if damage > 0.0 {
                            let text_pos = {
                                let mut pos = *rigid_body.translation();
                                pos = pos.component_mul(&Vector2::new(250.0, 250.0));

                                Vec3::new(pos.x, pos.y, 1000.0)
                            };

                            // Spawn damage text
                            commands.spawn_bundle(Text2dBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection {
                                            value: format!("{:.0}", damage),
                                            style: TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 11.0,
                                                color: match died {
                                                    false => Color::WHITE,
                                                    true => Color::RED,

                                                },
                                            },
                                        },
                                    ],
                                    ..Default::default()
                                },
                                transform: Transform::from_translation(text_pos),
                                ..Default::default()

                            })
                            .insert(DamageTextTimer(Timer::from_seconds(2.0, false)));

     
                        }

                    }


                }

            });


        }

    });

}

pub fn increase_speed_and_size(mut projectiles: Query<(&ProjectileType, &RigidBodyHandleWrapper, &ColliderHandleWrapper, &mut Sprite)>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>) {
    projectiles.for_each_mut(|(proj_type, rigid_body_handle, collider_handle, mut sprite)| {
        // Increase the size of speedballs and flames
        // Only speedballs have a negative linear damping, meaning they increase in speed over time
        if *proj_type == ProjectileType::Speedball {
            let rigid_body = rigid_body_set.get_mut(rigid_body_handle.0).unwrap();
            let collider = collider_set.get_mut(collider_handle.0).unwrap();

            let mut linvel = rigid_body.linvel().abs().amax() * 25.0;
            // The maximum speed of Speedball projectiles is 65, so that they aren't horribly difficult to doge
            linvel = match linvel > 65.0 {
                true => 65.0,
                false => linvel,
            };

            sprite.size = Vec2::splat(linvel);

            collider.set_shape(SharedShape::cuboid(linvel / 500.0, linvel / 500.0));

            let (_damage, proj_info) = u128_to_f32_u8(collider.user_data);                
            // Speedballs do more damage as their velocity increases
            let new_damage = linvel * 1.5;
            collider.user_data = f32_u8_to_u128(new_damage, proj_info);

        } else if *proj_type == ProjectileType::Flame {
            if sprite.size.x <= 60.0 {
                sprite.size *= 1.4;

            }

        }


    });

}


pub fn proj_distance(mut commands: Commands, mut query: Query<(Entity, &mut ProjectileType, &MaxDistance, &mut DistanceTraveled, &mut Sprite, &mut Handle<ColorMaterial>, &Speed, &RigidBodyHandleWrapper, &ColliderHandleWrapper), Without<ExplodeTimer>>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, mut island_manager: ResMut<IslandManager>, mut joint_set: ResMut<JointSet>, proj_materials: Res<ProjectileMaterials>) {
    query.for_each_mut(|(entity, mut projectile_type, max_distance, mut distance_traveled, mut sprite, mut material, speed, rigid_body_handle, collider_handle)| {
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body_handle.0) {
            let speed = speed.0;

            distance_traveled.0 += speed;

            if distance_traveled.0 >= max_distance.0 {
                if !rigid_body.is_static() && *projectile_type != ProjectileType::Molotov && *projectile_type != ProjectileType::StickyGrenade {

                    rigid_body_set.remove(rigid_body_handle.0, &mut island_manager, &mut collider_set, &mut joint_set);
                    commands.entity(entity).despawn_recursive();

                } else if *projectile_type == ProjectileType::Molotov || *projectile_type == ProjectileType::StickyGrenade {
                    rigid_body.set_linvel(Vector2::new(0.0, 0.0), false);

                    if *projectile_type == ProjectileType::Molotov {
                        let collider = collider_set.get_mut(collider_handle.0).unwrap();

                        *material = proj_materials.molotov_liquid.clone();
                        *projectile_type = ProjectileType::MolotovLiquid;
                        sprite.size = Vec2::splat(200.0);
                        collider.set_shape(SharedShape::ball(200.0 / 500.0));
                        
                        rigid_body.set_body_type(RigidBodyType::Static);
                        collider.set_collision_groups(InteractionGroups::new(0b0010, 0b0100));

                        commands.entity(entity).insert(DestructionTimer(Timer::from_seconds(45.0, false)));

                        let (_damage, (shot_from, _proj_type)) = u128_to_f32_u8(rigid_body.user_data);

                        rigid_body.user_data = f32_u8_to_u128(0.0, (shot_from, ProjectileType::MolotovLiquid.into()));
                        collider.user_data = f32_u8_to_u128(0.0, (shot_from, ProjectileType::MolotovLiquid.into()));


                    } else {
                        commands.entity(entity).insert(ExplodeTimer(Timer::from_seconds(3.0, false)));
                        rigid_body.apply_force(vector![0.0, 1.0], true);

                    }

                }
            }


        }

    });
}
