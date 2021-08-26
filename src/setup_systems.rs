#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

// This file is for storing all systems that are used as setups, such as setting up cameras, drawing the map, etc
use std::collections::BTreeSet;

use bevy::prelude::*;

use bevy::render::{
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::{RenderGraph, RenderResourcesNode},
    shader::ShaderStages,
};

use crate::*;
use crate::shaders::*;
use single_byte_hashmap::*;

#[allow(clippy::too_many_arguments)]
pub fn setup_players(mut commands: Commands, materials: Res<Skin>, maps: Res<Maps>, mut pipelines: ResMut<Assets<PipelineDescriptor>>, mut render_graph: ResMut<RenderGraph>, wnds: Res<Windows>, my_ability: Res<Ability>, my_gun_model: Res<Model>, my_perk: Res<Perk>, shader_assets: Res<AssetsLoading>, map_crc32: Res<MapCRC32>) {
    let mut i: u8 = 0;

    let mut availabie_player_ids: Vec<PlayerID> = Vec::with_capacity(256);
    let mut player_entities: HashMap<u8, Entity> =
        HashMap::with_capacity_and_hasher(256, BuildHasher::default());

    let wnd = wnds.get_primary().unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        // Vertex shaders are run once for every vertex in the mesh.
        // Each vertex can have attributes associated to it (e.g. position,
        // color, texture mapping). The output of a shader is per-vertex.
        vertex: shader_assets.vertex_shader.clone(),
        // Fragment shaders are run for each pixel
        fragment: Some(shader_assets.fragment_shader.clone()),
    }));

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

    let mut living = true;

    if let Some(map) = maps.0.get(&map_crc32.0) {
        map.spawn_points.iter().for_each(|coords| {
            let ability = *my_ability;
            let gun_model = *my_gun_model;
            let perk = *my_perk;

            let (helmet_color, inner_suit_color) = set_player_colors(&ability);

            let entity = commands
                .spawn_bundle(Player::new(i, ability, perk, living))
                .insert_bundle(Gun::new(gun_model, ability, perk))
                .insert_bundle(SpriteBundle {
                    material: match i {
                        0 => materials.player.clone(),
                        _ => materials.enemy.clone(),

                    },
                    sprite: Sprite {
                        size: Vec2::new(120.0, 75.0),
                        flip_x: true,
                        resize_mode: SpriteResizeMode::Manual,

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
                .insert(helmet_color)
                .insert(inner_suit_color)
                .insert(GameRelated)
                .insert(Alpha { value: 1.0})
                .id();

            player_entities.insert(i, entity);

            if i != 0 {
                availabie_player_ids.push(PlayerID(i));
            }

            living = false;

            i += 1;
        });
    }

    commands.insert_resource(availabie_player_ids);
    commands.insert_resource(player_entities);
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
        online_player_ids.insert(0);
        deathmatch_score.0.insert(0, 0);
        commands.insert_resource(MyPlayerID(Some(PlayerID(0))));
    }

    #[cfg(feature = "web")]
    commands.insert_resource(MyPlayerID(None));

    commands.insert_resource(OnlinePlayerIDs(online_player_ids));
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
