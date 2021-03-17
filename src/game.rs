mod bots;

use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::input::mouse;
use ggez::graphics;
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};

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
        
        // If a player started reloading in a previous tick, then it continues in this tick
        
        if player.gun.reloading {
            player.gun.reload()
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
    players[1].direction = bots::bounce(&players);
    
    // At the end of processing player movement, return the new player array
    players

}

fn out_of_bounds(x: f32, y: f32, w: f32, h: f32) -> bool {
    //Basically, if the rectangle is out of bounds, it returns true, if not it'll return false
    {
        x + w >= 500.0 || 
        x <= 0.0 || 
        y +h >= 500.0 || 
        y <= 0.0
    }

}

pub struct Projectile {
    pub x: f32,
    pub y: f32, 
    pub direction: u8,
    pub speed: f32,
    
}

#[derive(Copy, Clone)]
pub struct Gun {
    // Once again, storing the gun model as an int since it makes it fast and easy to deal with
    // 0 is the pistol
    model: u8,
    // This time is stored so that the bullets per second of guns can be limited dynamically
    time_since_last_shot: u128,
    time_since_start_reload: u128,
    reloading: bool,
    ammo_count: u8,
}

impl Gun {
    pub fn new(model: u8) -> Gun {
        Gun {
            model: model,
            // The time since the last shot is set as 0 so that you can start shooting as the start of the game
            time_since_last_shot: 0,
            time_since_start_reload: 0,
            reloading: false,
            ammo_count: match model {
                0 => 16,
                _ => 30,
            },
        }
    
    }

    pub fn reload (&mut self) {
        if !self.reloading {
            // Start reloading
            self.time_since_start_reload = current_time();
            self.reloading = true;
            
        } else {
            // Pistol has a reload time of 2 seconds
            if self.model == 0  && self.time_since_start_reload + 2000 <= current_time() {
                self.ammo_count = 16;
                self.reloading = false;
                
            }
            
        }
       
        
    }
    
    pub fn shoot (&mut self, x: f32, y: f32, direction: u8, projectiles: &mut Vec<Projectile>) {        
        if self.ammo_count > 0 {
            //Pistol
            //println!("Current time: {}\n Time since last shot: {}", current_time(), self.time_since_last_shot);
            if self.model == 0 && current_time() >= self.time_since_last_shot + 250 {
                self.time_since_last_shot = current_time();
                projectiles.push( Projectile {
                    x,
                    y,
                    direction, 
                    speed: 8.0,
                });
                
                self.ammo_count -= 1;
                
            }
            
        } else {
            self.reload();
            
        }
    }
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
    pub cooldown_finished_time: u128,
    pub speed: f32,
    
    pub gun: Gun,
    
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
            cooldown_finished_time: current_time(),
            online,
            gun: Gun::new(0),
        }
    }
    
    fn use_ability(&mut self) {
        if self.ability == 0 && current_time() >= self.cooldown_finished_time + 2000 {
        
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
            
            self.cooldown_finished_time = current_time();
        
        } else if self.ability == 1  {
            self.speed = 30.0;
            
        }

    }
    
    fn shoot(&mut self, projectiles: &mut Vec<Projectile>) {
        self.gun.shoot(self.x, self.y, self.direction, projectiles);
        
    }
    
}

fn current_time() -> u128 {
    // Returns the time in Unix Time (the number of seconds since 1970)
    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    
    //Return the current time
    time
}
