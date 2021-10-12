#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::prelude::Rect;
use bevy::prelude::*;

use crate::*;
use map::MapCRC32;
use single_byte_hashmap::*;

use bevy::render::camera::ScalingMode;
use rapier2d::prelude::*;

pub fn setup_cameras(mut commands: Commands, window: Res<WindowDescriptor>,) {
    commands.spawn_bundle(UiCameraBundle::default());

    let mut orthographic_camera = OrthographicCameraBundle::new_2d();
    orthographic_camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;
    // orthographic_camera.transform.local_z() = Vec3::new(x, y, 1.0);
    commands
        .spawn_bundle(orthographic_camera)
        .insert(GameCamera);

    #[cfg(feature = "native")]
    {
        let res_scale = (window.width / 1366.0).min(window.height / 768.0) * 0.95;
        commands.insert_resource(ResScale(res_scale.recip()));
        
    };
}

pub fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    //TODO: Use a spritesheet
    // The gorgeous assets are made by Shelby
    let default_sprite = asset_server.load("player_sprites/default.png");
    let enemy_sprite = asset_server.load("player_sprites/enemy.png");

    let molotov_fire_sprite = asset_server.load("projectile_sprites/molotov_fire.png");
    let molotov_liquid_sprite = asset_server.load("projectile_sprites/molotov_liquid.png");

    let pulsewave_sprite = asset_server.load("projectile_sprites/pulsewave.png");

    let rng = fastrand::Rng::new();

    let flame1 = rng.u8(200..=250);
    let flame2 = rng.u8(100..=150);
    let flame3 = rng.u8(100..=250);

    #[cfg(feature = "native")]
    let mut map_assets: HashMap<u8, Handle<ColorMaterial>> =
        HashMap::with_capacity_and_hasher(256, BuildHasher::default());

    #[cfg(feature = "web")]
    let map_assets: HashMap<u8, Handle<ColorMaterial>> =
        HashMap::with_capacity_and_hasher(256, BuildHasher::default());

    // Web builds can't preload assets
    #[cfg(feature = "native")]
    {
        let assets = asset_server.load_folder("map_assets/").unwrap();

        assets.iter().for_each(|asset| {
            let asset = asset.clone().typed();
            let asset_path = asset_server.get_handle_path(asset.clone()).unwrap();
            let path = asset_path.path();
            let file_name_string = path.file_stem().unwrap().to_str().unwrap();

            let int = file_name_string.parse::<u8>().unwrap();

            map_assets.insert(int, materials.add(asset.into()));
        });
    }

    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    commands.insert_resource(Skin {
        player: materials.add(default_sprite.into()),
        enemy: materials.add(enemy_sprite.into()),

    });

    commands.insert_resource(ProjectileMaterials {
        regular: materials.add(Color::BLACK.into()),
        speedball: materials.add(Color::rgb_u8(126, 192, 238).into()),
        flamethrower1: materials.add(Color::rgb_u8(flame1, 43, 9).into()),
        flamethrower2: materials.add(Color::rgb_u8(221, flame2, 9).into()),
        flamethrower3: materials.add(Color::rgb_u8(flame3, 43, 12).into()),
        engineer: materials.add(Color::rgb_u8(255, 0, 200).into()),
        molotov: materials.add(Color::rgb_u8(232, 35, 0).into()),
        molotov_fire: materials.add(molotov_fire_sprite.into()),
        molotov_liquid: materials.add(molotov_liquid_sprite.into()),
        pulsewave: materials.add(pulsewave_sprite.into()),
        beam: materials.add(Color::rgba_u8(173, 216, 230, 50).into()),
    });

    commands.insert_resource(ButtonMaterials {
        normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
    });

    const GAME_BUTTON_COLOR: Color = Color::rgb(4.0 / 256.0, 221.0 / 256.0, 185.0 / 256.0);

    commands.insert_resource(GameMenuButtonMaterials {
        normal: materials.add(GAME_BUTTON_COLOR.into()),
        hovered: materials.add((GAME_BUTTON_COLOR * (3.0 / 2.0)).into()),
    });

    commands.insert_resource(MapAssets(map_assets));
}

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Setup the UI
    // The text saying the player's ammo count
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                align_content: AlignContent::FlexStart,
                align_items: AlignItems::FlexStart,
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
        .insert(AmmoText)
        .insert(GameRelated);

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
                sections: vec![TextSection {
                    value: "0%".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0,
                        color: Color::RED,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AbilityChargeText)
        .insert(GameRelated);

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
                sections: vec![TextSection {
                    value: "Health: 0%".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0,
                        color: Color::GREEN,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HealthText)
        .insert(GameRelated);

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
        .insert(GameLogText)
        .insert(GameRelated);

    // Text saying the current score of all players in game
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
                align_items: AlignItems::FlexEnd,

                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|node_parent| {
            node_parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Score\n".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 45.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..Default::default()
                    },
                    visible: Visible {
                        is_visible: false,

                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScoreUI)
                .insert(GameRelated);
        });

    // The text saying that a player won the game
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
                align_items: AlignItems::FlexEnd,

                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|node_parent| {
            node_parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: String::from("Player X won!"),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 45.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..Default::default()
                    },
                    visible: Visible {
                        is_visible: false,

                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ChampionText)
                .insert(GameRelated);
        });
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
                    sections: vec![TextSection {
                        value: "Necrophaser".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::GOLD,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            // Only PC's can host games
            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: Rect {
                            bottom: Val::Percent(10.0),

                            ..Default::default()
                        },
                        size: Size::new(Val::Px(185.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Play"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: String::from("Settings"),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Down);
                });
        });
}

pub fn setup_customize_player(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>, my_ability: Res<Ability>, my_gun_model: Res<Model>, my_perk: Res<Perk>) {
    commands.insert_resource(ClearColor(Color::ORANGE));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexStart,
                margin: Rect {
                    bottom: Val::Auto,

                    ..Default::default()
                },
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexStart,
                align_items: AlignItems::FlexStart,

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
                    sections: vec![TextSection {
                        value: String::from("Customize"),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: Rect {
                            //bottom: Val::Percent(10.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Ability: {:?}", *my_ability),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(450.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Gun: {:?}", *my_gun_model),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(450.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Perk: {:?}", *my_perk),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Back"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: String::from(" "),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CustomizeHelpText);
        });
}

pub fn setup_customize_game(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>, map_crc32: Res<MapCRC32>, maps: Res<Maps>) {
    commands.insert_resource(ClearColor(Color::ORANGE));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexStart,
                margin: Rect {
                    bottom: Val::Auto,

                    ..Default::default()
                },
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexStart,
                align_items: AlignItems::FlexStart,

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
                    sections: vec![TextSection {
                        value: String::from("Customize Game"),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: Rect {
                            //bottom: Val::Percent(10.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Map: {:?}", maps.0.get(&map_crc32.0).unwrap().name),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Back"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: String::from(" "),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CustomizeHelpText);
        });
}

pub fn setup_download_map_menu(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>) {
    commands.insert_resource(ClearColor(Color::ORANGE));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexStart,
                margin: Rect {
                    bottom: Val::Auto,

                    ..Default::default()
                },
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexStart,
                align_items: AlignItems::FlexStart,

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
                    sections: vec![TextSection {
                        value: String::from("Downloading Map..."),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            node_parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: String::from("0.0%"),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: Rect {
                            //bottom: Val::Percent(10.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Cancel"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

pub fn setup_game_menu(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>) {
    commands.insert_resource(ClearColor(Color::ORANGE));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::FlexStart,
                margin: Rect {
                    bottom: Val::Auto,

                    ..Default::default()
                },
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::FlexStart,
                align_items: AlignItems::FlexStart,

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
                    sections: vec![TextSection {
                        value: String::from("Play"),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            // Only PC's can host games
            #[cfg(feature = "native")]
            node_parent
                .spawn_bundle(ButtonBundle {
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
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Host game"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            // Only WASM can join games
            #[cfg(feature = "web")]
            node_parent
                .spawn_bundle(ButtonBundle {
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
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Join game"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(365.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: String::from("Customize Player"),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Down);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: String::from("Customize Game"),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Down);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Back"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

pub fn setup_settings( mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<ButtonMaterials>, keybindings: Res<KeyBindings>, ) {
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.spawn().insert(SelectedKeyButton(None));

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
                    sections: vec![TextSection {
                        value: "Settings".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 75.0,
                            color: Color::GOLD,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Up: {:?}", keybindings.up),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Up);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Down: {:?}", keybindings.down),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Down);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Left: {:?}", keybindings.left),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Left);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Right: {:?}", keybindings.right),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Right);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Ability: {:?}", keybindings.use_ability),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::UseAbility);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Reload: {:?}", keybindings.reload),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Reload);
                });

                node_parent
                .spawn_bundle(ButtonBundle {
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
                                sections: vec![TextSection {
                                    value: format!("Melee: {:?}", keybindings.melee),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 55.0,
                                        color: Color::WHITE,
                                    },
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(KeyBindingButtons::Reload);
                });

            node_parent
                .spawn_bundle(ButtonBundle {
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
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Back"),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

pub fn setup_connection_menu(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<ButtonMaterials>) {
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
                    sections: vec![TextSection {
                        value: String::from("IP to connect to:"),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(NetConnStateText);

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
                            sections: vec![TextSection {
                                value: String::with_capacity(15),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 55.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(IpText);
                    
            });
            
        });
}


pub fn set_player_colors(_ability: &Ability) -> (HelmetColor, InnerSuitColor) {
    // Since shaders aren't (currently) using player colors, removing all the const fn calls should improve compile times
/*    const INFERNO_HELMET_COLOR: HelmetColor = HelmetColor::new([231, 120, 1]);
    const INFERNO_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([232, 35, 0]);

    const ENGINEER_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const ENGINEER_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([238, 166, 34]);

    const HACKER_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const HACKER_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([107, 1, 1]);

    const WARP_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const WARP_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([229, 2, 146]);

    const WALL_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const WALL_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([43, 36, 245]);

    const STIM_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const STIM_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([65, 238, 35]);

    const CLOAK_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const CLOAK_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([158; 3]);

    const PULSEWAVE_HELMET_COLOR: HelmetColor = HelmetColor::new([9, 145, 160]);
    const PULSEWAVE_SUIT_COLOR: InnerSuitColor = InnerSuitColor::new([230, 238, 35]);

    let (helmet_color, inner_suit_color) = match ability {
        Ability::Inferno => (INFERNO_HELMET_COLOR, INFERNO_SUIT_COLOR),
        Ability::Engineer => (ENGINEER_HELMET_COLOR, ENGINEER_SUIT_COLOR),
        Ability::Hacker => (HACKER_HELMET_COLOR, HACKER_SUIT_COLOR),
        Ability::Warp => (WARP_HELMET_COLOR, WARP_SUIT_COLOR),
        Ability::Wall => (WALL_HELMET_COLOR, WALL_SUIT_COLOR),
        Ability::Stim => (STIM_HELMET_COLOR, STIM_SUIT_COLOR),
        Ability::Cloak => (CLOAK_HELMET_COLOR, CLOAK_SUIT_COLOR),
        Ability::PulseWave => (PULSEWAVE_HELMET_COLOR, PULSEWAVE_SUIT_COLOR),
        Ability::Ghost => (PULSEWAVE_HELMET_COLOR, PULSEWAVE_SUIT_COLOR),

    };*/
    let (helmet_color, inner_suit_color) = (HelmetColor::new([9, 145, 160]), InnerSuitColor::new([9, 145, 160]));

    (helmet_color, inner_suit_color)
}


pub fn setup_physics(mut commands: Commands) {
    commands.insert_resource(PhysicsPipeline::new());
    commands.insert_resource(IslandManager::new());
    commands.insert_resource(BroadPhase::new());
    commands.insert_resource(NarrowPhase::new());
    commands.insert_resource(JointSet::new());
    commands.insert_resource(CCDSolver::new());

}