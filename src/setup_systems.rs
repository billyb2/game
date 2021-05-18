#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

// This file is for storing all systems that are used as setups, such as setting up cameras, drawing the map, etc
use std::collections::BTreeSet;

use bevy::prelude::*;
use rand::Rng;

use crate::*;

 pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);

}

pub fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    //TODO: Use a spritesheet
    // The gorgeous assets are made by Shelby
    let hacker_sprite= asset_server.load("player_sprites/hacker.png");
    let wall_sprite= asset_server.load("player_sprites/wall.png");
    let stim_sprite= asset_server.load("player_sprites/stim.png");
    let engineer_sprite = asset_server.load("player_sprites/engineer.png");
    let phase_sprite = asset_server.load("player_sprites/phase.png");

    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(Skins {
        phase: materials.add(phase_sprite.into()),
        engineer: materials.add(engineer_sprite.into()),
        stim: materials.add(stim_sprite.into()),
        wall: materials.add(wall_sprite.into()),
        hacker: materials.add(hacker_sprite.into()),

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

    // Text saying the player's health
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(80.0),
                    top: Val::Percent(12.5),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Health: 0%".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 45.0,
                            color: Color::GREEN,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HealthText);

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

    let mut rng = rand::thread_rng();

    let mut availabie_player_ids: Vec<PlayerID> = Vec::with_capacity(256);
    let mut online_player_ids: BTreeSet<u8> = BTreeSet::new();
    online_player_ids.insert(0);

    for object in map.objects.iter() {
        if object.player_spawn {
            let ability: Ability = rng.gen();
            let gun_model: Model = rng.gen();

            commands
                .spawn_bundle(Player::new(i, ability))
                .insert_bundle(Gun::new(gun_model, ability))
                .insert_bundle(SpriteBundle {
                    material: match ability {
                        Ability::Phase => materials.phase.clone(),
                        Ability::Engineer => materials.engineer.clone(),
                        Ability::Stim => materials.stim.clone(),
                        Ability::Wall => materials.wall.clone(),
                        Ability::Hacker => materials.hacker.clone(),

                    },
                    sprite: Sprite {
                        size: Vec2::new(60.0, 60.0),
                        flip_x: true,
                        resize_mode: SpriteResizeMode::Manual,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(object.coords),
                    ..Default::default()
                });

            if i != 0 {
                availabie_player_ids.push(PlayerID(i));

            }

            i += 1;

        }
    }

    commands.insert_resource(availabie_player_ids);
    commands.insert_resource(OnlinePlayerIDs(online_player_ids));
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<ButtonMaterials>) {
    commands.insert_resource(ClearColor(Color::BLACK));

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
                            value: "Necrophaser".to_string(),
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

            // Only PC's can host games
            #[cfg(feature = "native")]
            node_parent.spawn_bundle(ButtonBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: Rect {
                    bottom: Val::Percent(10.0),

                    ..Default::default()
                },
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
                                    value: String::from("Play (Host)"),
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

            // Only WASM can join games
            #[cfg(feature = "web")]
            node_parent.spawn_bundle(ButtonBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: Rect {
                    bottom: Val::Percent(10.0),

                    ..Default::default()
                },
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
                                    value: String::from("Play (Join)"),
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
                                    value: String::from("Settings"),
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

        });



}

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

pub fn setup_connection_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::BLACK));

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
                            value: "Connecting, please wait...".to_string(),
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

        });
}

pub fn setup_default_controls(mut commands: Commands) {
    commands.insert_resource(KeyBindings {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,

        use_ability: KeyCode::Q,
        reload: KeyCode::R,

    });
}

pub fn setup_id(mut commands: Commands, hosting: Res<Hosting>) {
    #[cfg(feature = "native")]
    if hosting.0 {
        commands.insert_resource(MyPlayerID(Some(PlayerID(0))));

    }

    #[cfg(feature = "web")]
    if !hosting.0 {
        commands.insert_resource(MyPlayerID(None));

    }

}
