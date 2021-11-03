#![deny(clippy::all)]

use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::shader::ShaderStages;

use game_types::*;

// All this is to keep hot shaders from crashing (see https://github.com/bevyengine/bevy/issues/1359)

pub fn setup_asset_loading(asset_server: Res<AssetServer>, mut commands: Commands,) {
    asset_server.watch_for_changes().unwrap();

    let (vert_shader, frag_shader) = match cfg!(feature = "web") {
        true => ("shaders/sprite_wasm.vert", "shaders/sprite_wasm.frag"),
        false => ("shaders/sprite.vert", "shaders/sprite.frag"),
    };

    commands.insert_resource(AssetsLoading {
        loaded: false,
        vertex_shader: asset_server.load::<Shader, _>(vert_shader),
        fragment_shader: asset_server.load::<Shader, _>(frag_shader),
    });

}

pub fn check_assets_ready(
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
) {
    if loading.loaded {
        return;
    }

    if asset_server.get_load_state(loading.fragment_shader.clone()) != LoadState::Loaded
        || asset_server.get_load_state(loading.vertex_shader.clone()) != LoadState::Loaded
    {
        return;
    }

    loading.loaded = true;

    let _ = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: loading.vertex_shader.clone(),
        fragment: Some(loading.fragment_shader.clone()),
    }));
}
