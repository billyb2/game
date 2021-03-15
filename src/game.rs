use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::input::mouse;
use ggez::graphics;
use rand::{Rng, thread_rng};
use std::time::{Duration, SystemTime};

pub fn update_game (mut players: [Player; 8], ctx: &mut ggez::Context) -> [Player; 8] {

    // Move every player 
    for player in players.iter_mut() {
        match player.direction {
            1 => {player.y -= player.speed;},
            2 => {player.y += player.speed;},
            3=> {player.x += player.speed;},
            4 => {player.x -= player.speed;},
            5 => {player.y -= player.speed; player.x += player.speed;},
            6 => {player.y -= player.speed; player.x -= player.speed;},
            7 => {player.y += player.speed; player.x += player.speed;},
            8 => {player.y += player.speed; player.x -= player.speed;},
            _ => {},
            
        }        
    }

    
    if is_key_pressed(ctx, KeyCode::W) && !is_key_pressed(ctx, KeyCode::S) {
        if is_key_pressed(ctx, KeyCode::D) {
            players[0].direction = 5;
        } else if is_key_pressed(ctx, KeyCode::A) {
            players[0].direction = 6;
        } else {
            players[0].direction = 1;
        }
    } else if is_key_pressed(ctx, KeyCode::S) && !is_key_pressed(ctx, KeyCode::W) {
        if is_key_pressed(ctx, KeyCode::D) {
            players[0].direction = 7;
        } else if is_key_pressed(ctx, KeyCode::A) {
            players[0].direction = 8;
        } else {
            players[0].direction = 2;
        }
    } else if is_key_pressed(ctx, KeyCode::D) && !is_key_pressed(ctx, KeyCode::A) {
        players[0].direction = 3;
    } else if is_key_pressed(ctx, KeyCode::A) && !is_key_pressed(ctx, KeyCode::D) {
        players[0].direction = 4;
    } else {
        players[0].direction = 0;
    }
    
    if is_key_pressed(ctx, KeyCode::E) && players[0].cooldown_finished_time.elapsed().unwrap() >= Duration::from_secs(5) {
    
        if players[0].ability == 0 {
        
            let teleport_distance = 500.0;
        
            match players[0].direction {
                1 => {players[0].y -= teleport_distance;},
                2 => {players[0].y += teleport_distance;},
                3=> {players[0].x += teleport_distance;},
                4 => {players[0].x -= teleport_distance;},
                5 => {players[0].y -= teleport_distance; players[0].x += teleport_distance;},
                6 => {players[0].y -= teleport_distance; players[0].x -= teleport_distance;},
                7 => {players[0].y += teleport_distance; players[0].x += teleport_distance;},
                8 => {players[0].y += teleport_distance; players[0].x -= teleport_distance;},
                _ => {},
            }
            
            players[0].cooldown_finished_time = SystemTime::now();
        
        } else if players[0].ability == 1  {
            players[0].speed = 30.0;
        }

    }
        
    if mouse::button_pressed(&ctx, mouse::MouseButton::Left) {
        //println!("Hello mouse!");
        
    }
    
    // At the end of processing player movement, return the new player array
    players

}

#[derive(Copy, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    // 0 not moving
    // 1 N
    // 2 S
    // 3 E
    // 4 W
    // 5 NE
    // 6 NW
    // 7 SE
    // 8 SW
    
    pub direction: u8,
    pub color: graphics::Color,
    
    // The ability is stored as an int in order to allow for faster code
    // If it was stored as a string, then the players couldn't be stored in an array, causing more variable memory usage
    // 0 is phase
    // 1 is stim
    
    pub ability: u8,
    pub cooldown_finished_time: SystemTime,
    pub speed: f32,
    pub online: bool,
}

impl Player {
    pub fn new(color: Option<graphics::Color>, ability: u8, online: bool) -> Player {
        let mut rng = thread_rng();
    
        Player {
            x: 0.0,
            y: 0.0,
            direction: 0,
            color:match color {
                Some(color) => color,
                None => graphics::Color::from_rgb(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
            },
            ability,
            speed: 10.0,
            cooldown_finished_time: SystemTime::now(),
            online,
        }
    }
}
