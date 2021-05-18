use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use bevy::render::camera::Camera;

const VERTEX_SHADER: &str = r"
#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Color;

layout(location = 1) out vec3 v_Color;
layout(location = 2) out vec3 color_change;
layout(location = 3) out vec2 v_Uv;



layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) uniform MousePos_value {
    vec3 mouse_pos;
};

void main() {
    vec3 lightColor = vec3(1.0, 1.0, 1.0);

    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = Vertex_Position.xy / vec2(300.0, 300.0);

    // Time varying pixel color
    //vec3 col = 0.5 + 0.5*cos(mouse_pos.x + uv.xyx+vec3(0,2,4));
    vec3 col = vec3(0.0, 0.0, 0.0);

    float color_change = 0.0;
    float mouse_vert_dist = 0.0;

    mouse_vert_dist = distance(mouse_pos[0], Vertex_Position[0]) / 630.0;
    float ambient_light = -0.001;

    color_change = (1 - mouse_vert_dist) / 100.0;

    //v_Color = vec3(Vertex_Position[0] / 200.0, Vertex_Position[1] / 200.0, Vertex_Position[2] / 200.0);
    v_Color = col + vec3(color_change, color_change, color_change) + vec3(ambient_light, ambient_light, ambient_light);

    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
";

const FRAGMENT_SHADER: &str = r"
#version 450

layout(set = 3, binding = 0) uniform WindowSize_value {
    vec2 window_size;
};

layout(set = 2, binding = 0) uniform MousePos_value {
    vec3 mouse_pos;
};

in vec4 gl_FragCoord;


layout(location = 0) out vec4 o_Target;
void main() {
    vec3 color = vec3(0.0, 0.0, 0.0);
    vec2 pixel_pos = gl_FragCoord.xy;
    vec2 mouse_position = mouse_pos.xy;

    float mouse_vert_dist = 0.0;
    float color_change = 0.0;

    mouse_vert_dist = distance(mouse_position, pixel_pos) / 100000.0;
    color_change = mouse_vert_dist;

    o_Target = vec4(color + vec3(color_change, color_change, color_change), 1.0);
}
";

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
struct MousePos {
    value: Vec3,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c880063ba92"]
struct WindowSize {
    value: Vec2,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .add_system(animate_shader.system())
        .run();
}

fn setup(
    mut commands: Commands,
    // We will add a new Mesh for the star being created
    mut meshes: ResMut<Assets<Mesh>>,
    // A pipeline will be added with custom shaders
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    // Access to add new shaders
    mut shaders: ResMut<Assets<Shader>>,
    time: Res<Time>,
    mut render_graph: ResMut<RenderGraph>,
    wnds: Res<Windows>,
) {
    let mut star = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);

    let wnd = wnds.get_primary().unwrap();

    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..10 {
        // Angle of each vertex is 1/10 of TAU, plus PI/2 for positioning vertex 0
        let a = std::f32::consts::FRAC_PI_2 - i as f32 * std::f32::consts::TAU / 10.0;
        // Radius of internal vertices (2, 4, 6, 8, 10) is 100, it's 200 for external
        let r = (1 - i % 2) as f32 * 100.0 + 100.0;
        // Add the vertex coordinates
        v_pos.push([r * a.cos(), r * a.sin(), 0.0]);

    }

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        // Vertex shaders are run once for every vertex in the mesh.
        // Each vertex can have attributes associated to it (e.g. position,
        // color, texture mapping). The output of a shader is per-vertex.
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        // Fragment shaders are run for each pixel belonging to a triangle on
        // the screen. Their output is per-pixel.
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "time_uniform",
        RenderResourcesNode::<MousePos>::new(true),
    );

    render_graph.add_system_node(
        "screen_dimensions",
        RenderResourcesNode::<WindowSize>::new(true),
    );

    star.set_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    let mut v_color = vec![[0.0, 0.0, 0.0]];

    v_color.extend_from_slice(&[[1.0, 1.0, 0.0]; 10]);
    star.set_attribute("Vertex_Color", v_color);

   let mut indices = vec![0, 1, 10];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    star.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));

    // We can now spawn the entities for the star and the camera
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(star),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle,
        )]),
        ..Default::default()
    })
    .insert(MousePos { value: Vec3::new(0.0, 0.0, 0.0) })
    .insert(WindowSize { value: Vec2::new(wnd.width(), wnd.height()) });
    commands
        // And use an orthographic projection
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn animate_shader(time: Res<Time>, mut query: Query<&mut MousePos>, wnds: Res<Windows>, camera: Query<&Transform, With<Camera>>) {
    // assuming there is exactly one main camera entity, so this is OK
    let camera_transform = camera.single().unwrap();

    // get the size of the window that the event is for
    let wnd = wnds.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let cursor_pos = match wnd.cursor_position() {
        Some(pos) => pos,
        None => Vec2::ZERO,

    };

    let p = cursor_pos - size / 2.0;

    // apply the camera transform
    let mut pos_wld: Vec3 = (camera_transform.compute_matrix() * p.extend(0.0).extend(1.0)).into();
    pos_wld.z = 0.0;

    let mut mouse_pos = query.single_mut().unwrap();

    //time_uniform.value = time.seconds_since_startup() as f32 * 2.0;
    //mouse_pos.value = pos_wld / wnd.width() + Vec3::new(0.5, 0.5, 0.5);
    mouse_pos.value = pos_wld;
}
