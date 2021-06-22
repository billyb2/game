#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

use crate::*;

pub fn settings_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), With<Button>>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut keybindings: ResMut<KeyBindings>, mut selected_key_button: Query<&mut SelectedKeyButton>, mut keyboard_input: ResMut<Input<KeyCode>>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
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
    });
}

pub fn main_menu_system(button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
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
    });
}

pub fn game_menu_system(button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text.len() >= 5 && &text[5..] == "game" {
                    app_state.set(AppState::Connecting).unwrap();

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
    });
}

pub fn customize_menu_system(button_materials: Res<GameMenuButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, mut app_state: ResMut<State<AppState>>, mut my_ability: ResMut<Ability>, mut my_gun_model: ResMut<Model>) {
    interaction_query.for_each_mut(|(interaction, mut material, children)| {
        let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Clicked => {
                if text.len() >= 7 && &text[..7] == "Ability" {
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

                } else if text.len() >= 3 && &text[..3] == "Gun" {
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
    });
}

pub fn in_game_settings_menu_system(mut commands: Commands, settings_button_materials: Res<ButtonMaterials>, mut interaction_query: Query<(&Interaction, &mut Handle<ColorMaterial>, &Children), (Changed<Interaction>, With<Button>)>, mut text_query: Query<&mut Text>, in_game_settings: Query<(Entity, &InGameSettings)>, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>, mut my_ability: ResMut<Ability>, mut my_gun_model: ResMut<Model>, mut materials: ResMut<Assets<ColorMaterial>>, my_player_id: Res<MyPlayerID>, mut net: ResMut<NetworkResource>, mut players: Query<(Entity, &mut Ability, &mut AbilityCharge, &mut AbilityCompleted, &mut HelmetColor, &mut InnerSuitColor)>, player_entity: Res<HashMap<u8, Entity>>) {
    if !in_game_settings.is_empty() {
        interaction_query.for_each_mut(|(interaction, mut material, children)| {
            let text = &mut text_query.get_mut(children[0]).unwrap().sections[0].value;
            let menu = *in_game_settings.single().unwrap().1;

            match *interaction {
                Interaction::Clicked => {
                    let entity = in_game_settings.single().unwrap().0;

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
                                    material: materials.add(Color::rgba_u8(255, 255, 255, 10).into()),
                                    visible: Visible {
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

                                    node_parent.spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),
                                        align_content: AlignContent::Center,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,

                                        ..Default::default()
                                    },
                                    material: button_materials.normal.clone(),
                                    ..Default::default()
                                    })
                                    .with_children(|button_parent| {
                                        button_parent
                                            .spawn_bundle(TextBundle {
                                                text: Text {
                                                    sections: vec![
                                                        TextSection {
                                                            value: format!("Ability: {:?}", *my_ability),
                                                            style: TextStyle {
                                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                                font_size: 55.0,
                                                                color: Color::WHITE,
                                                            },
                                                        },
                                                    ],
                                                    ..Default::default()
                                                },
                                                ..Default::default()

                                        });
                                    });

                                    node_parent.spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(450.0), Val::Px(85.0)),
                                        align_content: AlignContent::Center,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,

                                        ..Default::default()
                                    },
                                    material: button_materials.normal.clone(),
                                    ..Default::default()
                                    })
                                    .with_children(|button_parent| {
                                        button_parent
                                            .spawn_bundle(TextBundle {
                                                text: Text {
                                                    sections: vec![
                                                        TextSection {
                                                            value: format!("Gun: {:?}", *my_gun_model),
                                                            style: TextStyle {
                                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                                font_size: 55.0,
                                                                color: Color::WHITE,
                                                            },
                                                        },
                                                    ],
                                                    ..Default::default()
                                                },
                                                ..Default::default()

                                        })
                                        .insert(KeyBindingButtons::Down);
                                    });

                                    node_parent.spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(225.0), Val::Px(85.0)),
                                        align_content: AlignContent::Center,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,

                                        ..Default::default()
                                    },
                                    material: button_materials.normal.clone(),
                                    ..Default::default()
                                    })
                                    .with_children(|button_parent| {
                                        button_parent
                                            .spawn_bundle(TextBundle {
                                                text: Text {
                                                    sections: vec![
                                                        TextSection {
                                                            value: String::from("Back"),
                                                            style: TextStyle {
                                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                                font_size: 55.0,
                                                                color: Color::WHITE,
                                                            },
                                                        },
                                                    ],
                                                    ..Default::default()
                                                },
                                                ..Default::default()

                                        });
                                    });

                                })
                                .insert(InGameSettings::Customize);

                        }

                    } else if menu == InGameSettings::Customize {
                        if text.len() >= 7 && &text[..7] == "Ability" {
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

                        } else if text.len() >= 3 && &text[..3] == "Gun" {
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

                        } else if text == "Back" {
                            commands.entity(entity).despawn_recursive();


                            let set_ability_message: [u8; 3] = [1, (*my_ability).into(), my_player_id.0.as_ref().unwrap().0];
                            net.broadcast_message(set_ability_message);

                            let my_ability = *my_ability.deref();
                            let my_player_id = my_player_id.deref().0.as_ref();

                            let (entity, mut ability, mut ability_charge, mut ability_completed, mut helmet_color, mut inner_suit_color) = players.get_mut(*player_entity.get(&my_player_id.unwrap().0).unwrap()).unwrap();

                            *ability.deref_mut() = my_ability;

                            let (new_helmet_color, new_inner_suit_color) = set_player_colors(&my_ability);

                            *helmet_color.deref_mut() = new_helmet_color;
                            *inner_suit_color.deref_mut() = new_inner_suit_color;

                            set_ability_player_attr(ability_charge.deref_mut(), ability_completed.deref_mut(), *ability.deref());

                            commands.entity(entity).insert_bundle(Gun::new(*my_gun_model.deref(), *ability));

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
                                material: materials.add(Color::rgba_u8(255, 255, 255, 10).into()),
                                visible: Visible {
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

                                node_parent.spawn_bundle(ButtonBundle {
                                style: Style {
                                    align_content: AlignContent::Center,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    size: Size::new(Val::Px(225.0), Val::Px(85.0)),

                                    ..Default::default()
                                },
                                material: button_materials.normal.clone(),
                                ..Default::default()
                                })
                                .with_children(|button_parent| {
                                    button_parent
                                        .spawn_bundle(TextBundle {
                                            text: Text {
                                                sections: vec![
                                                    TextSection {
                                                        value: String::from("Customize"),
                                                        style: TextStyle {
                                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                            font_size: 55.0,
                                                            color: Color::WHITE,
                                                        },
                                                    },
                                                ],
                                                ..Default::default()
                                            },
                                            ..Default::default()

                                    });
                                });

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
