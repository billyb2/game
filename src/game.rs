mod bots;

use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::input::mouse;
use ggez::graphics;
use rand::{Rng, thread_rng};
use std::time::{Duration, SystemTime};

pub fn update_game (mut players: [Player; 8], mut projectiles: &mut Vec<Projectile>, ctx: &mut ggez::Context) -> [Player; 8] {

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
    
    // Move every projectile
    for projectile in projectiles.iter_mut(){
        match projectile.direction {
            1 => {projectile.y -= projectile.speed;},
            2 => {projectile.y += projectile.speed;},
            3=> {projectile.x += projectile.speed;},
            4 => {projectile.x -= projectile.speed;},
            5 => {projectile.y -= projectile.speed; projectile.x += projectile.speed;},
            6 => {projectile.y -= projectile.speed; projectile.x -= projectile.speed;},
            7 => {projectile.y += projectile.speed; projectile.x += projectile.speed;},
            8 => {projectile.y += projectile.speed; projectile.x -= projectile.speed;},
            0 => {projectile.y -= projectile.speed;},
            _ => {},
            
        }
        
    }
        
    // Remove all out of bounds projectiles
    projectiles.retain(|projectile| !out_of_bounds(projectile.x, projectile.y, 5.0, 5.0));

    
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
    
    if is_key_pressed(ctx, KeyCode::E) {
    
        players[0].use_ability();
    }
        
    if mouse::button_pressed(&ctx, mouse::MouseButton::Left) {
        players[0].shoot(&mut projectiles);
        
    }
        
    //TODO: Multithreaded bots
    players[1].direction = bots::move_right(players);
    
    // At the end of processing player movement, return the new player array
    players

}

fn out_of_bounds(x: f32, y: f32, w: f32, h: f32) -> bool {
    if x + w >= 500.0 {
        true
        
    } else if x <= 0.0 {
        true
        
    } else if y + h >= 500.0 {
        true
        
    } else if y <= 0.0 {
        true
        
    } else {
        false
        
    }
}

pub struct Projectile {
    pub x: f32,
    pub y: f32, 
    pub direction: u8,
    pub speed: f32,
    
}

/*struct Weapon {
    shoot_cooldown
    reload_time
    projectile_speed
}*/

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
            x: 100.0,
            y: 100.0,
            direction: 0,
            color:match color {
                Some(color) => color,
                //Random color
                None => graphics::Color::from_rgb(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255)),
            },
            ability,
            speed: 10.0,
            cooldown_finished_time: SystemTime::now(),
            online,
        }
    }
    
    fn use_ability(&mut self) {
        if self.ability == 0 && self.cooldown_finished_time.elapsed().unwrap() >= Duration::from_secs(2) {
        
            let teleport_distance = 250.0;
        
            match self.direction {
                1 => {self.y -= teleport_distance;},
                2 => {self.y += teleport_distance;},
                3=> {self.x += teleport_distance;},
                4 => {self.x -= teleport_distance;},
                5 => {self.y -= teleport_distance; self.x += teleport_distance;},
                6 => {self.y -= teleport_distance; self.x -= teleport_distance;},
                7 => {self.y += teleport_distance; self.x += teleport_distance;},
                8 => {self.y += teleport_distance; self.x -= teleport_distance;},
                _ => {},
            }
            
            self.cooldown_finished_time = SystemTime::now();
        
        } else if self.ability == 1  {
            self.speed = 30.0;
            
        }

    }
    
    fn shoot(&self, projectiles: &mut Vec<Projectile>) {
        projectiles.push(Projectile { x: self.x + 5.0, y: self.y + 5.0, direction: self.direction, speed: 8.0 });
    
    }
}
