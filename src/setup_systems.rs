#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

// This file is for storing all systems that are used as setups, such as setting up cameras, drawing the map, etc
use std::collections::BTreeSet;
use std::convert::TryInto;

use bevy::prelude::*;

use bevy::render::{
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{RenderGraph, RenderResourcesNode},
    shader::ShaderStages,
};

use crate::*;
use crate::shaders::*;
use map::MapCRC32;
use single_byte_hashmap::*;

#[cfg(feature = "graphics")]
use crate::setup_graphical_systems::*;

#[allow(clippy::too_many_arguments)]
pub fn setup_players(mut commands: Commands, _materials: Option<Res<Skin>>, maps: Res<Maps>, mut _pipelines: Option<ResMut<Assets<PipelineDescriptor>>>, mut _render_graph: Option<ResMut<RenderGraph>>, _wnds: Option<Res<Windows>>, _shader_assets: Option<Res<AssetsLoading>>, map_crc32: Res<MapCRC32>) {
    let mut availabie_player_ids: Vec<PlayerID> = Vec::with_capacity(10);
    let mut player_entities: HashMap<u8, Entity> = HashMap::with_capacity_and_hasher(10, BuildHasher::default());

    #[cfg(feature = "graphics")]
    let wnd = _wnds.unwrap();
    #[cfg(feature = "graphics")]
    let wnd = wnd.get_primary().unwrap();

    #[cfg(feature = "graphics")]
    let mut shader_assets = _shader_assets.unwrap();

    #[cfg(feature = "graphics")]
    let pipeline_handle = _pipelines.unwrap().add(PipelineDescriptor::default_config(ShaderStages {
        // Vertex shaders are run once for every vertex in the mesh.
        // Each vertex can have attributes associated to it (e.g. position,
        // color, texture mapping). The output of a shader is per-vertex.
        vertex: shader_assets.vertex_shader.clone(),
        // Fragment shaders are run for each pixel
        fragment: Some(shader_assets.fragment_shader.clone()),
    }));

    #[cfg(feature = "graphics")]
    {
        let mut render_graph = _render_graph.unwrap();
        render_graph.add_system_node(
            "mouse_position",
            RenderResourcesNode::<ShaderMousePosition>::new(true),
        );

        render_graph.add_system_node(
            "screen_dimensions",
            RenderResourcesNode::<WindowSize>::new(true),
        );

        render_graph.add_system_node(
            "helmet_color",
            RenderResourcesNode::<HelmetColor>::new(true),
        );

        render_graph.add_system_node(
            "inner_suit_color",
            RenderResourcesNode::<InnerSuitColor>::new(true),
        );

        render_graph.add_system_node(
            "alpha",
            RenderResourcesNode::<Alpha>::new(true),
        );
    }

    let mut living = true;

    let map = maps.0.get(&map_crc32.0).unwrap();
    
    map.spawn_points.iter().enumerate().for_each(|(mut i, coords)| {
        i += 1;

        let ability = Ability::Engineer;
        let gun_model = Model::AssaultRifle;
        let perk = Perk::ExtendedMag;


        #[cfg(feature = "graphics")]
        let (helmet_color, inner_suit_color) = set_player_colors(&ability);

        let mut entity = 
        commands
            .spawn_bundle(Player::new(i.try_into().unwrap(), ability, perk, living));

        entity
            .insert_bundle(Gun::new(gun_model, ability, perk))
            .insert(Transform::from_translation(coords.extend(101.0)));

        #[cfg(feature = "graphics")]
        entity
            .insert_bundle(SpriteBundle {
                material: match i {
                    1 => _materials.as_ref().unwrap().player.clone(),
                    _ => _materials.as_ref().unwrap().enemy.clone(),

                },
                sprite: Sprite {
                    size: Vec2::new(120.0, 75.0),
                    flip_x: true,
                    resize_mode: SpriteResizeMode::Automatic,

                    ..Default::default()
                },
                visible: Visible {
                    is_visible: living,
                    is_transparent: true,
                },
                transform: Transform::from_translation(coords.extend(101.0)),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline_handle.clone(),
                )]),

                ..Default::default()
            })
            .insert(ShaderMousePosition { value: Vec2::ZERO })
            .insert(WindowSize {
                value: Vec2::new(wnd.width(), wnd.height()),
            })
            .insert(GameRelated)
            .insert(Alpha { value: 1.0})
            .insert(helmet_color)
            .insert(inner_suit_color);

        player_entities.insert(i.try_into().unwrap(), entity.id());
        availabie_player_ids.push(PlayerID(i.try_into().unwrap()));

        living = false;
    });

    commands.insert_resource(availabie_player_ids);
    commands.insert_resource(player_entities);

    #[cfg(not(feature = "graphics"))]
    commands.insert_resource(OnlinePlayerIDs(BTreeSet::new()));
}

/*pub fn setup_continue_menu(mut commands: Commands, asset_server: Res<AssetServer>, button_materials: Res<GameMenuButtonMaterials>) {
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
                    sections: vec![
                        TextSection {
                            value: String::from("Continue playing?"),
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
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: Rect {
                    bottom: Val::Percent(10.0),

                    ..Default::default()
                },
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
                            sections: vec![
                                TextSection {
                                    value: String::from("Yes"),
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
                size: Size::new(Val::Px(450.0), Val::Px(85.0)),

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
                                    value: String::from("No"),
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
}*/

pub fn setup_default_controls(mut commands: Commands) {
    commands.insert_resource(KeyBindings {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,

        use_ability: KeyCode::LShift,
        reload: KeyCode::R,

        show_score: KeyCode::Tab,
        dash: KeyCode::E,
    });
}

pub fn setup_id(mut commands: Commands, mut deathmatch_score: ResMut<DeathmatchScore>) {
    let mut online_player_ids: BTreeSet<u8> = BTreeSet::new();

    #[cfg(feature = "native")]
    {
        online_player_ids.insert(1);
        deathmatch_score.0.insert(1, 0);
        commands.insert_resource(MyPlayerID(Some(PlayerID(1))));
    }

    #[cfg(feature = "web")]
    commands.insert_resource(MyPlayerID(None));

    commands.insert_resource(OnlinePlayerIDs(online_player_ids));
}