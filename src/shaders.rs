#![deny(clippy::all)]

use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::shader::ShaderStages;

use game_types::*;

// All this is to keep hot shaders from crashing (see https://github.com/bevyengine/bevy/issues/1359)

pub fn setup_asset_loading(asset_server: Res<AssetServer>, mut commands: Commands,) {
    asset_server.watch_for_changes().unwrap();

    #[cfg(feature = "web")]
    let (vert_shader, player_frag_shader, lighting_frag_shader) = (
        "shaders/sprite_wasm.vert", 
        "shaders/player_wasm.frag", 
        "shaders/lighting_wasm.frag"
    );

    #[cfg(feature = "native")]
    let (vert_shader, player_frag_shader, lighting_frag_shader) = (
        "shaders/sprite.vert", 
        "shaders/player.frag", 
        "shaders/lighting.frag"
    );

    commands.insert_resource(AssetsLoading {
        loaded: false,
        vertex: asset_server.load::<Shader, _>(vert_shader),
        player_frag: asset_server.load::<Shader, _>(player_frag_shader),
        lighting_frag: asset_server.load::<Shader, _>(lighting_frag_shader),
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

    if 
        asset_server.get_load_state(loading.player_frag.clone()) != LoadState::Loaded ||
        asset_server.get_load_state(loading.lighting_frag.clone()) != LoadState::Loaded ||
        asset_server.get_load_state(loading.vertex.clone()) != LoadState::Loaded
    {
        return;
    }

    loading.loaded = true;

    let _ = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: loading.vertex.clone(),
        fragment: Some(loading.player_frag.clone()),
    }));

    let _ = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: loading.vertex.clone(),
        fragment: Some(loading.lighting_frag.clone()),
    }));
}
