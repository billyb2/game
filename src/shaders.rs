use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::shader::ShaderStages;
use bevy::render::render_graph::RenderGraph;

use lazy_static::lazy_static;

use crate::*;

// The UUID is just random

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-5c880063ba92"]
pub struct HelmetColor {
    pub value: Vec3,

}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c881163ba92"]
pub struct InnerSuitColor {
    pub value: Vec3,

}

// Need to adjust the position of the mouse for the shader (for  some reason, I'm unsure why)
// Lazy satatic lets us do a little more than a const, except that it's run once at runtime instead of at compile time
lazy_static! {
    static ref ADJUSTMENT: Vec3 = Vec3::new(50.0, 25.0, 0.0);

}

pub fn animate_shaders(mut query: Query<&mut MousePosition>, wnds: Res<Windows>, camera: Query<&Transform, With<GameCamera>>) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = camera.single().unwrap();

    // get the size of the window that the event is for
    let wnd = wnds.get_primary().unwrap();

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let cursor_pos = match wnd.cursor_position() {
        Some(pos) => pos,
        None => Vec2::ZERO,

    };

    let p = cursor_pos;

    // apply the camera transform
    let mut pos_wld: Vec3 = (camera_transform.compute_matrix() * p.extend(0.0).extend(1.0)).into();
    pos_wld.z = 0.0;
    pos_wld.y = wnd.height() - pos_wld.y;

    pos_wld += *ADJUSTMENT;


    for mut mouse_pos in query.iter_mut() {
        mouse_pos.value = pos_wld;

    }
}


// All this is to keep hot shaders from crashing (see https://github.com/bevyengine/bevy/issues/1359)

#[derive(Default)]
pub struct AssetsLoading {
    vertex_shader: Handle<Shader>,
    fragment_shader: Handle<Shader>,
    loaded: bool,
}

pub fn setup_asset_loading(asset_server: Res<AssetServer>, mut commands: Commands,) {
    asset_server.watch_for_changes().unwrap();

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
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