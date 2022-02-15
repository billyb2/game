#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::prelude::Rect;
use bevy::prelude::*;
use bevy::math::{Size, const_vec2};

use config::*;
use game_types::*;
use map::*;

use map::MapCRC32;
use single_byte_hashmap::*;

use bevy::render::camera::ScalingMode;

use helper_functions::graphics::{spawn_button};

//TODO: Reduce boilerplate by writing functions to generate stuff graphical stuff, instead of doing
//a ton of copy pastes.

pub fn setup_cameras(mut commands: Commands, window: Res<WindowDescriptor>,) {
    commands.spawn_bundle(UiCameraBundle::default());

    let mut orthographic_camera = OrthographicCameraBundle::new_2d();
    orthographic_camera.orthographic_projection.scaling_mode = ScalingMode::WindowSize;

    commands
        .spawn_bundle(orthographic_camera)
        .insert(GameCamera);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let res_scale = (window.width / 1366.0).min(window.height / 768.0) * 0.95;
        commands.insert_resource(ResScale(res_scale.recip()));
        
    };
}

pub fn setup_materials(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(debug_assertions)]
    asset_server.watch_for_changes().unwrap();

    //TODO: Use a spritesheet
    // The gorgeous assets are made by Shelby
    let p_pistol_sprite = asset_server.load("player_sprites/player_pistol.png");
    let p_ar_sprite = asset_server.load("player_sprites/player_assault_rifle.png");
    let p_shotgun_sprite = asset_server.load("player_sprites/player_shotgun.png");
    let p_speedball_sprite = asset_server.load("player_sprites/player_speedball.png");
    let p_br_sprite = asset_server.load("player_sprites/player_burst_rifle.png");
    let p_smg_sprite = asset_server.load("player_sprites/player_submachinegun.png");
    let p_cluster_shotgun_sprite = asset_server.load("player_sprites/player_cluster_shotgun.png");
    let p_flamethrower_sprite = asset_server.load("player_sprites/player_flamethrower.png");
    let p_sniper_sprite = asset_server.load("player_sprites/player_sniper.png");
    let p_sprite = asset_server.load("player_sprites/player.png");
    //TODO: Widowmaker sprite
    let p_widowmaker_sprite = asset_server.load("player_sprites/player_sniper.png");
    let p_bow = asset_server.load("player_sprites/player_bow.png");
    //TODO: StickyGrenade sprite
    let p_sticky_grenade = asset_server.load("player_sprites/player_bow.png");
    let p_lachancla_sprite = asset_server.load("player_sprites/player_lachancla.png");

    let enemy_sprite = asset_server.load("player_sprites/enemy.png");

    let molotov_fire_sprite = asset_server.load("projectile_sprites/molotov_fire.png");
    let molotov_liquid_sprite = asset_server.load("projectile_sprites/molotov_liquid.png");

    let pulsewave_sprite = asset_server.load("projectile_sprites/pulsewave.png");
    let arrow_sprite = asset_server.load("projectile_sprites/arrow.png");
    let lachancla_sprite = asset_server.load("projectile_sprites/lachancla.png");

    let shield_cell_sprite = asset_server.load("misc/shield_cell.png");

    let rng = fastrand::Rng::new();

    let flame1 = rng.u8(200..=250);
    let flame2 = rng.u8(100..=150);
    let flame3 = rng.u8(100..=250);    

    #[cfg(target_arch = "wasm32")]
    let map_assets: HashMap<u8, Handle<Image>> = HashMap::with_capacity_and_hasher(20, BuildHasher::default());

    // Web builds can't preload assets
    #[cfg(not(target_arch = "wasm32"))]
    let map_assets: HashMap<u8, Handle<Image>> = {
        let assets = asset_server.load_folder("map_assets/").unwrap();

        assets.iter().map(|asset| {
            let asset = asset.clone().typed();
            let asset_path = asset_server.get_handle_path(asset.clone()).unwrap();
            let path = asset_path.path();
            let file_name_string = path.file_stem().unwrap().to_str().unwrap();

            let int = file_name_string.parse::<u8>().unwrap();

            (int, asset.into())

        }).collect()

    };

    commands.insert_resource(Skin {
        player: [
            // All the sprite sizes are manually calculated, since I can't figure out a way using Bevy to calculate them automatically
            (p_pistol_sprite.into(), const_vec2!([82.808, 61.0755])),
            (p_shotgun_sprite.into(), const_vec2!([89.5625, 63.099])), 
            (p_speedball_sprite.into(), const_vec2!([119.97, 85.3565])), 
            (p_br_sprite.into(), const_vec2!([142.8885, 63.099])), 
            (p_ar_sprite.into(), const_vec2!([143.6135, 61.099])), 
            (p_smg_sprite.into(), const_vec2!([105.3375, 71.4285])), 
            (p_cluster_shotgun_sprite.into(), const_vec2!([94.6025, 69.964])), 
            (p_flamethrower_sprite.into(), const_vec2!([117.351, 79.0005])), 
            (p_sniper_sprite.into(), const_vec2!([143.6135, 63.099])),
            (p_sprite.into(), const_vec2!([82.808, 61.0755])),
            (p_widowmaker_sprite.into(), const_vec2!([143.6135, 63.099])),
            (p_bow.into(), const_vec2!([124.107, 96.8625])),
            (p_sticky_grenade.into(), const_vec2!([124.107, 96.8625])),
            (p_lachancla_sprite.into(), const_vec2!([82.808, 61.0755])),

        ],
        invisible: Color::rgba_u8(0, 0, 0, 0).into(),
        half_invisible: Color::rgba_u8(255, 255, 255, 128).into(),
        enemy: (enemy_sprite.into(), const_vec2!([82.808, 61.0755])),

    });

    commands.insert_resource(ProjectileMaterials {
        // TODO: Image spirtes
        regular: Color::BLACK.into(),
        speedball: Color::rgb_u8(126, 192, 238).into(),
        flamethrower1: Color::rgb_u8(flame1, 43, 9).clone().into(),
        flamethrower2: Color::rgb_u8(221, flame2, 9).into(),
        flamethrower3: Color::rgb_u8(flame3, 43, 12).into(),
        engineer: Color::rgb_u8(255, 0, 200).into(),
        molotov: Color::rgb_u8(232, 35, 0).into(),
        molotov_fire: molotov_fire_sprite.into(),
        molotov_liquid: molotov_liquid_sprite.clone().into(),
        pulsewave: pulsewave_sprite.into(),
        beam: Color::rgba_u8(173, 216, 230, 75).into(),
        arrow: arrow_sprite.into(),
        used_bullet: Color::rgb(0.5, 0.5, 0.5).into(),
        shield_cell: shield_cell_sprite.into(),
        chancla: lachancla_sprite.into(),
    });

    commands.insert_resource(ButtonMaterials {
        normal: UiColor(Color::rgb(4.0 / 255.0, 221.0 / 255.0, 185.0 / 255.0)),
        hovered: UiColor(Color::rgb(4.0 / 255.0, 221.0 / 255.0, 185.0 / 255.0)),
    });

    const GAME_BUTTON_COLOR: Color = Color::rgb(4.0 / 255.0, 221.0 / 255.0, 185.0 / 255.0);

    commands.insert_resource(GameMenuButtonMaterials {
        normal: UiColor(GAME_BUTTON_COLOR),
        hovered: UiColor(GAME_BUTTON_COLOR),
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

    // Text saying the game log
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

    // Text saying the current game chat
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(2.5),
                    top: Val::Percent(5.0),

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
        .insert(ChatLogText)
        .insert(GameRelated);

    // Text saying the player's chat input
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(2.5),
                    bottom: Val::Percent(6.0),

                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: {
                    let mut text_vec = Vec::with_capacity(25);

                    text_vec.push(TextSection {
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            // The text size becomes smaller as the actual text becomes larger, so that it will always fit on the screen
                            font_size: 25.0,
                            color: Color::WHITE,
                        },
                        value: String::from("Chat: "),

                    });

                    text_vec

                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ChatText)
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
            visibility: Visibility {
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
                    visibility: Visibility {
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
            visibility: Visibility {
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
                    visibility: Visibility {
                        is_visible: false,

                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ChampionText)
                .insert(GameRelated);
        });
}

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            visibility: Visibility {
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

            const MARGIN: Rect<Val> = Rect {
                bottom: Val::Percent(10.0),
                top: Val::Percent(0.0),
                left: Val::Percent(0.0),
                right: Val::Percent(0.0),

            };

            spawn_button::<{ Some(185.0) } , 85.0>(node_parent, String::from("Play"), asset_server.load("fonts/FiraSans-Bold.ttf"), MARGIN);

            spawn_button::<{ Some(225.0) } , 85.0>(node_parent, String::from("Settings"), asset_server.load("fonts/FiraSans-Bold.ttf"), Default::default());

        });
}

pub fn setup_customize_player(mut commands: Commands, asset_server: Res<AssetServer>, my_ability: Res<Ability>, my_gun_model: Res<Model>, my_perk: Res<Perk>, my_player_name: Res<PlayerName>) {
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
            visibility: Visibility {
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Ability: {}", *my_ability),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Gun: {}", *my_gun_model),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Perk: {}", *my_perk),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: String::from("Click to set name"),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                            value: String::new(),
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


            node_parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{}", *my_player_name),
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
                .insert(NameText);
        });
}

pub fn add_player_name_text(mut commands: Commands, names: Query<(Entity, &PlayerName, &Transform,  &Visibility)>, asset_server: Res<AssetServer>) {
    names.for_each(|(parent_entity, player_name, transform, visibility)| {
        let transform = Transform {
            translation: transform.translation.normalize(),
            rotation: transform.rotation.inverse(),
            scale: Vec3::ONE,
        };


        let child_entity = commands.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("{}", player_name),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            transform,
            visibility: visibility.clone(),
            ..Default::default()
        }).id();

        commands.entity(parent_entity).push_children(&[child_entity]);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn setup_customize_game(mut commands: Commands, asset_server: Res<AssetServer>, map_crc32: Res<MapCRC32>, maps: Res<Maps>, num_of_bots: Res<NumOfBots>, bot_algs: Res<BotAlgs>) {
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
            visibility: Visibility {
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                        margin: Rect {
                            //bottom: Val::Percent(10.0),
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Number of bots: {}", num_of_bots.0),
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
                        margin: Rect {
                            ..Default::default()
                        },
                        size: Size::new(Val::Px(650.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("Bot algorithm: {}", &bot_algs.algs[bot_algs.current_index].0),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                            value: String::new(),
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

pub fn setup_download_map_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            visibility: Visibility {
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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

pub fn setup_game_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            visibility: Visibility {
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
            #[cfg(not(target_arch = "wasm32"))]
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .with_children(|button_parent| {
                    button_parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: String::from("Bot Battle"),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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

            #[cfg(not(target_arch = "wasm32"))]
            node_parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(350.0), Val::Px(85.0)),

                        ..Default::default()
                    },
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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

pub fn setup_settings( mut commands: Commands, asset_server: Res<AssetServer>, keybindings: Res<KeyBindings>, ) {
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
            visibility: Visibility {
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
                        .insert(KeyBindingButtons::Melee);
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
                    color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
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
            visibility: Visibility {
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
                color: UiColor(Color::rgb(0.15, 0.15, 0.15)),
                ..Default::default()
            })
            .with_children(|button_parent| {
                button_parent
                    .spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: get_data("server_ip").unwrap_or(String::with_capacity(15)),
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

pub fn setup_default_controls(mut commands: Commands) {
    let key_bindings: KeyBindings = match get_data("key_bindings") {
        Some(key_bindings) => key_bindings,
        None => {
            let key_bindings = KeyBindings {
                up: KeyCode::W,
                down: KeyCode::S,
                left: KeyCode::A,
                right: KeyCode::D,
    
                use_ability: KeyCode::LShift,
                reload: KeyCode::R,
    
                show_score: KeyCode::Tab,
                dash: KeyCode::E,
                melee: KeyCode::F,
                talk: KeyCode::T,
            };

            write_data("key_bindings", key_bindings);

            key_bindings

        },
        
    };

    commands.insert_resource(key_bindings);

}
