#![allow(clippy::type_complexity)]

mod components;
mod system_labels;
mod map;
mod helper_functions;
mod menus;
mod player_input;
mod player_attributes;
mod setup_systems;

//use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::sprite::SpriteSettings;

use map::*;
use player_input::*;
use helper_functions::collide;

use components::*;
use menus::*;
use player_attributes::*;
use system_labels::*;
use setup_systems::*;

// The game will always run at 60 fps
//TODO: Make this a setting
//const TIME_STEP: f32 = 1.0 / 60.0;

pub struct GameCamera;

struct AmmoText;
struct AbilityChargeText;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Settings,

}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProjectileType {
    Regular,
    Speedball,

}

#[derive(Bundle, Debug, PartialEq)]
pub struct Projectile {
    pub distance_traveled: DistanceTraveled,
    pub requested_movement: RequestedMovement,
    pub movement_type: MovementType,
    pub projectile_type: ProjectileType,
    // A general purpose identifier for projectiles, to distinguish between guns and projectiles
    pub projectile: ProjectileIdent,
    pub projectile_size: Size,
    pub damage: Damage,

}


impl Projectile {
    pub fn new(requested_movement: RequestedMovement, projectile_type: ProjectileType, max_distance: f32, size: Size, player_id: u8, damage: Damage) -> Projectile {
        Projectile {
            distance_traveled: DistanceTraveled(0.0),
            requested_movement,
            movement_type: MovementType::StopAfterDistance(max_distance),
            projectile_type,
            projectile: ProjectileIdent(player_id),
            projectile_size: size,
            damage,

        }
    }
}

pub struct Skins {
    phase: Handle<ColorMaterial>,

}

pub struct ProjectileMaterials {
    pub regular: Handle<ColorMaterial>,
    pub speedball: Handle<ColorMaterial>,
    pub engineer: Handle<ColorMaterial>,

}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,

}


// The mouse's position in 2D world coordinates
pub struct MousePosition(Vec2);

#[derive(Debug)]
pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub use_ability: KeyCode,
    pub reload: KeyCode,

}

#[derive(Debug, PartialEq)]
pub enum KeyBindingButtons {
    Up,
    Down,
    Left,
    Right,
    UseAbility,
    Reload,
}

#[derive(Debug, PartialEq)]
pub struct SelectedKeyButton(Option<KeyBindingButtons>);

#[derive(Debug, PartialEq)]
pub enum GameMode {
    Deathmatch,

}

fn main() {
    let mut app = App::build();
        // Antialiasing
        app.insert_resource(Msaa { samples: 1 });

        // Since text looks like garbage in browsers without antialiasing, it's higher for WASM by default
        #[cfg(target_arch = "wasm32")]
        app.insert_resource(Msaa { samples: 8 });

        app.insert_resource( WindowDescriptor {
            title: String::from("Necrophaser"),
            vsync: true,
            ..Default::default()

        });

        // I want the screen size to be smaller on wasm
        #[cfg(target_arch = "wasm32")]
        app.insert_resource( WindowDescriptor {
            title: String::from("Necrophaser"),
            vsync: true,
            width: 1366.0 * 0.85,
            height: 768.0 * 0.85,
            ..Default::default()

        });
        // Sprite culling doesn't render sprites outside of the camera viewport when enabled
        // It's fairly buggy when rendering many many  sprites (thousands) at the same time, however
        // Frustum culling also doesn't work with more than 1 camera, so it needs to be disabled for split screen
        // Though it does give a performance boost, especially where there are many sprites to render
        // Currently it's disabled, since we use the UI camera and the game camera
        app.insert_resource(SpriteSettings { frustum_culling_enabled: false })

        //Start in the main menu
        .add_state(AppState::MainMenu)

        // Embed the map into the binary
        .insert_resource(Map::from_bin(include_bytes!("../tiled/map1.custom")))
        // Gotta initialize the mouse position with something, or else the game crashes
        .insert_resource(MousePosition(Vec2::new(0.0, 0.0)))
        .insert_resource(GameMode::Deathmatch)

        .add_plugins(DefaultPlugins)

        // Adds some possible events, like reloading and using your ability
        .add_event::<ReloadEvent>()
        .add_event::<AbilityEvent>();

        //The WebGL2 plugin is only added if we're compiling to WASM
        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app
        // All the materials of the game NEED to be added before everything else
        .add_startup_system(setup_materials.system())
        // The cameras also need to be added first as well
        .add_startup_system(setup_cameras.system())
        .add_startup_system(setup_default_controls.system())

        // Initialize InGame
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup_game_ui.system())
                .with_system(draw_map.system())
                .with_system(setup_players.system())
                // Set the mouse coordinates initially
                .with_system(set_mouse_coords.system())

        )

        // Run every tick when InGame
        .add_system_set(
            // Anything that needs to run at a set framerate goes here (so basically everything in game)
            SystemSet::on_update(AppState::InGame)

                //TODO: Figure out how to use with_run_criteria with SystemSet to set a manual frame rate
                // Run the game at TIME_STEP per seconds (currently 60)
                // Currently disabled since states mess with with_run_criteria
                //.with_run_criteria(FixedTimestep::step(TIME_STEP as f64))

                // Timers should be ticked first
                .with_system(timer_system.system().before("player_attr").before(InputFromPlayer))
                .with_system(player_1_keyboard_input.system().label(InputFromPlayer).before("player_attr"))
                .with_system(shoot.system().label(InputFromPlayer))
                .with_system(set_mouse_coords.system().label(InputFromPlayer))
                .with_system(reset_player_resources.system().label(InputFromPlayer).label("player_attr"))
                .with_system(start_reload.system().label(InputFromPlayer).label("player_attr"))
                .with_system(use_ability.system().label(InputFromPlayer).label("player_attr"))
                .with_system(move_objects.system().after(InputFromPlayer).label("move_objects"))
                .with_system(dead_players.system().after("move_objects"))
                .with_system(move_camera.system().after(InputFromPlayer))
                .with_system(update_game_ui.system().after(InputFromPlayer))
        )

        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_main_menu.system())

        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(main_menu_system.system())

        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(exit_menu.system())

        )
        .add_system_set(
            SystemSet::on_enter(AppState::Settings)
                .with_system(setup_settings.system())

        )

        .add_system_set(
            SystemSet::on_update(AppState::Settings)
                .with_system(settings_system.system())

        )


        .add_system_set(
            SystemSet::on_exit(AppState::Settings)
                .with_system(exit_menu.system())
                .with_system(remove_selected.system())

        )

        .run();
}

//TODO: Turn RequestedMovement into an event
// Move objects will first validate whether a movement can be done, and if so move them
fn move_objects(mut commands: Commands, mut player_movements: Query<(&mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &Sprite, &PlayerID, &mut Health), Without<ProjectileIdent>>, mut projectile_movements: Query<(Entity, &mut Transform, &mut RequestedMovement, &MovementType, &mut DistanceTraveled, &mut Sprite, &ProjectileType, &ProjectileIdent, &mut Damage), (Without<PlayerID>, With<ProjectileIdent>)>,mut map: ResMut<Map>) {
    for (mut object, mut movement, movement_type, mut distance_traveled, sprite, _player_id, health) in player_movements.iter_mut() {
        if movement.speed != 0.0 && *health != Health(0){
            // Only lets you move if the movement doesn't bump into a wall
            let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);

            if !map.collision(object.translation + next_potential_movement, sprite.size, 0) {
                object.translation.x += movement.speed * movement.angle.cos();
                object.translation.y += movement.speed * movement.angle.sin();

                // Gotta make sure that it's both a projectile and has a projectile type, since guns also have a projectile type
                // If you don't do the is_projectile bit, you get a great bug where a player's size will increase as it moves (if they're using the speedball weapon)
                /*if let Some(projectile_type) = projectile_type {
                    // The speedball's weapon speeds up and gets bigger
                    if *projectile_type == ProjectileType::Speedball && is_projectile.is_some() {
                        movement.speed *= 1.1;
                        sprite.size *= 1.03;

                    }
                }*/

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        // If an object uses the StopAfterDistance movement type, it MUST have the distance traveled component, or it will crash
                        // Need to get the absolute value of the movement speed, since speed can be negative (backwards)
                        distance_traveled.0 += movement.speed.abs();

                        if distance_traveled.0 >= *distance_to_stop_at {
                            movement.speed = 0.0;

                        }
                    },
                }

            } else {
                movement.speed = 0.0;

            }
        }
    }

    for (_, mut object, mut movement, movement_type, mut distance_traveled, mut sprite, projectile_type, shot_from, mut damage) in projectile_movements.iter_mut() {
        if movement.speed != 0.0 {
            // Only lets you move if the movement doesn't bump into a wall
            let next_potential_movement = Vec3::new(movement.speed * movement.angle.cos(), movement.speed * movement.angle.sin(), 0.0);

            let mut player_collision = false;

            // Check to see if a player-projectile collision takes place
            for (player, _, _, _, player_sprite, player_id, mut health) in player_movements.iter_mut() {
                // Player bullets cannot collide with the player who shot them (thanks @Susorodni for the idea)
                // Checks that players aren't already dead as well lol
                if collide(player.translation, player_sprite.size, object.translation + next_potential_movement, sprite.size) && player_id.0 != shot_from.0 && *health != Health(0) {
                    if (health.0 as i8 - damage.0 as i8) < 0 {
                        health.0 = 0;

                    } else {
                        health.0 -= damage.0;

                    }

                    player_collision = true;
                    break;

                }

            }

            if !map.collision(object.translation + next_potential_movement, sprite.size, damage.0) && !player_collision {
                object.translation.x += movement.speed * movement.angle.cos();
                object.translation.y += movement.speed * movement.angle.sin();

                // Gotta make sure that it's both a projectile and has a projectile type, since guns also have a projectile type
                // If you don't do the is_projectile bit, you get a great bug where a player's size will increase as it moves (if they're using the speedball weapon)
                // The speedball's weapon speeds up and gets bigger
                if *projectile_type == ProjectileType::Speedball {
                    movement.speed *= 1.1;
                    sprite.size *= 1.03;

                    if damage.0 <= 75 {
                        damage.0 += (distance_traveled.0 / 60.0 ) as u8;

                    }

                }

                match movement_type {
                    // The object moves one frame, and then stops
                    MovementType::SingleFrame => {
                        movement.speed = 0.0;

                    },

                    MovementType::StopAfterDistance(distance_to_stop_at) => {
                        // If an object uses the StopAfterDistance movement type, it MUST have the distance traveled component, or it will crash
                        // Need to get the absolute value of the movement speed, since speed can be negative (backwards)
                        distance_traveled.0 += movement.speed.abs();

                        if distance_traveled.0 >= *distance_to_stop_at {
                            movement.speed = 0.0;

                        }
                    },
                }

            } else {
                movement.speed = 0.0;

            }
        }
    }

    // Remove all stopped bullets
    for object in projectile_movements.iter_mut() {
        if object.2.speed == 0.0 {
            commands.entity(object.0).despawn_recursive();

        }
    }

}

// This system just deals with stuff like making dead players invisible, respawning players, etc
fn dead_players(mut players: Query<(&mut Health, &mut Visible, &mut RespawnTimer), With<PlayerID>>, game_mode: Res<GameMode>) {
    for (mut health, mut visibility, mut respawn_timer) in players.iter_mut() {
        if health.0 == 0 {
            visibility.is_visible = false;

        }

        if respawn_timer.0.finished() && *game_mode == GameMode::Deathmatch {
            health.0 = 100;
            respawn_timer.0.reset();
            visibility.is_visible = true;

        }

    }

}

/// This system ticks all the `Timer` components on entities within the scene
/// using bevy's `Time` resource to get the delta between each update.
// Also adds ability charge to each player
fn timer_system(time: Res<Time>, mut timers: Query<(&mut AbilityCharge, &mut AbilityCompleted, &UsingAbility, &Health, &mut TimeSinceLastShot, &mut TimeSinceStartReload, &mut RespawnTimer)>, game_mode: Res<GameMode>) {
    for (mut ability_charge, mut ability_completed, using_ability, health, mut time_since_last_shot, mut time_since_start_reload, mut respawn_timer) in timers.iter_mut() {
        time_since_last_shot.0.tick(time.delta());
        ability_charge.0.tick(time.delta());

        // If the player is reloading
        if time_since_start_reload.reloading {
            time_since_start_reload.timer.tick(time.delta());

        }

        if using_ability.0 {
            ability_completed.0.tick(time.delta());

        }

        if *health == Health(0) && *game_mode == GameMode::Deathmatch {
            respawn_timer.0.tick(time.delta());

        }
    }
}

fn update_game_ui(query: Query<(&AbilityCharge, &AmmoInMag, &MaxAmmo, &PlayerID, &TimeSinceStartReload), With<Model>>, mut ammo_style: Query<&mut Style, With<AmmoText>>,
    mut t: QuerySet<(
        Query<&mut Text, With<AmmoText>>,
        Query<&mut Text, With<AbilityChargeText>>
    )>
) {
    let mut ammo_in_mag = 0;
    let mut max_ammo = 0;

    let mut ability_charge_percent = 0.0;

    let mut reloading = false;

    for (ability_charge, player_ammo_count, player_max_ammo, id, reload_timer) in query.iter() {
        if *id == PlayerID(0) {
            ammo_in_mag = (*player_ammo_count).0;
            max_ammo = (*player_max_ammo).0;

            ability_charge_percent = ability_charge.0.percent() * 100.0;

            reloading = reload_timer.reloading;

            break;

        }
    }

    let mut ammo_text = t.q0_mut().single_mut().unwrap();
    let mut ammo_pos = ammo_style.single_mut().unwrap();

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

    let mut ability_charge_text = t.q1_mut().single_mut().unwrap();
    ability_charge_text.sections[0].value = format!("{:.0}%", ability_charge_percent);

    let ability_charge_percent = ability_charge_percent as u8;

    if ability_charge_percent < 50 {
        ability_charge_text.sections[0].style.color = Color::RED;

    } else if (50..100).contains(&ability_charge_percent) {
        ability_charge_text.sections[0].style.color = Color::YELLOW;

    } else if ability_charge_percent == 100 {
        ability_charge_text.sections[0].style.color = Color::GREEN;

    }

}
