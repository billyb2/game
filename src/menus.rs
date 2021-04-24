use bevy::prelude::*;
use crate::*;

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
        }

        match *interaction {
            Interaction::Clicked => {
                if text[0..=1] == *"Up" {
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
                if text == &String::from("Play") {
                    app_state.set(AppState::InGame).unwrap();

                } else if text == &String::from("Settings") {
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

// When exiting anything with a UI, it removes all Nodes, which removes all text and buttons, as well as there children
pub fn exit_menu(mut commands: Commands, mut query: Query<(Entity, &Node)>) {
    for q in query.iter_mut() {
        commands.entity(q.0).despawn_recursive();

    }
}
