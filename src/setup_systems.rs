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
use config::*;
use map::MapCRC32;
use single_byte_hashmap::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

#[cfg(feature = "graphics")]
use crate::setup_graphical_systems::*;

#[allow(clippy::too_many_arguments)]
pub fn setup_players(mut commands: Commands, _materials: Option<Res<Skin>>, maps: Res<Maps>, mut _pipelines: Option<ResMut<Assets<PipelineDescriptor>>>, mut _render_graph: Option<ResMut<RenderGraph>>, _wnds: Option<Res<Windows>>, _shader_assets: Option<Res<AssetsLoading>>, map_crc32: Res<MapCRC32>, mut _deathmatch_score: ResMut<DeathmatchScore>, my_gun_model: Option<Res<Model>>, my_ability: Option<Res<Ability>>, my_perk: Option<Res<Perk>>, mut _rigid_body_set: Option<ResMut<RigidBodySet>>, mut _collider_set: Option<ResMut<ColliderSet>>) {
    let mut available_player_ids: Vec<PlayerID> = Vec::with_capacity(10);
    let mut player_entities: HashMap<u8, Entity> = HashMap::with_capacity_and_hasher(10, BuildHasher::default());

    #[cfg(feature = "graphics")]
    let wnds = _wnds.unwrap();

    #[cfg(feature = "graphics")]
    let wnd = wnds.get_primary().unwrap();

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

    let map = maps.0.get(&map_crc32.0).unwrap();
    
    map.spawn_points.iter().enumerate().for_each(|(mut i, coords)| {
        i += 1;

        let ability = match &my_ability {
            Some(ability) => **ability,
            None => Ability::Engineer,

        };

        let gun_model = match &my_gun_model {
            Some(gun_model) => **gun_model,
            None => Model::Pistol,

        };

        let perk = match &my_perk {
            Some(perk) => **perk,
            None => Perk::ExtendedMag,

        };


        #[cfg(feature = "graphics")]
        let (helmet_color, inner_suit_color) = set_player_colors(&ability);

        let mut entity = commands.spawn_bundle(Player::new(i.try_into().unwrap(), ability, perk, false));

        entity
            .insert_bundle(Gun::new(gun_model, ability, perk))
            .insert(Transform::from_translation(coords.extend(101.0)));

        #[cfg(feature = "graphics")]
        entity
            .insert_bundle(SpriteBundle {
                material: _materials.as_ref().unwrap().player.clone(),
                sprite: Sprite {
                    size: Vec2::new(150.0, 93.75),
                    flip_x: true, 
                    resize_mode: SpriteResizeMode::Automatic,

                    ..Default::default()
                },
                visible: Visible {
                    is_visible: false,
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
        available_player_ids.push(PlayerID(i.try_into().unwrap()));

        #[cfg(feature = "graphics")]
        { 
        let rigid_body_set = _rigid_body_set.as_mut().unwrap();
        let collider_set = _collider_set.as_mut().unwrap();

        let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .translation(Vector2::new(coords.x, coords.y).component_div(&Vector2::new(250.0, 250.0)))
            .linvel(Vector2::new(0.0, 0.0))
            .gravity_scale(0.0)
            .linear_damping(80.0)
            .user_data(u128::MAX)
            //CCD is purposely disabled so stuff like warping works
            .ccd_enabled(false)
            .build();

        let collider_size = Vec2::new(150.0, 93.75) / Vec2::new(500.0, 500.0);

        let collider = ColliderBuilder::cuboid(collider_size.x, collider_size.x)
            .collision_groups(InteractionGroups::new(0b1000, 0b1111))
            .restitution(0.000001)
            .friction(0.4)
            .user_data(u128::MAX)
            .build();

        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

        entity.insert(rigid_body_handle);
        entity.insert(collider_handle);

        }

    });
    #[allow(unused_mut)]
    let mut online_player_ids = HashMap::with_capacity_and_hasher(10, BuildHasher::default());

    #[cfg(all(feature = "native", feature = "graphics"))]
    {
        let id = available_player_ids.remove(0);
        online_player_ids.insert(id.0, None);
        _deathmatch_score.0.insert(id.0, 0);
        commands.insert_resource(MyPlayerID(Some(id)));
        
    }

    #[cfg(feature = "web")]
    commands.insert_resource(MyPlayerID(None));

    commands.insert_resource(OnlinePlayerIDs(online_player_ids));

    commands.insert_resource(available_player_ids);
    commands.insert_resource(player_entities);
}

pub fn setup_default_controls(mut commands: Commands) {
    let key_bindings: KeyBindings = match get_data(String::from("key_bindings")) {
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
            };

            write_data(String::from("key_bindings"), key_bindings);

            key_bindings

        },
        
    };

    commands.insert_resource(key_bindings);
}
