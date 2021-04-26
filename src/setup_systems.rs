// This file is for storing all systems that are used as setups, such as setting up cameras, drawing the map, etc
use bevy::prelude::*;

use crate::*;

 pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);

}

pub fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Skins {
        phase: materials.add(Color::rgb_u8(100, 242, 84).into()),

    });

    commands.insert_resource(ProjectileMaterials {
        regular: materials.add(Color::rgb_u8(255, 255, 255).into()),
        speedball: materials.add(Color::rgb_u8(126, 192, 238).into()),
        engineer: materials.add(Color::rgb_u8(255, 0, 200).into()),

    });

    commands.insert_resource(ButtonMaterials {
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),

    });
}

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Setup the UI
    // The text saying the player's ammo count
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(90.0),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "16".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "/".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "16".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AmmoText);

    // Text saying the player's ability charge
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(92.0),
                    top: Val::Percent(6.0),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "0%".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::RED,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AbilityChargeText);

    // Text saying the game log charge
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(75.0),
                    bottom: Val::Percent(6.0),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: Vec::with_capacity(10),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameLogText);

}

pub fn setup_players(mut commands: Commands, materials: Res<Skins>, map: Res<Map>) {
    let mut i: u8 = 0;

    for object in map.objects.iter() {
        if object.player_spawn {
            commands
                .spawn_bundle(Player::new(i, Ability::Engineer))
                .insert_bundle(Gun::new(Model::BurstRifle, Ability::Engineer))
                .insert_bundle(SpriteBundle {
                    material: materials.phase.clone(),
                    sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                    transform: Transform::from_translation(object.coords),
                    ..Default::default()
                });

                i += 1;

        }
    }
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<ButtonMaterials>) {
    commands.insert_resource(ClearColor(Color::BLACK));

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Relative,
                position: Rect {
                    left: Val::Percent(40.0),
                    top: Val::Percent(0.0),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Necrophaser".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 75.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()

        });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                position: Rect {
                    left: Val::Percent(20.0),
                    bottom: Val::Percent(14.0),

                    ..Default::default()
                },
                align_self: AlignSelf::Center,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
        parent
            .spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Play".to_string(),
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

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                position: Rect {
                    left: Val::Percent(8.15),
                    bottom: Val::Percent(7.0),

                    ..Default::default()
                },
                align_self: AlignSelf::Center,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
        parent
            .spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Settings".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 45.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()

            });

        });



}

//TODO: Replace all UI stuff with somethings similar to setup_settings, since it's a good example of how to use a proper flexbox, like using NodeBundles and stuff
pub fn setup_settings(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<ButtonMaterials>, keybindings: Res<KeyBindings>) {
    commands.insert_resource(ClearColor(Color::BLACK));
    commands
        .spawn()
        .insert(SelectedKeyButton(None));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexEnd,
                margin: Rect {
                   left: Val::Auto,
                   right: Val::Auto,

                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,

                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
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
                                font_size: 75.0,
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
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),

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
                                    value: format!("Up: {:?}", keybindings.up),
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
                .insert(KeyBindingButtons::Up);
            });

            node_parent.spawn_bundle(ButtonBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),

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
                                    value: format!("Down: {:?}", keybindings.down),
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
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),

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
                                    value: format!("Left: {:?}", keybindings.left),
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
                .insert(KeyBindingButtons::Left);
            });

            node_parent.spawn_bundle(ButtonBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),

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
                                    value: format!("Right: {:?}", keybindings.right),
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
                .insert(KeyBindingButtons::Right);
            });

            node_parent.spawn_bundle(ButtonBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),

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
                                    value: format!("Ability: {:?}", keybindings.use_ability),
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
                .insert(KeyBindingButtons::UseAbility);
            });

            node_parent.spawn_bundle(ButtonBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),

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
                                    value: format!("Reload: {:?}", keybindings.reload),
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
                .insert(KeyBindingButtons::Reload);
            });

        });

}

pub fn setup_default_controls(mut commands: Commands) {
    commands.insert_resource(KeyBindings {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,

        use_ability: KeyCode::E,
        reload: KeyCode::R,

    });
}
