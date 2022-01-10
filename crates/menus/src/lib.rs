#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(incomplete_features)]

#![feature(adt_const_params)]

use std::ops::Deref;
use std::net::{SocketAddr, IpAddr};

use bevy::prelude::*;
use bevy::math::Size;

use bevy::input::ElementState;
use bevy::input::keyboard::KeyboardInput;

//use helper_functions::color_to_image_handle;
use single_byte_hashmap::HashMap;

use config::{get_data, write_data};
use setup_systems::*;
use map::*;
use net::*;
use game_types::*;

use helper_functions::graphics::spawn_button;

pub fn settings_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), With<Button>>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut keybindings: ResMut<KeyBindings>, mut selected_key_button: Query<&mut SelectedKeyButton>, mut keyboard_input: ResMut<Input<KeyCode>>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        let mut selected_key_button = selected_key_button.single_mut();

        let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;

        if let Some(selected_key) = &selected_key_button.0 {
            if text.starts_with("Up")  && selected_key != &KeyBindingButtons::Up {
                *text = format!("Up: {:?}", keybindings.up);

            }

            if text.starts_with("Down") && selected_key != &KeyBindingButtons::Down {
                *text = format!("Down: {:?}", keybindings.down);

            }

            if text.starts_with("Left") && selected_key != &KeyBindingButtons::Left {
                *text = format!("Left: {:?}", keybindings.left);

            }

            if text.starts_with("Right") && selected_key != &KeyBindingButtons::Right {
                *text = format!("Right: {:?}", keybindings.right);

            }

            if text.starts_with("Ability") && selected_key != &KeyBindingButtons::UseAbility {
                *text = format!("Ability: {:?}", keybindings.use_ability);

            }

            if text.starts_with("Reload") && selected_key != &KeyBindingButtons::Reload {
                *text = format!("Reload: {:?}", keybindings.reload);

            }

            if text.starts_with("Melee") && selected_key != &KeyBindingButtons::Melee {
                *text = format!("Melee: {:?}", keybindings.melee);

            }

            if text.starts_with("Score") && selected_key != &KeyBindingButtons::ShowScore {
                *text = format!("Score: {:?}", keybindings.show_score);

            }

            if text.starts_with("Talk") && selected_key != &KeyBindingButtons::Talk {
                *text = format!("Talk: {:?}", keybindings.talk);

            }

        } else {
            if text.starts_with("Up") {
                *text = format!("Up: {:?}", keybindings.up);

            }

            if text.starts_with("Down") {
                *text = format!("Down: {:?}", keybindings.down);

            }

            if text.starts_with("Left") {
                *text = format!("Left: {:?}", keybindings.left);

            }

            if text.starts_with("Right") {
                *text = format!("Right: {:?}", keybindings.right);

            }

            if text.starts_with("Ability") {
                *text = format!("Ability: {:?}", keybindings.use_ability);

            }

            if text.starts_with("Reload") {
                *text = format!("Reload: {:?}", keybindings.reload);

            }

            if text.starts_with("Melee") {
                *text = format!("Melee: {:?}", keybindings.melee);

            }

            if text.starts_with("Score") {
                *text = format!("Score: {:?}", keybindings.show_score);

            }

            if text.starts_with("Talk") {
                *text = format!("Talk: {:?}", keybindings.talk);

            }
        }

        match *interaction {
            Interaction::Clicked => {
                if text == "Back" {
                    write_data(String::from("key_bindings"), &*keybindings);
                    app_state.set(AppState::MainMenu).unwrap();

                } else if text.starts_with("Up") {
                    *text = "Up:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Up);

                } else if text.starts_with("Down") {
                    *text = "Down:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Down);

                } else if text.starts_with("Left") {
                    *text = "Left:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Left);

                } else if text.starts_with("Right") {
                    *text = "Right:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Right);

                } else if text.starts_with("Ability") {
                    *text = "Ability:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::UseAbility);

                } else if text.starts_with("Reload") {
                    *text = "Reload:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Reload);

                } else if text.starts_with("Melee") {
                    *text = "Melee:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Melee);

                } else if text.starts_with("Score") {
                    *text = "Score:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::ShowScore);

                } else if text.starts_with("Talk") {
                    *text = "Talk:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Talk);

                }

            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();

            }
            Interaction::None => {
                *material = button_materials.normal.clone();

            }

        }


        if keyboard_input.just_pressed(KeyCode::Escape) {
            write_data(String::from("key_bindings"), &*keybindings);
            app_state.set(AppState::MainMenu).unwrap();
            keyboard_input.clear();

        } else if selected_key_button.0.is_some() {
            if let Some(key) = keyboard_input.get_just_pressed().last() {
                match selected_key_button.0.as_ref().unwrap() {
                    KeyBindingButtons::Up => {
                        keybindings.up = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Up") {
                            *text = format!("Up: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Down => {
                        keybindings.down = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Down") {
                            *text = format!("Down: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Left => {
                        keybindings.left = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Left") {
                            *text = format!("Left: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Right => {
                        keybindings.right = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Right") {
                            *text = format!("Right: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::UseAbility => {
                        keybindings.use_ability = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Ability") {
                            *text = format!("Ability: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Reload => {
                        keybindings.reload = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Reload") {
                            *text = format!("Reload: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Melee => {
                        keybindings.melee = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Melee") {
                            *text = format!("Melee: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::ShowScore => {
                        keybindings.show_score = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Score") {
                            *text = format!("Score: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Talk => {
                        keybindings.talk = *key;
                        selected_key_button.0 = None;

                        if text.starts_with("Talk") {
                            *text = format!("Talk: {:?}", *key);

                        }

                    }

                }
            }
        }
    });
}

pub fn main_menu_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                app_state.set(match text.as_str() {
                    "Play" => AppState::GameMenu,
                    "Settings" => AppState::Settings,
                    _ => unimplemented!(),

                }).unwrap();

            }
            Interaction::Hovered => {
                *material = button_materials.hovered;

            }
            Interaction::None => {
                *material = button_materials.normal;

            }
        }
    });
}

pub fn connection_menu(button_materials: Res<ButtonMaterials>, mut text_query: Query<(Entity, &mut Text), With<IpText>>, mut char_input_events: EventReader<ReceivedCharacter>, keyboard_input: Res<Input<KeyCode>>, mut interaction_query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>, mut net: ResMut<NetworkResource>, mut header_text: Query<&mut Text, Without<IpText>>, mut commands: Commands, addr: Option<Res<SocketAddr>>, mut app_state: ResMut<State<AppState>>, hosting: Res<Hosting>) {    
    if addr.is_none() && !hosting.0 {
        let (entity, mut text) = text_query.single_mut();
        let text = &mut text.sections[0].value;
        let header_text = &mut header_text.single_mut().sections[0].value;

        let mut connect_or_clear = 
        |text: &mut String, header_text: &mut String| {
            match text.parse::<IpAddr>() {
                Ok(addr) => {
                    let socket_addr = SocketAddr::new(addr, match cfg!(target_arch = "wasm32") {
                        true => 9363,
                        false => 9364,
                    });

                    commands.entity(entity).despawn_recursive();
                    header_text.str_write(format!("Connecting to {socket_addr}...").as_str());
    
                    commands.insert_resource(socket_addr);
                    
                    let connect_config = ConnectConfig {
                        addr: socket_addr,
                        udp_addr: Some(SocketAddr::new(addr, 9365)),

                    };

                    net.connect(connect_config, Some(2048));
                    
                },
                Err(err) => {
                    header_text.str_write(format!("Couldn't connect to {text} due to error: {:?}", err).as_str());
                    text.clear();
                },
            }
        };

        interaction_query.for_each_mut(|(interaction, mut material)| {
            match *interaction {
                Interaction::Clicked => connect_or_clear(text, header_text),
                Interaction::Hovered => {
                    *material = button_materials.hovered.clone();
                    header_text.str_write("Connect");
    
                }
                Interaction::None => {
                    *material = button_materials.normal.clone();
                    header_text.str_write("IP to connect to:");
    
                }
            }
    
        });
    
        if keyboard_input.just_pressed(KeyCode::Back) {
            text.pop();
    
        }
    
        if keyboard_input.just_pressed(KeyCode::Return) {
            connect_or_clear(text, header_text);
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::GameMenu).unwrap();
        }

        char_input_events.iter().for_each(|c| {
            if c.char.is_numeric() || c.char == '.' {
                text.push(c.char);
            }

        });

    } 

    #[cfg(debug_assertions)]
    if hosting.0 {
        panic!("Hosting?????");
    }
}

pub fn download_map_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut net: ResMut<NetworkResource>, map_crc32: Res<MapCRC32>, mut maps: ResMut<Maps>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        const DEFAULT_MAP_OBJECT: MapObject = MapObject::default();

        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text == "Cancel" {
                    app_state.set(AppState::GameMenu).unwrap();

                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();

            }
            Interaction::None => {
                *material = button_materials.normal.clone();

            }
        }

        //Checks to see if ther map's metadata is downloaded first
        // I check the map object capacity since if it's zero,, that's an effectively worthless map
        // Of course, this means maps with 0 map objects will download forever, so sad :(
        let map = maps.0.get_mut(&map_crc32.0).unwrap(); 

        if map.objects.capacity() == 0 {
            //net.broadcast_message(&(String::new(), 0_u64, [0.0_f32; 3], [0.0_f32; 2], map.crc32));

        } else {
/*            map.objects.iter_mut().enumerate().filter_map(|(i, object)| 
                match *object == DEFAULT_MAP_OBJECT {
                    true => {
                        let index: u64 = i.try_into().unwrap();
                        Some(index)

                    },
                    false => None,

            }).for_each(|i| net.broadcast_message((map_crc32.0, i)));
*/
            // Request a map object for each default map object
            
        }




    });
}

pub fn game_menu_system(button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut hosting: ResMut<Hosting>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text.ends_with("game") {
                    hosting.0 = text.starts_with("Host");
                    app_state.set(AppState::Connecting).unwrap();

                } else {
                    app_state.set(
                        match text.as_str() {
                            "Customize Player" => AppState::CustomizePlayerMenu,
                            "Customize Game" => AppState::CustomizeGame,
                            "Back" => AppState::MainMenu,
                            _ => unimplemented!(),
                        }
                    ).unwrap();

                }

            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();

            }
            Interaction::None => {
                *material = button_materials.normal.clone();

            }
        }
    });
}

pub fn customize_player_system(button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text, (Without<CustomizeHelpText>, Without<NameText>)>, mut app_state: ResMut<State<AppState>>, mut my_ability: ResMut<Ability>, mut my_gun_model: ResMut<Model>, mut my_perk: ResMut<Perk>, mut my_name: ResMut<PlayerName>, mut help_text: Query<&mut Text, (With<CustomizeHelpText>, Without<NameText>)>, mut typing: ResMut<Typing>, mut keyboard_input_events: EventReader<KeyboardInput>, mut name_text: Query<&mut Text, (With<NameText>, Without<CustomizeHelpText>)>) {
    if typing.0 {
        let text = &mut name_text.single_mut().sections[0].value;

        keyboard_input_events.iter().for_each(|ev| {
            if let Some(key_code) = ev.key_code {
                if ev.state == ElementState::Pressed {
                    // Whether or not the player has typed anything
                    let text_in_chat = text.len() > 0;

                    match key_code {
                        KeyCode::Return => {
                            if text_in_chat {
                                typing.0 = false;
                                my_name.0.clear();

                                let mut name_as_string = text.deref().clone();
                                name_as_string.truncate(24);

                                my_name.0.push_str(&name_as_string);

                            }

                        },
                        KeyCode::Back => {
                            if text_in_chat {
                                text.pop();

                            }

                        },
                        KeyCode::Escape => {
                            typing.0 = false;
                            *text = my_name.0.to_string();

                        },
                        _ => {
                            let key_text = format!("{:?}", ev.key_code.unwrap());

                            if key_text.len() == 1 {
                                text.push_str(&key_text);

                            } else if key_text.starts_with("Key") {
                                text.push(key_text.chars().nth(3).unwrap());

                            }

                        },
                    };
                }

            }

        });
    }

    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        if let Ok(text) = text_query.get_mut(children[0]).as_mut() {
            let text = &mut text.sections[0].value;

            if text.starts_with("Type") && !typing.0 {
                *text = String::from("Click to set name");

            }

        }

        match *interaction {
            Interaction::Clicked => {
                if let Ok(text) = text_query.get_mut(children[0]).as_mut() {
                    let text = &mut text.sections[0].value;

                    if text.starts_with("Ability") {
                        set_player_attr::<NUM_OF_ABILITIES, r#"Ability"#, Ability>(&mut my_ability, text);

                    } else if text.starts_with("Gun") {
                        set_player_attr::<NUM_OF_GUN_MODELS, r#"Gun"#, Model>(&mut my_gun_model, text);

                    } else if text.starts_with("Perk") {
                        set_player_attr::<NUM_OF_PERKS, r#"Perk"#, Perk>(&mut my_perk, text);

                    } else if text.starts_with("Click") {
                        typing.0 = !typing.0;

                        *text = String::from("Type name and press ENTER when completed...");

                    } else if text == "Back" {
                        if *my_ability == Ability::Brute {
                            *my_gun_model = Model::Melee;

                        }

                        typing.0 = false;

                        write_data(String::from("model"), *my_gun_model);
                        write_data(String::from("ability"), *my_ability);
                        write_data(String::from("perk"), *my_perk);

                        if my_name.0.len() > 0 {
                            write_data(String::from("name"), *my_name);

                        } else {
                            my_name.0 = get_data(String::from("name")).unwrap();

                        }

                        app_state.set(AppState::GameMenu).unwrap();

                    }

                }

            },
            Interaction::Hovered => {
                if let Ok(button_text) = text_query.get_mut(children[0]).as_mut() {
                    let button_text = &mut button_text.sections[0].value;

                    let help_text = &mut help_text.single_mut().sections[0].value;

                    *help_text = if button_text.starts_with("Ability") {
                        match *my_ability {
                            Ability::Warp => String::from("Your suit is equipped with a space-time warping device that allows you\n to teleport short distances"),
                            Ability::Stim => String::from("Your robot body allows you to run faster than normal, and can supercharge\n itself with a large battery, allowing you to temporarily increase your running speed"),
                            Ability::Engineer => String::from("Using your years of experience designing weapons, you've modified\n your guns to reload much faster and your bullets to move more quickly, at the cost of having higher recoil (PASSIVE)"),
                            Ability::Wall => String::from("You can generate walls of pure energy, that you can shoot through but\n your opponents cannot"),
                            Ability::Inferno => String::from("Your flame tipped bullets can light the molotovs you throw"),
                            Ability::Cloak => String::from("Your suit is modified to be able to temporarily be invisible to the eye"),
                            Ability::PulseWave => String::from("You can generate pulses of electricity, significantly slowing down your opponents temporarily"),
                            Ability::Ghost => String::from("Your nano tech armor and body allow you to, with effort, temporarily move through walls"),
                            Ability::Brute => String::from("You use your tractor beam and mechanical arms to\n beat your opponents into submission"),

                        }

                    } else if button_text.starts_with("Gun") {
                        match *my_gun_model {
                            Model::Shotgun => String::from("A close-mid range high spread shotgun"),
                            Model::ClusterShotgun => String::from("A high risk, high reward very close range shotgun"),
                            Model::BurstRifle => String::from("A relatively accurate burst damage assault rifle"),
                            Model::Speedball => String::from("Shoots projetiles with low damage and speed at first, but pick up speed and increases damage over time"),
                            Model::AssaultRifle => String::from("A high recoil high damage automatic rifle"),
                            Model::Pistol => String::from("A high damage, slow firing pistol"),
                            Model::SubmachineGun => String::from("Sprays down an area with a very high fire rate"),
                            Model::Flamethrower => String::from("Melts opponents with extremely high damage, but low range"),
                            Model::SniperRifle => String::from("Long range, extremely high-damage sniper with severely slow reload times"),
                            Model::Melee => String::from("Enhanced arms let you punch stronger"),
                            Model::Widowmaker => String::from("Utilizes your health as ammo. Health is returned for the amount of damage you do, and it does not reload"),
                            Model::Bow => String::from("Long range, short range, slow reload rate"),
                            Model::StickyGrenade => String::from("Fires grenades that stick to walls and opponents"),
                        }

                    } else if button_text.starts_with("Perk") {
                        match *my_perk {
                            Perk::ExtendedMag => String::from("Your guns can hold more rounds at a time"),
                            Perk::HeavyArmor => String::from("Your armor is stronger, in exchange for moving a little slower"),
                            Perk::LightArmor => String::from("Your armor is weaker, and in exchange you move a bit faster"),
                            Perk::ExtendedVision => String::from("Your view of the map is slightly larger, allowing you to see players that are farther away"),
                        }

                    } else {
                        String::new()

                    };

                }

                *material = button_materials.hovered.clone();

            }
            Interaction::None => {}
        }

    });

}

pub fn customize_game_system(button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut map_crc32: ResMut<MapCRC32>, maps: Res<Maps>, mut num_of_bots: ResMut<NumOfBots>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text == "Back" {
                    app_state.set(AppState::GameMenu).unwrap();

                } else if text.starts_with("Map") {
                    // Basically an incredibly genius way of iterating through the map
                    // It just tries to figure out what the next map item is after the current one, and switch the current map item to that one

                    // Obviously, no point in doing all the work if there's only 1 map
                    if maps.0.len() > 1 {
                        let crc32_iter = maps.0.keys();

                        // Firstly, if the current map is the last one in the hashmap, just loop around and set the current map to the first one
                        if crc32_iter.last().unwrap() == &map_crc32.0 {
                            map_crc32.0 = *maps.0.keys().next().unwrap();

                        } else {
                            // If not, then just try to find the map directly next to the currrent one
                            let mut crc32_iter = maps.0.keys();

                            if crc32_iter.any(|&crc32| map_crc32.0 == crc32) {
                                map_crc32.0 = *crc32_iter.next().unwrap();
                            }

                        }

                        *text = format!("Map: {:?}", maps.0.get(&map_crc32.0).unwrap().name); 

                    }

                } else if text.starts_with("Number of bots") {
                    let mut max_num_of_bots: u8 = maps.0.get(&map_crc32.0).unwrap().spawn_points.len().try_into().unwrap();
                    max_num_of_bots -= 1;

                    num_of_bots.0 = match num_of_bots.0 + 1 > max_num_of_bots {
                        true => 0,
                        false => num_of_bots.0 + 1
                    };

                    *text = format!("Number of bots: {}", num_of_bots.0);
                }

            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();

            }
            Interaction::None => {
                *material = button_materials.normal.clone();

            }
        }
    });
}

pub fn in_game_settings_menu_system(mut commands: Commands, settings_button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, in_game_settings: Query<(Entity, &InGameSettings)>, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>, mut my_ability: ResMut<Ability>, mut my_gun_model: ResMut<Model>, mut my_perk: ResMut<Perk>, mut materials: ResMut<Assets<ColorMaterial>>, my_player_id: Res<MyPlayerID>, mut net: ResMut<NetworkResource>, mut players: Query<(Entity, &mut AbilityInfo, &mut Model, &mut Perk)>, player_entity: Res<HashMap<u8, Entity>>) {
    if !in_game_settings.is_empty() {
        interaction_query.for_each_mut(|(interaction, mut material, children)| {
            let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;
            let menu = *in_game_settings.single().1;

            match *interaction {
                Interaction::Clicked => {
                    let entity = in_game_settings.single().0;

                    if menu == InGameSettings::Settings {
                        if text == "Customize" {
                            commands.entity(entity).despawn_recursive();

                            commands
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        align_self: AlignSelf::Center,
                                        position_type: PositionType::Absolute,
                                        justify_content: JustifyContent::Center,
                                        align_content: AlignContent::Center,
                                        align_items: AlignItems::Center,
                                        size: Size {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                        },

                                        ..Default::default()
                                    },
                                    color: UiColor(Color::rgba_u8(255, 255, 255, 10)),
                                    visibility: Visibility {
                                        is_visible: true,
                                        ..Default::default()
                                    },
                                    ..Default::default()

                                })
                                .with_children(|node_parent| {
                                    node_parent.spawn_bundle(TextBundle {
                                        text: Text {
                                            sections: vec![
                                                TextSection {
                                                    value: String::from("Customize"),
                                                    style: TextStyle {
                                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                        font_size: 80.0,
                                                        color: Color::WHITE,
                                                    },
                                                },
                                            ],
                                            ..Default::default()
                                        },
                                        ..Default::default()

                                    });

                                    spawn_button::<{ None }, 85.0>(node_parent, format!("Ability: {}", *my_ability), asset_server.load("fonts/FiraSans-Bold.ttf"), Default::default());

                                    spawn_button::<{ None }, 85.0>(node_parent, format!("Gun: {}", *my_gun_model), asset_server.load("fonts/FiraSans-Bold.ttf"), Default::default());

                                    spawn_button::<{ None }, 85.0>(node_parent, format!("Perk: {}", *my_perk), asset_server.load("fonts/FiraSans-Bold.ttf"), Default::default());

                                    spawn_button::<{ Some(225.0) }, 85.0>(node_parent, String::from("Back"), asset_server.load("fonts/FiraSans-Bold.ttf"), Default::default());


                                })
                                .insert(InGameSettings::Customize);

                        }

                    } else if menu == InGameSettings::Customize {
                        if text.starts_with("Ability") {
                            set_player_attr::<NUM_OF_ABILITIES, r#"Ability"#, Ability>(&mut my_ability, text);

                        } else if text.starts_with("Gun") {
                            set_player_attr::<NUM_OF_GUN_MODELS, r#"Gun"#, Model>(&mut my_gun_model, text);

                        } else if text.starts_with("Perk") {
                            set_player_attr::<NUM_OF_PERKS, r#"Perk"#, Perk>(&mut my_perk, text);


                        } else if text == "Back" {
                            commands.entity(entity).despawn_recursive();


                            let set_ability_message: [u8; 3] = [1, (*my_ability).into(), my_player_id.0.as_ref().unwrap().0];
                            net.broadcast_message(&set_ability_message, &INFO_MESSAGE_CHANNEL);

                            let my_player_id = my_player_id.0.as_ref();

                            let (entity, mut ability_info, mut model, mut perk) = players.get_mut(*player_entity.get(&my_player_id.unwrap().0).unwrap()).unwrap();

                            if *my_ability == Ability::Brute {
                                *my_gun_model = Model::Melee;

                            }


                            write_data(String::from("model"), *my_gun_model);
                            write_data(String::from("ability"), *my_ability);
                            write_data(String::from("perk"), *my_perk);

                            ability_info.ability = *my_ability;
                            *model = *my_gun_model;
                            *perk = *my_perk;

                            /*let (new_helmet_color, new_inner_suit_color) = set_player_colors(&my_ability);

                            *helmet_color = new_helmet_color;
                            *inner_suit_color = new_inner_suit_color;*/

                            set_ability_player_attr(&mut ability_info);

                            commands.entity(entity).insert_bundle(Gun::new(*my_gun_model, *my_ability, *my_perk));

                            commands
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::ColumnReverse,
                                    align_self: AlignSelf::Center,
                                    position_type: PositionType::Absolute,
                                    justify_content: JustifyContent::Center,
                                    align_content: AlignContent::Center,
                                    align_items: AlignItems::Center,
                                    size: Size {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                    },

                                    ..Default::default()
                                },
                                color: UiColor(Color::rgba_u8(255, 255, 255, 10)),
                                visibility: Visibility {
                                    is_visible: true,
                                    ..Default::default()
                                },
                                ..Default::default()

                            })
                            .with_children(|node_parent| {
                                node_parent.spawn_bundle(TextBundle {
                                    text: Text {
                                        sections: vec![
                                            TextSection {
                                                value: "Settings".to_string(),
                                                style: TextStyle {
                                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                    font_size: 80.0,
                                                    color: Color::GOLD,
                                                },
                                            },
                                        ],
                                        ..Default::default()
                                    },
                                    ..Default::default()

                                });

                                spawn_button::<{ None }, 85.0>(node_parent, String::from("Customize"), asset_server.load("fonts/FiraSans-Bold.ttf"), Default::default());

                            })
                            .insert(InGameSettings::Settings);
                        }
                    }

                }
                Interaction::Hovered => {
                    *material = match menu {
                        InGameSettings::Customize => button_materials.hovered.clone(),
                        InGameSettings::Settings => settings_button_materials.hovered.clone(),

                    };

                }
                Interaction::None => {
                    *material = match menu {
                        InGameSettings::Customize => button_materials.normal.clone(),
                        InGameSettings::Settings => settings_button_materials.normal.clone(),

                    };

                }
            }
        });
    }
}


// When exiting anything with a UI, it removes all Nodes, which removes all text and buttons, as well as there children
pub fn exit_menu(mut commands: Commands, query: Query<(Entity, &Node)>) {
    query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });
}

pub fn remove_selected(mut commands: Commands, query: Query<(Entity, &SelectedKeyButton)>) {
    query.for_each(|q| {
        commands.entity(q.0).despawn_recursive();

    });

}

fn set_player_attr<const NUM_OF_T: u8, const ATTR_NAME: &'static str, T>(my_player_attr: &mut T, text: &mut String) where T: From<u8> + std::fmt::Display + Copy, u8: From<T> {
    let current_t_int: u8 = (*my_player_attr).into();

    let new_player_attr: T = match current_t_int == NUM_OF_T - 1 {
        true => 0.into(),
        false => (current_t_int + 1).into(),

    };

    *my_player_attr = new_player_attr;
    *text = format!("{}: {}", ATTR_NAME, new_player_attr);
}
