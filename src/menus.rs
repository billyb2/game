#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::*;
use crate::net::Hosting;

pub fn settings_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), With<Button>>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut keybindings: ResMut<KeyBindings>, mut selected_key_button: Query<&mut SelectedKeyButton>, mut keyboard_input: ResMut<Input<KeyCode>>) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut selected_key_button = selected_key_button.single_mut().unwrap();

        let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;

        if let Some(selected_key) = &selected_key_button.0 {
            if text.len() >= 3 && text[0..=1] == *"Up"  && selected_key != &KeyBindingButtons::Up {
                *text = format!("Up: {:?}", keybindings.up);

            }

            if text.len() >= 5 && text[0..=3] == *"Down" && selected_key != &KeyBindingButtons::Down {
                *text = format!("Down: {:?}", keybindings.down);

            }

            if text.len() >= 5 && text[0..=3] == *"Left" && selected_key != &KeyBindingButtons::Left {
                *text = format!("Left: {:?}", keybindings.left);

            }

            if text.len() >= 6 && text[0..=4] == *"Right" && selected_key != &KeyBindingButtons::Right {
                *text = format!("Right: {:?}", keybindings.right);

            }

            if text.len() >= 8 && text[0..=6] == *"Ability" && selected_key != &KeyBindingButtons::UseAbility {
                *text = format!("Ability: {:?}", keybindings.use_ability);

            }

            if text.len() >= 7 && text[0..=5] == *"Reload" && selected_key != &KeyBindingButtons::Reload {
                *text = format!("Reload: {:?}", keybindings.reload);

            }

            if text.len() >= 6 && text[0..=4] == *"Score" && selected_key != &KeyBindingButtons::ShowScore {
                *text = format!("Score: {:?}", keybindings.show_score);

            }

        } else {
            if text.len() >= 3 && text[0..=1] == *"Up" {
                *text = format!("Up: {:?}", keybindings.up);

            }

            if text.len() >= 5 && text[0..=3] == *"Down" {
                *text = format!("Down: {:?}", keybindings.down);

            }

            if text.len() >= 5 && text[0..=3] == *"Left" {
                *text = format!("Left: {:?}", keybindings.left);

            }

            if text.len() >= 6 && text[0..=4] == *"Right" {
                *text = format!("Right: {:?}", keybindings.right);

            }

            if text.len() >= 8 && text[0..=6] == *"Ability" {
                *text = format!("Ability: {:?}", keybindings.use_ability);

            }

            if text.len() >= 7 && text[0..=5] == *"Reload" {
                *text = format!("Reload: {:?}", keybindings.reload);

            }

            if text.len() >= 6 && text[0..=4] == *"Score" {
                *text = format!("Score: {:?}", keybindings.show_score);

            }
        }

        match *interaction {
            Interaction::Clicked => {
                if text == "Back" {
                    app_state.set(AppState::MainMenu).unwrap();

                } else if text[0..=1] == *"Up" {
                    *text = "Up:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Up);

                } else if text[0..=3] == *"Down" {
                    *text = "Down:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Down);

                } else if text[0..=3] == *"Left" {
                    *text = "Left:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Left);

                } else if text[0..=4] == *"Right" {
                    *text = "Right:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Right);

                } else if text[0..=6] == *"Ability" {
                    *text = "Ability:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::UseAbility);

                } else if text[0..=5] == *"Reload" {
                    *text = "Reload:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::Reload);

                } else if text[0..=4] == *"Score" {
                    *text = "Score:".to_string();
                    selected_key_button.0 = Some(KeyBindingButtons::ShowScore);

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
            app_state.set(AppState::MainMenu).unwrap();
            keyboard_input.clear();

        } else if selected_key_button.0.is_some() {
            if let Some(key) = keyboard_input.get_just_pressed().last() {
                match selected_key_button.0.as_ref().unwrap() {
                    KeyBindingButtons::Up => {
                        keybindings.up = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 3 && text[0..=1] == *"Up" {
                            *text = format!("Up: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Down => {
                        keybindings.down = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 5 && text[0..=3] == *"Down" {
                            *text = format!("Down: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Left => {
                        keybindings.left = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 5 && text[0..=3] == *"Left" {
                            *text = format!("Left: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Right => {
                        keybindings.right = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 6 && text[0..=4] == *"Right" {
                            *text = format!("Right: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::UseAbility => {
                        keybindings.use_ability = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 8 && text[0..=6] == *"Ability" {
                            *text = format!("Ability: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::Reload => {
                        keybindings.reload = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 6 && text[0..=5] == *"Reload" {
                            *text = format!("Reload: {:?}", *key);

                        }

                    },
                    KeyBindingButtons::ShowScore => {
                        keybindings.show_score = *key;
                        selected_key_button.0 = None;

                        if text.len() >= 5 && text[0..=4] == *"Score" {
                            *text = format!("Score: {:?}", *key);

                        }

                    },

                }
            }
        }
    }
}

pub fn main_menu_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text == "Play" {
                    app_state.set(AppState::GameMenu).unwrap();

                } else if text == "Settings"{
                    app_state.set(AppState::Settings).unwrap();

                }

            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();

            }
            Interaction::None => {
                *material = button_materials.normal.clone();

            }
        }
    }
}

pub fn game_menu_system(mut commands: Commands, button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text.len() >= 5 && &text[5..] == "game" {
                    app_state.set(AppState::Connecting).unwrap();
                    #[cfg(feature = "native")]
                    commands.insert_resource(Hosting(true));
                    #[cfg(feature = "web")]
                    commands.insert_resource(Hosting(false));

                } else if text == "Customize" {
                    app_state.set(AppState::CustomizePlayerMenu).unwrap();

                } else if text == "Back" {
                    app_state.set(AppState::MainMenu).unwrap();

                }

            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();

            }
            Interaction::None => {
                *material = button_materials.normal.clone();

            }
        }
    }
}

pub fn customize_menu_system(button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut my_ability: ResMut<Ability>, mut my_gun_model: ResMut<Model>) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text.len() >= 7 && &text[..=6] == "Ability" {
                    let current_ability_int: u8 = (*my_ability).into();

                    match current_ability_int == NUM_OF_ABILITIES - 1 {
                        true => {
                            let new_ability: Ability = 0.into();
                            *my_ability.deref_mut() = new_ability;
                        },
                        false => {
                            let new_ability: Ability = (current_ability_int + 1).into();
                            *my_ability.deref_mut() = new_ability;
                        },


                    };
                    

                    *text = format!("Ability: {:?}", *my_ability);

                } else if text.len() >= 3 && &text[..=2] == "Gun" {
                    let current_gun_int: u8 = (*my_gun_model).into();

                    match current_gun_int == NUM_OF_GUN_MODELS - 1 {
                        true => {
                            let new_gun_model: Model = 0.into();
                            *my_gun_model.deref_mut() = new_gun_model;
                        },
                        false => {
                            let new_gun_model: Model = (current_gun_int + 1).into();
                            *my_gun_model.deref_mut() = new_gun_model;
                        },


                    };

                    *text = format!("Gun: {:?}", *my_gun_model);

                } else if text == "Customize" {
                    app_state.set(AppState::CustomizePlayerMenu).unwrap();

                } else if text == "Back" {
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
    }
}

// When exiting anything with a UI, it removes all Nodes, which removes all text and buttons, as well as there children
pub fn exit_menu(mut commands: Commands, mut query: Query<(Entity, &Node)>) {
    for q in query.iter_mut() {
        commands.entity(q.0).despawn_recursive();

    }
}

pub fn remove_selected(mut commands: Commands, mut query: Query<(Entity, &SelectedKeyButton)>) {
    for q in query.iter_mut() {
        commands.entity(q.0).despawn_recursive();

    }

}
