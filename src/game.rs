mod bots;

use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::input::mouse;
use ggez::graphics;
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn tick (mut players: [Player; 8], mut projectiles: &mut Vec<Projectile>, ctx: &mut ggez::Context) -> [Player; 8] {

    // Move every player 
    for player in players.iter_mut() {
        if player.health > 0 {
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
        
        // If a player started reloading in a previous tick, then it continues in this tick
        if player.gun.reloading {
            player.gun.reload()
        }
        
    }
    
    //vec drain_filter isn't stable, so Ima just use a while loop
    // Why not use a for loop? Well because of how Rust borrow checking works, I can't remove an element while using a mutable iterator
    let mut i = 0;
    while i != projectiles.len() {
        // Move every projectile
        match projectiles[i].direction {
            1 => {projectiles[i].y -= projectiles[i].speed;},
            2 => {projectiles[i].y += projectiles[i].speed;},
            3=> {projectiles[i].x += projectiles[i].speed;},
            4 => {projectiles[i].x -= projectiles[i].speed;},
            5 => {projectiles[i].y -= projectiles[i].speed; projectiles[i].x += projectiles[i].speed;},
            6 => {projectiles[i].y -= projectiles[i].speed; projectiles[i].x -= projectiles[i].speed;},
            7 => {projectiles[i].y += projectiles[i].speed; projectiles[i].x += projectiles[i].speed;},
            8 => {projectiles[i].y += projectiles[i].speed; projectiles[i].x -= projectiles[i].speed;},
            0 => {projectiles[i].y -= projectiles[i].speed;},
            _ => {},
            
        }
    
        // Check for a collision
        let mut collided = false;
        
        for player in players.iter_mut() {
            let player_rect = graphics::Rect::new(player.x, player.y, 15.0, 15.0);
            let projectile_rect = graphics::Rect::new(projectiles[i].x, projectiles[i].y, 5.0, 5.0);
            
            // Projectiles can only hit living players
            if collision(player_rect, projectile_rect) && player.health > 0{
                player.health -= projectiles[i].damage;
                println!("Player health: {} ", player.health);
                
                collided = true;
                break;
                
            }
            
        }
        
        // Remove all out of bounds projectiles + projectiles colliding w living players
        if out_of_bounds(projectiles[i].x, projectiles[i].y, 5.0, 5.0) || collided {
            projectiles.remove(i);
            
        } else {
            i += 1;
            
        }
    
    }
              
    check_user_input(&ctx, &mut players, &mut projectiles);
        
    //TODO: Multithreaded bots
    let player2_info = bots::bounce(&players, &projectiles);
    
    players[1].direction = player2_info[0];
    
    if player2_info[1] == 1 {
        players[1].use_ability();
        
    }
    
    if player2_info[2] == 1 {
        players[1].shoot(&mut projectiles);
        
    }
    
    // At the end of processing player movement, return the new player array
    players

}

pub struct Projectile {
    pub x: f32,
    pub y: f32, 
    pub direction: u8,
    pub speed: f32,
    pub damage: u8,
    
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
    ammo_in_mag: u8,
    damage: u8,
    
}

impl Gun {
    pub fn new(model: u8) -> Gun {
        Gun {
            model,
            // The time since the last shot is set as 0 so that you can start shooting as the start of the game
            time_since_last_shot: 0,
            time_since_start_reload: 0,
            reloading: false,
            ammo_in_mag: match model {
                0 => 16,
                _ => 30,
            },
            damage: match model {
                0 => 25,
                _ => 100,
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
                self.ammo_in_mag = 16;
                self.reloading = false;
                
            }
            
        }
       
        
    }
    
    pub fn shoot (&mut self, x: f32, y: f32, direction: u8, projectiles: &mut Vec<Projectile>) {        
        if self.ammo_in_mag > 0 && !self.reloading {
            //Pistol
            if self.model == 0 && current_time() >= self.time_since_last_shot + 250 {
                self.time_since_last_shot = current_time();
                projectiles.push( Projectile {
                    x,
                    y,
                    direction, 
                    speed: 12.0,
                    damage: self.damage,
                    
                });
                
                self.ammo_in_mag -= 1;
                
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
    
    pub health: u8,
}

impl Player {
    pub fn new(color: Option<graphics::Color>, ability: u8, health: u8, gun: u8) -> Player {
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
            health,
            gun: Gun::new(gun),
        }
    }
    
    fn use_ability(&mut self) {
        if self.health > 0 {
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

    }
    
    fn shoot(&mut self, projectiles: &mut Vec<Projectile>) {
        let direction = match self.direction {
            // If the player is staying still, it will shoot north (1)
            0 => 1,
            _ => self.direction,
        };
    
        let x = match self.direction {
            // If the player is looking east or west, it moves where the bullet will spawn so the player doesn't hit it
            3 | 5 => self.x + 30.0,
            4 | 6=> self.x - 15.0,
            _ => self.x + 5.0,
        };
        
        let y = match direction {
            // Same for if it's looking north or south
            1 | 5 | 6 => self.y - 15.0,
            2 | 7 | 8 => self.y + 25.0,
            _ => self.y + 5.0,
        };
        
        if self.health > 0 {
            self.gun.shoot(x, y, direction, projectiles);
        }
        
    }
    
}

fn current_time() -> u128 {
    // Returns the time in Unix Time (the number of seconds since 1970)
    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    
    //Return the current time
    time
}

fn collision (rect1: graphics::Rect, rect2: graphics::Rect) -> bool {
    // A bounding box collision test between two rectangles
    {
        rect1.x < rect2.x + rect2.w &&
        rect1.x + rect1.w > rect2.x &&
        rect1.y < rect2.y + rect2.h &&
        rect1.y + rect1.h > rect2.y
    }
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

// All the user input code is in here, instead of update_game, for readability purposes
fn check_user_input(ctx: &ggez::Context, mut players: &mut [Player; 8], mut projectiles: &mut Vec<Projectile>) {
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
    
    if is_key_pressed(ctx, KeyCode::R) {
        players[0].gun.reload();
        
    }
        
    if mouse::button_pressed(&ctx, mouse::MouseButton::Left) {
        players[0].shoot(&mut projectiles);
        
    }
}
