use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::reflect::TypeUuid;

use bevy::render::renderer::RenderResources;
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::shader::ShaderStages;

use bevy::math::const_vec3;

// The UUID is just random

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-5c880063ba92"]
pub struct HelmetColor {
    pub value: Vec3,

}

const fn u8_to_color(value: [u8; 3]) -> [f32; 3] {
    let new_values: [f32; 3] = {
        let mut new_values: [f32; 3] = [0.0; 3];

        let mut i = 0;

        while i < value.len() {
            let mut v: f32 = value[i] as f32;
            v /= 255.0;

            new_values[i] = v;

            i += 1;

        }

        new_values

    };
    
    new_values
}

impl HelmetColor {
    //TODO: this function is a great canidate for SIMD
    pub const fn new(value: [u8; 3]) -> Self {
        Self {
            value: const_vec3!(u8_to_color(value)),

        }

    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c881163ba92"]
pub struct InnerSuitColor {
    pub value: Vec3,

}

impl InnerSuitColor {
    pub const fn new(value: [u8; 3]) -> Self {
        Self {
            value: const_vec3!(u8_to_color(value)),

        }
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d554-4fc2-bc9f-4c881163ba92"]
pub struct ShaderPhasing {
    pub value: f32,
}


// All this is to keep hot shaders from crashing (see https://github.com/bevyengine/bevy/issues/1359)

#[derive(Default)]
pub struct AssetsLoading {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
    loaded: bool,
}

pub fn setup_asset_loading(asset_server: Res<AssetServer>, mut commands: Commands,) {
    asset_server.watch_for_changes().unwrap();

    // Web builds use a slightly different shader language
    #[cfg(feature = "web")]
    commands.insert_resource(AssetsLoading {
        loaded: false,
        vertex_shader: asset_server.load::<Shader, _>("shaders/sprite_wasm.vert"),
        fragment_shader: asset_server.load::<Shader, _>("shaders/sprite_wasm.frag"),
    });

    #[cfg(feature = "native")]
    commands.insert_resource(AssetsLoading {
        loaded: false,
        vertex_shader: asset_server.load::<Shader, _>("shaders/sprite.vert"),
        fragment_shader: asset_server.load::<Shader, _>("shaders/sprite.frag"),
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
