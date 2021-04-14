mod map2;

//use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;
use bevy::render::camera::Camera;

use map2::*;

use getrandom::getrandom;

struct Health(u8);

//Each player has a unique player id
struct Player(u8);

#[derive(Copy, Clone)]
pub struct Size {
    pub w: f32,
    pub h: f32,

}

#[derive(Copy, Clone)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

impl Coords {
    fn new(x: f32, y: f32) -> Coords {
        Coords {
            x,
            y,
        }
    }
}

struct Skins {
    phase: Handle<ColorMaterial>,

}

struct Tiles {
    colors: Vec<Handle<ColorMaterial>>,

}


fn main() {
    App::build()
        //Antialiasing
        .insert_resource(Msaa { samples: 8 })
        // The background is a black rectangle
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        //Just checks for possible ambiguouty issue
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Map::from_bin(include_bytes!("../tiled/map1.custom")))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_graphics.system())
        //Spawning players happens in its own stage since setup_graphics needs to happen first
        .add_startup_stage("setup_game",
        SystemStage::parallel()
            .with_system(add_players.system())
            .with_system(init_map.system().label("init_map"))
            .with_system(draw_map.system().after("init_map"))
        )
        .add_system(move_players.system())
        .add_system(move_camera.system())
        .run();
}

fn setup_graphics(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Skins {
        phase: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),

    });

}

fn add_players(mut commands: Commands, materials: Res<Skins>, asset_server: Res<AssetServer>) {
    for i in 0..=9 {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.phase.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                // Since everything in bevy is 3d, we just work from a flat plane, so the z axis is 0
                ..Default::default()
            })
            .insert(Player(i))
            .insert_bundle(Text2dBundle {
                text: Text::with_section(
                    100.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                transform: Transform::from_xyz(i as f32 * 25.0, 300.0, 0.0),
                ..Default::default()
            })
            .insert(Health(100));

    }


}

fn init_map(mut map: ResMut<Map>) {
    map.objects.push(MapObject::new(200.0, 200.0, Color::rgba_u8(255, 255, 255, 255), false, false, Some(100)));

}

fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, map: Res<Map>) {
    let mut i = 0;

    while i < (*map).objects.len() {
        let map_object = (*map).objects[i].coords;
        let color = (*map).objects[i].color;

        //Either create a new material, or grab a currently existing one
        let color: Handle<ColorMaterial> = {
            let mut color_to_return = None;

            for (id, material_to_return) in materials.iter() {
                if color == material_to_return.color {
                    color_to_return = Some(materials.get_handle(id));

                }

            }


            if color_to_return.is_some() {
                color_to_return.unwrap()


            } else {
                materials.add(color.into())

            }
        };

        commands
            .spawn_bundle(SpriteBundle {
                material: color.clone(),
                sprite: Sprite::new(Vec2::new(15.0, 15.0)),
                transform: Transform::from_xyz(map_object.x, map_object.y, 0.0),
                ..Default::default()
            })
            .insert(map_object);

        i += 1;
    }
}

fn move_players(mut players: Query<&mut Transform, With<Player>>, map: Res<Map>) {
    // Gotta make it negative since up is positive and down is negative
    for mut player in players.iter_mut() {
        if -player.translation.y <= (*map).size.h {
            player.translation.y -= 3.0;

        }

    }

}

fn move_camera(keyboard_input: Res<Input<KeyCode>>, mut cameras: Query<&mut Transform, With<Camera>>) {
    for mut camera in cameras.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            camera.translation.x -= 6.0;

        }
        if keyboard_input.pressed(KeyCode::D) {
            camera.translation.x += 6.0;

        }
        if keyboard_input.pressed(KeyCode::S) {
            camera.translation.y -= 6.0;

        }
        if keyboard_input.pressed(KeyCode::W) {
            camera.translation.y += 6.0;

        }
    }
}
