mod map;
mod game_logic_2;
mod helper_functions_2;

//use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::sprite::SpriteSettings;

use map::*;

#[derive(Debug, PartialEq)]
struct RequestedMovement(Coords);

#[derive(Debug, PartialEq)]
struct Health(u8);

#[derive(Debug, PartialEq)]
struct ID(u8);

//Each player has a unique player id
#[derive(Bundle, Debug, PartialEq)]
struct Player {
    id: ID,
    health: Health,
    requested_movement: RequestedMovement,

}

impl Player {
    fn new(id: u8) -> Player {
        Player {
            id: ID(id),
            health: Health(100),
            requested_movement: RequestedMovement(Coords::new(0.0, 0.0)),

        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Size {
    pub w: f32,
    pub h: f32,

}

impl Size {
    fn new(w: f32, h: f32) -> Size {
        Size {
            w,
            h,
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
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

//Anything that moves an object
#[derive(SystemLabel, Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct MoveReq;

fn main() {
    App::build()
        //Antialiasing
        .insert_resource(Msaa { samples: 1 })
        .insert_resource( WindowDescriptor {
            //vsync: true,
            ..Default::default()

        })
        // Sprite culling doesn't render sprites outside of the camera viewport when enabled
        // It's fairly buggy when rendering many many sprites at the same time, however
        // Frustum culling also doesn't work with more than 1 camera, so it needs to be disabled for split screen
        .insert_resource(SpriteSettings { frustum_culling_enabled: true })
        //Just checks for possible ambiguouty issue
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(Map::from_bin(include_bytes!("../tiled/map1.custom")))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_graphics.system().label("setup_graphics"))
        //Spawning players happens in its own stage since setup_graphics needs to happen first
        .add_startup_stage("setup_game",
        SystemStage::parallel()
            //Players should be draw on on top of objects
            .with_system(draw_map.system().label("draw_map"))
            .with_system(add_players.system().after("draw_map"))
        )
        .add_system(move_player.system().label(MoveReq))
        .add_system(move_objects.system().after(MoveReq))
        .add_system(move_camera.system().after(MoveReq))
        .run();
}

fn setup_graphics(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Skins {
        phase: materials.add(Color::rgb_u8(100, 242, 84).into()),

    });

}

fn add_players(mut commands: Commands, materials: Res<Skins>, asset_server: Res<AssetServer>) {
    for i in 0..=9 {
        commands
            .spawn_bundle(Player::new(i))
            .insert_bundle(Text2dBundle {
                text: Text::with_section(
                    100.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 14.0,
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
            .insert_bundle(SpriteBundle {
                material: materials.phase.clone(),
                sprite: Sprite::new(Vec2::new(10.0, 10.0)),
                ..Default::default()
            });

    }


}

fn draw_map(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, map: Res<Map>) {

    // Set the background color to the map's specified color
    commands.insert_resource(ClearColor((*map).background_color));

    let mut i = 0;

    while i < (*map).objects.len() {
        let map_coords = (*map).objects[i].coords;
        let map_size =  (*map).objects[i].size;
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
                sprite: Sprite::new(Vec2::new(map_size.w, map_size.h)),
                transform: Transform::from_xyz(map_coords.x, map_coords.y, 0.0),
                ..Default::default()
            })
            .insert(map_coords);

        i += 1;
    }
}

// Mobe objects will first validate whether a movement can be done, and if so move them
fn move_objects(mut movements: Query<(&mut Transform, &mut RequestedMovement,  Changed<RequestedMovement>)>, mut map: ResMut<Map>) {
    for (mut coords, mut movement, _) in movements.iter_mut() {
        // Only do any math if a change has been detected, in order to avoid triggering this event without need
        // Only lets you move if the movement doesn't bump into a wall
        if movement.0 != Coords::new(0.0, 0.0) {
            if !map.collision(&Coords::new(coords.translation.x + movement.0.x, coords.translation.y + movement.0.y), &Size::new(15.0, 15.0), 0) {
                coords.translation.x += movement.0.x;
                coords.translation.y += movement.0.y;

            }

            movement.0.x = 0.0;
            movement.0.y = 0.0;

        }
    }
}

fn move_camera(
    mut q: QuerySet<(
        Query<&mut Transform, With<Camera>>,
        Query<(&Transform, &ID, Changed<Transform>)>)
    >) {
    let mut x =  q.q0_mut().single_mut().unwrap().translation.x;
    let mut y =  q.q0_mut().single_mut().unwrap().translation.y;


    for (player, id, _) in q.q1_mut().iter_mut() {
        if id.0 == 0 {
            x = player.translation.x;
            y= player.translation.y;

        }
    }

    q.q0_mut().single_mut().unwrap().translation.x = x;
    q.q0_mut().single_mut().unwrap().translation.y = y;
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut RequestedMovement, &ID)>) {
    let mut x = 0.0;
    let mut y = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        x = -5.0;

    }

    if keyboard_input.pressed(KeyCode::D) {
        x = 5.0;

    }

    if keyboard_input.pressed(KeyCode::S) {
        y = -5.0;

    }

    if keyboard_input.pressed(KeyCode::W) {
        y = 5.0;

    }

    // Only do a change event if a key has been pressed
    if x != 0.0 || y != 0.0 {
        for (mut requested_movement, id) in query.iter_mut() {
            if id.0 == 0 {
                requested_movement.0 = Coords::new(x, y);
                break;

            }
        }
    }
}


