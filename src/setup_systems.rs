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
use crate::config::get_data;
use map::MapCRC32;
use single_byte_hashmap::*;

use ron::de::from_str;

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
    let shader_assets = _shader_assets.unwrap();

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

pub fn setup_default_controls(mut commands: Commands) {
    let key_bindings: KeyBindings = match get_data(String::from("key_bindings")) {
        Some(key_bindings) => {
            from_str(&key_bindings).unwrap()
        },
        None => KeyBindings {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
    
            use_ability: KeyCode::LShift,
            reload: KeyCode::R,
    
            show_score: KeyCode::Tab,
            dash: KeyCode::E,
        },
        
    };

    commands.insert_resource(key_bindings);
}

pub fn setup_id(mut commands: Commands, mut _deathmatch_score: ResMut<DeathmatchScore>) {
    let mut online_player_ids: BTreeSet<u8> = BTreeSet::new();

    #[cfg(feature = "native")]
    {
        online_player_ids.insert(1);
        _deathmatch_score.0.insert(1, 0);
        commands.insert_resource(MyPlayerID(Some(PlayerID(1))));
    }

    #[cfg(feature = "web")]
    commands.insert_resource(MyPlayerID(None));

    commands.insert_resource(OnlinePlayerIDs(online_player_ids));
}