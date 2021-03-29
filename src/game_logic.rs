use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::input::mouse;
use ggez::graphics::{Color, Rect, screen_coordinates};
use crate::game_libs::map::{Map, MapObject};
use crate::game_libs::bots;
use rand::{Rng, thread_rng};
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn tick (mut players: [Player; 8], mut projectiles: &mut Vec<Projectile>, map: &mut Map, ctx: &mut ggez::Context) -> [Player; 8] {
    // Move every player 
    for player in players.iter_mut() {
        if player.health > 0 {
            match player.direction {
                1 => {
                        if !out_of_bounds(player.x, player.y - player.speed, 15.0, 15.0, map.width, map.height) && 
                            !map.collision(&Rect::new(player.x, player.y - player.speed, 15.0, 15.0), 0) {
                            player.y -= player.speed; 
                            
                        }
                    },
                
                2 => {
                        if !out_of_bounds(player.x, player.y + player.speed, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x, player.y + player.speed, 15.0, 15.0), 0) {
                            player.y += player.speed; 
                        
                        }
                    },
                
                3=> {
                        if !out_of_bounds(player.x + player.speed, player.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x + player.speed, player.y, 15.0, 15.0), 0) {
                            player.x += player.speed; 
                            
                        }
                    },
                
                4 => {
                        if !out_of_bounds(player.x - player.speed, player.y, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(player.x - player.speed, player.y, 15.0, 15.0), 0){
                            player.x -= player.speed; 
                            
                            
                        }
                    },
                
                5 => {
                        if !out_of_bounds(player.x + player.speed, player.y - player.speed, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(player.x + player.speed, player.y - player.speed, 15.0, 15.0), 0){
                            player.y -= player.speed; player.x += player.speed; 
                            
                        }
                    },
                
                6 => {
                        if !out_of_bounds(player.x - player.speed , player.y - player.speed, 15.0, 15.0, map.width, map.height)  &&  !map.collision(&Rect::new(player.x - player.speed, player.y - player.speed, 15.0, 15.0), 0) {
                            player.x -= player.speed; 
                            player.y -= player.speed; 
                            
                        }
                    },
                
                7 => {
                        if !out_of_bounds(player.x + player.speed, player.y + player.speed, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x + player.speed, player.y + player.speed, 15.0, 15.0), 0) {
                            player.x += player.speed;
                            player.y += player.speed; 
                            
                        }
                    },
                
                8 => {
                        if !out_of_bounds(player.x - player.speed, player.y + player.speed, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x - player.speed, player.y + player.speed, 15.0, 15.0), 0) {
                            player.x -= player.speed; 
                            player.y += player.speed; 
                    
                    }
                },
                
                _ => {},
                
            }
            
            // A players ability charge increases every tick (60 ticks per second on average)
            if player.ability_charge < player.max_ability_charge {
                // The second requirement for the if block is basically && player.speed == 20.0, but it has no chance of messing up because of floating point rounding
                if player.ability == 1 && (player.speed - 20.0).abs() < f32::EPSILON {
                    if player.ability_charge > 0 {
                    
                        player.ability_charge -= 1;
                        
                    } else {
                        player.speed = 10.0;
                        
                    }
                
                } else {
                    player.ability_charge += 1;
                    
                }
                
            }

        }
        
        // If a player started reloading in a previous tick, then it continues in this tick
        if player.gun.reloading {
            player.gun.reload();
            
        }
        
        // Likewise, if a player was shooting in the previous tick (which only happens with guns with special ways of shooting, like the burst rifle), then it continues firing (in the case of the burst rifle, it will shoot the next bullet of the burst).
        if player.gun.shooting.is_some() {
            let args = player.gun.shooting.unwrap();
            player.gun.shoot(args.0, args.1, args.2, args.3, player.ability, projectiles);
            
        }
        
    }
    
    //vec drain_filter isn't stable, so Ima just use a while loop
    // Why not use a for loop? Well because of how Rust borrow checking works, I can't remove an element while using a mutable iterator
    let mut i = 0;
    while i != projectiles.len() {
        // Move every projectile
        if projectiles[i].right {
            projectiles[i].x += projectiles[i].angle.cos() * projectiles[i].speed;
            projectiles[i].y += projectiles[i].angle.sin() * projectiles[i].speed;
        
        } else {
            projectiles[i].x -= projectiles[i].angle.cos() * projectiles[i].speed;
            projectiles[i].y -= projectiles[i].angle.sin() * projectiles[i].speed;
            
        }
        
        // The speedball projectile starts off slow, but increases its size and speed exponentially
        if projectiles[i].projectile_type == 1 {
            projectiles[i].speed *= 1.1;
            projectiles[i].w *= 1.03;
            projectiles[i].h *= 1.03;
            
            // The speedball's damage increases over the distance traveled (up to 75)
            if projectiles[i].damage <= 75 {
                projectiles[i].damage += (projectiles[i].distance_traveled / 60.0 ) as u8;
                
            }
            
        }
        
        // Each projectile keeps track of how far its traveled, so that it will delete itself after a certain distance
        projectiles[i].distance_traveled += projectiles[i].speed;
        
        let projectile_rect = Rect::new(projectiles[i].x, projectiles[i].y, projectiles[i].w, projectiles[i].h);
        let fired_from = projectiles[i].fired_from;
        
    
        // Check for a player-projectile collision
        let mut player_collision = false;
        // Bullet has reached its maximum distance
        let mut max_distance_reached = false;
        
        if projectiles[i].distance_traveled > projectiles[i].max_distance {
            max_distance_reached = true;
            
        }
        
        // Projectile collisions with player
        for player in players.iter_mut() {
            let player_rect = Rect::new(player.x, player.y, 15.0, 15.0);
            
            // Projectiles can only hit living players
            if (collision(&player_rect, &projectile_rect) && player.health > 0) && player.gun.fired_from != fired_from {
                if player.health as i8 - projectiles[i].damage as i8 > 0 {
                    player.health -= projectiles[i].damage;
                    
                } else {
                    player.health = 0;
                    
                }
                println!("Player health: {} ", player.health);
                
                //The player's color slowly fades as they take more damage
                let mut color_tuple = player.color.to_rgba();
                color_tuple.3 = ((player.health as f64 / 100.0) * 255.0) as u8;
                
                player.color = color_tuple.into();
                
                player_collision = true;
                break;
                
            }
            
        }
        
        
        // Remove all out of bounds projectiles + projectiles colliding w living players/ other projectiles
        if out_of_bounds(projectiles[i].x, projectiles[i].y, projectiles[i].w, projectiles[i].h, map.width, map.height) || player_collision || map.collision(&projectile_rect, projectiles[i].damage as u16) || max_distance_reached {
            projectiles.remove(i);
            
        } else {
            i += 1;
            
        }
    
    }
              
    check_user_input(&ctx, &mut players, &mut projectiles, map);
        
    //TODO: Multithreaded bots
    // Basically, a bot is given information on every player, info on the projectiles, and info on how the map looks, and it outputs 4 things. 
    // The first is the direction that it wants to move in
    // The second and third is whether or not it wants to shoot, and what angle it'll shoot at (if it gives an angle of 0, it won't shoot).
    // The fourth is whether or not the bot wants to use its ability
    
    let player2_info = bots::bounce(&players, &projectiles);
    
    players[1].direction = player2_info.0;
    
    if player2_info.2 != 0.0 {
        players[1].shoot(player2_info.1, player2_info.2, &mut projectiles);
        
    }
    
    if player2_info.3 == 1 {
        players[1].use_ability(map);
        
    }
    
    // At the end of processing player movement, return the new player array
    players

}

pub struct Projectile {
    pub x: f32,
    pub y: f32, 
    pub w: f32,
    pub h: f32,
    right: bool,
    angle: f32,
    speed: f32,
    damage: u8,
    // 0 is just a regular bullet
    // 1 is a bullet that speeds up over time
    fired_from: u8, // Fixes a game-breaking bug where the bots get killed by their own bullets
    projectile_type: u8,
    distance_traveled: f32,
    max_distance: f32,
    
}

#[derive(Copy, Clone)]
pub struct Gun {
    // Once again, storing the gun model as an int since it makes it fast and easy to deal with
    // 0 is the pistol
    // 1 is the shotgun
    // 2 is the speedball
    // 3 is the burst rifle
    // 4 is the assault rifle
    pub model: u8,
    // This time is stored so that the bullets per second of guns can be limited dynamically
    time_since_last_shot: u128,
    time_since_start_reload: u128,
    // Shooting's,arguments are the arguments it had previously from the last frame, used for guns that don't just shoot one bullet at a time (like the burst rifle)
    shooting: Option<(f32, f32, bool, f32)>,
    projectiles_fired: u8,
    pub reloading: bool,
    // Reload time is in miliseconds
    reload_time: u16,
    pub ammo_in_mag: u8,
    pub max_ammo: u8,
    damage: u8,
    pub fired_from: u8,
    max_distance: f32,
    
}

impl Gun {
    pub fn new(model: u8, ability: u8, player_fired_from: u8) -> Gun {
        let mut gun = Gun {
            model,
            // The time since the last shot is set as 0 so that you can start shooting as the start of the game
            time_since_last_shot: 0,
            time_since_start_reload: 0,
            reloading: false,
            reload_time: match model {
                0 => 2000,
                1 => 5000,
                2 => 3000,
                3 => 3250,
                4 => 3750,
                _ => 3000,
            },
            // Some guns have special shooting behaviors that last over the course of mutliple ticks, which shooting and projectiles_fired take advantage of
            shooting: None,
            projectiles_fired: 0,
            ammo_in_mag: match model {
                0 => 16,
                1 => 8,
                2 => 6,
                3 => 21,
                4 => 25,
                _ => 30,

            },
            max_ammo: match model {
                0 => 16,
                1 => 8,
                2 => 6,
                3 => 21,
                4 => 25,
                _ => 30,

            },
            damage: match model {
                0 => 45,
                1 => 25,
                2 => 1,
                3 => 13,
                4 => 15,
                _ => 100,
            },
            fired_from: player_fired_from,
            max_distance: match model {
                0 => 900.0,
                1 => 300.0,
                2 => 3000.0,
                3 => 1000.0,
                4 => 1000.0,
                _ => 900.0,
            }

        };

        // The engineer ability can reload in half the time
        if ability == 3 {
            gun.reload_time /= 2;

        }

        gun
    
    }

    pub fn reload (&mut self) {
        if !self.reloading {
            // Start reloading
            self.time_since_start_reload = current_time();
            self.reloading = true;
            
        } else if self.time_since_start_reload + self.reload_time as u128 <= current_time() {
            self.ammo_in_mag = self.max_ammo;
            self.reloading = false;
            
        }
       
        
    }
    
    pub fn shoot (&mut self, x: f32, y: f32, right: bool, angle: f32, ability: u8, projectiles: &mut Vec<Projectile>) {
        if self.ammo_in_mag > 0 && !self.reloading {
            //Pistol
            if self.model == 0 && current_time() >= self.time_since_last_shot + 500 {
                self.time_since_last_shot = current_time();
                
                projectiles.push( Projectile {
                    x: match right {
                        true => x + (angle.cos() * 25.0),
                        false => x - (angle.cos() * 15.0),
                    },
                    y: match right {
                        true => y + (angle.sin() * 25.0),
                        false => y - (angle.sin() * 5.0),
                    },
                    w: 5.0,
                    h: 5.0,
                    right,
                    angle, 
                    speed: match ability {
                        3 => 15.0,
                        _ => 12.0,
                    },
                    damage: self.damage,
                    fired_from: self.fired_from,
                    projectile_type: 0,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,
                    
                });
                
                self.ammo_in_mag -= 1;
                
            } else if self.model == 1 && current_time() >= self.time_since_last_shot + 1500 {
                let mut rng = thread_rng();
                let recoil_range: f32 = 0.2;
                            
                self.time_since_last_shot = current_time();
                
                let mut shoot_several_bullets = |mut num_of_bullets: u8| {
                    while num_of_bullets > 0 {
                        num_of_bullets -= 1;
                    
                        projectiles.push( 
                            Projectile {
                                x: match right {
                                    true => x + (angle.cos() * 25.0 ) as f32,
                                    false => x - (angle.cos() * 15.0) as f32,
                                    
                                },
                                y: match right {
                                    true => y + (angle.sin() * 25.0) as f32,
                                    false => y - (angle.sin() * 15.0) as f32,
                                    
                                },
                                w: 5.0,
                                h: 5.0,
                                right,
                                angle: angle + rng.gen_range(-recoil_range..recoil_range), 
                                speed: match ability {
                                    3 => 13.75,
                                    _ => 11.0,
                                },
                                projectile_type: 0,
                                damage: self.damage,
                                fired_from: self.fired_from,
                                distance_traveled: 0.0,
                                max_distance: self.max_distance,
                                
                            }
                        );
                    
                    }
                };
                
                shoot_several_bullets(12);
                
                self.ammo_in_mag -= 1;
                
            } else if self.model == 2 && current_time() >= self.time_since_last_shot + 1500 {
                self.time_since_last_shot = current_time();
                
                projectiles.push( Projectile {
                    x: match right {
                        true => x + (angle.cos() * 25.0) as f32,
                        false => x - (angle.cos() * 15.0) as f32,
                    },
                    y: match right {
                        true => y + (angle.sin() * 25.0) as f32,
                        false => y - (angle.sin() * 15.0) as f32,
                    },
                    w: match ability {
                        3 => 6.25,
                        _ => 5.0,
                    },
                    h: match ability {
                        3 => 6.25,
                        _ => 5.0,
                    },
                    right,
                    angle, 
                    speed: match ability {
                        3 => 0.31,
                        _ => 0.25,
                    },
                    projectile_type: 1,
                    damage: self.damage,
                    fired_from: self.fired_from,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,
                    
                });
                
                self.ammo_in_mag -= 1;
            } else if self.model == 3 {
                if self.shooting.is_some() {
                    if current_time() >= self.time_since_last_shot + 50 {
                        self.time_since_last_shot = current_time();
                    
                        if self.projectiles_fired != 3 {
                            self.projectiles_fired += 1;
                            projectiles.push( Projectile {
                                x: match right {
                                    true => x + (angle.cos() * 25.0) as f32,
                                    false => x - (angle.cos() * 15.0) as f32,
                                },
                                y: match right {
                                    true => y + (angle.sin() * 25.0) as f32,
                                    false => y - (angle.sin() * 15.0) as f32,
                                },
                                w: 5.0,
                                h: 5.0,
                                right,
                                angle, 
                                speed: match ability {
                                    3 => 15.0,
                                    _ => 12.0,
                                },
                                projectile_type: 0,
                                damage: self.damage,
                                fired_from: self.fired_from,
                                distance_traveled: 0.0,
                                max_distance: self.max_distance,
                                
                            });
                            self.ammo_in_mag -= 1;
                            
                        
                        } else {
                            self.projectiles_fired = 0;
                            self.shooting = None;
                        
                        }
                        
                    }
                    
                
                } else if current_time() >= self.time_since_last_shot + 500  {
                    self.time_since_last_shot = current_time();
            
                    self.shooting = Some((x, y, right, angle));
                    self.projectiles_fired += 1;
                    
                    projectiles.push( Projectile {
                        x: match right {
                            true => x + (angle.cos() * 25.0) as f32,
                            false => x - (angle.cos() * 15.0) as f32,
                        },
                        y: match right {
                            true => y + (angle.sin() * 25.0) as f32,
                            false => y - (angle.sin() * 15.0) as f32,
                        },
                        w: 5.0,
                        h: 5.0,
                        right,
                        angle, 
                        speed: match ability {
                            3 => 15.0,
                            _ => 12.0,
                        },
                        projectile_type: 0,
                        fired_from: self.fired_from,
                        damage: self.damage,
                        distance_traveled: 0.0,
                        max_distance: self.max_distance,
                        
                    });
                    
                    self.ammo_in_mag -= 1;
                
                }
                    
            } else if self.model == 4 && current_time() >= self.time_since_last_shot + 80 {
                self.time_since_last_shot = current_time();
                
                projectiles.push( Projectile {
                    x: match right {
                        true => x + (angle.cos() * 25.0),
                        false => x - (angle.cos() * 15.0),
                    },
                    y: match right {
                        true => y + (angle.sin() * 25.0),
                        false => y - (angle.sin() * 5.0),
                    },
                    w: 5.0,
                    h: 5.0,
                    right,
                    angle, 
                    speed: match ability {
                        3 => 10.0,
                        _ => 8.0,
                    },
                    damage: self.damage,
                    fired_from: self.fired_from,
                    projectile_type: 0,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,
                    
                });
                
                self.ammo_in_mag -= 1;
        
            } 
            
        } else {
            // Reload if no ammo is available
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
    pub color: Color,
    
    // The ability is stored as an int in order to allow for faster code
    // If it was stored as a string, then the players couldn't be stored in an array, causing more variable memory usage
    // 0 is phase
    // 1 is stim
    // 2 is the wall
    // 3 is the engineer (should probably be default)
    pub ability: u8,
    // Your ability charges every tick, and then when it hits its minimum threshold you can use it, though waiting until it hits its maximum threshold may be better, as it will increase the ability's power/duration/whatever.
    // For example, the stim ability will run longer then longer you wait for its ability to charge
    pub ability_charge: u16,
    min_ability_charge: u16,
    pub max_ability_charge: u16,
    
    
    speed: f32,
    
    pub gun: Gun,
    
    pub health: u8,
}

impl Player {
    pub fn new(color: Option<Color>, ability: u8, health: u8, gun: u8, player_id: u8) -> Player {
        let mut rng = thread_rng();
            
        Player {
            x: 596.0,
            y: 342.0,
            direction: 0,
            color:match color {
                Some(color) => color,
                //Random color
                None => Color::from_rgba(rng.gen_range(100..255), rng.gen_range(100..255), rng.gen_range(100..255), 255),
            },
            ability,
            ability_charge: match ability {
                0 => 150,
                1 => 150,
                2 => 150,
                3 => 1,
                _ => 150,
            },
            min_ability_charge: match ability {
                // There's on average, 60 ticks per second, so 2.5 seconds need to pass to have enough charge to use your ability
                0 => 150,
                1 => 1,
                2 => 150,
                3 => 1,
                _ => 150,
            },
            max_ability_charge: match ability {
                0 => 300,
                1 => 300,
                2 => 150,
                3 => 1,
                _ => 300,
            },
            speed: 10.0,
            health,
            gun: Gun::new(gun, ability, player_id),
        }
    }
    
    fn use_ability(&mut self, map: &mut Map) {
        if self.health > 0 && self.ability_charge >= self.min_ability_charge{
            if self.ability == 0  {
            
                let teleport_distance = 250.0;
            
                //I know this is ugly, it just lets a player move if it's movement wouldn't put it out of bounds
                match self.direction {
                    1 => {
                        if !out_of_bounds(self.x, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x, self.y - teleport_distance, 15.0, 15.0), 0) {
                            self.y -= teleport_distance;
                            
                        }
                    },
                    2 => {
                        if !out_of_bounds(self.x, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x, self.y + teleport_distance, 15.0, 15.0), 0){
                            self.y += teleport_distance;
                            
                        }
                    },
                    3=> {
                        if !out_of_bounds(self.x + teleport_distance, self.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y, 15.0, 15.0), 0){
                            self.x += teleport_distance;
                        
                        }
                    },
                    4 => {
                        if !out_of_bounds(self.x - teleport_distance, self.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x - teleport_distance, self.y, 15.0, 15.0), 0){
                            self.x -= teleport_distance;
                            
                        }
                    },
                    5 => {
                        if !out_of_bounds(self.x + teleport_distance, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y - teleport_distance, 15.0, 15.0), 0){
                            self.x += teleport_distance;
                            self.y -= teleport_distance; 
                            
                        }
                    },
                    6 => {
                        if !out_of_bounds(self.x - teleport_distance, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x - teleport_distance, self.y - teleport_distance, 15.0, 15.0), 0) {
                            self.x -= teleport_distance;
                            self.y -= teleport_distance; 
                            
                        }
                    },
                    7 => {
                        if !out_of_bounds(self.x + teleport_distance, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y + teleport_distance, 15.0, 15.0), 0) {
                            self.x += teleport_distance;
                            self.y += teleport_distance; 
                            
                        }
                    },
                    8 => {
                        if !out_of_bounds(self.x - teleport_distance, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(self.x - teleport_distance, self.y + teleport_distance, 15.0, 15.0), 0) {
                            self.x -= teleport_distance;
                            self.y += teleport_distance;
                            
                        }
                    },
                    _ => {},
                }
                
                self.ability_charge -= 150;
                            
            } else if self.ability == 1  {
                self.speed = 20.0;
                
                self.ability_charge -= 1;
                
            } else if self.ability == 2 {
                let x = match self.direction {
                    3 | 5 | 7 => self.x + 25.0,
                    4 | 6 | 8=> self.x - 25.0,
                    _ => self.x,

                };

                let y = match self.direction {
                    1 | 5 | 6=> self.y - 25.0,
                    2 | 7 | 8 => self.y + 25.0,
                    0 => self.y - 25.0,
                    _ => self.y,

                };

                let w = match self.direction {
                    1 | 5 | 6 | 2 | 7 | 8 => 40.0,
                    _ => 20.0,

                };

                // Can't compare floats since the compiler complains, so I convert the width to a u8
                //See https://github.com/rust-lang/rust/issues/41620
                let h = match w as u8{
                    40 => 20.0,
                    _ => match self.direction {
                        3 | 5 | 7 | 4 | 6 | 8 => 40.0,
                        _ => 20.0,
                    }
                };

                let color = Color::from_rgb(0, 255, 0);

                map.objects.push(MapObject::new(Rect::new(x, y, w, h), color, Some(100)));

                self.ability_charge -= 150;
            }
        }

    }
    
    fn shoot(&mut self, right: bool, angle: f32, projectiles: &mut Vec<Projectile>) {
        
        if self.health > 0 {
            self.gun.shoot(self.x, self.y, right, angle, self.ability, projectiles);
            
        }
        
    }
    
}

fn current_time() -> u128 {
    // Returns the time in Unix Time (the number of milliseconds since 1970)
    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    
    //Return the current time
    time
}

pub fn collision (rect1: &Rect, rect2: &Rect) -> bool {
    // A bounding box collision test between two rectangles
    {
        rect1.x < rect2.x + rect2.w &&
        rect1.x + rect1.w > rect2.x &&
        rect1.y < rect2.y + rect2.h &&
        rect1.y + rect1.h > rect2.y
    }
}

fn out_of_bounds(x: f32, y: f32, w: f32, h: f32, world_width: f32, world_height: f32,) -> bool {
    //Basically, if the rectangle is out of bounds, it returns true, if not it'll return false    
    {
        x + w >= world_width || 
        x <= 0.0 || 
        y +h >= world_height || 
        y <= 0.0
    }

}

// All the user input code is in here, instead of the tick fn, for readability purposes
fn check_user_input(ctx: &ggez::Context, mut players: &mut [Player; 8], mut projectiles: &mut Vec<Projectile>, map: &mut Map) {
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
        players[0].use_ability(map);
        
    }
    
    if is_key_pressed(ctx, KeyCode::R) {
        players[0].gun.reload();
        
    }
        
    if mouse::button_pressed(&ctx, mouse::MouseButton::Left) {
        let screen_coords = screen_coordinates(&ctx);
        // Because of trig stuff, you need to know whether the bullet is going to move right or left as well as what angle
        let player_x = if players[0].x - screen_coords.w / 2.0 < 0.0 {
            players[0].x
            
        } else {
            screen_coords.w / 2.0
            
        };
        
        let player_y = if players[0].y - screen_coords.h / 2.0 < 0.0 {
            players[0].y
            
        } else {
            screen_coords.h / 2.0
            
        };
        
        let rad = get_angle(player_x + 7.5, player_y + 7.5, mouse::position(&ctx).x,  mouse::position(&ctx).y);
        let right = { mouse::position(&ctx).x > player_x };
    
        players[0].shoot(right, rad, &mut projectiles);
        
    }
    
}

fn get_angle(cx: f32, cy: f32, ex: f32, ey: f32) -> f32 {
    let dy = ey - cy;
    let dx = ex - cx;

    if dx != 0.0 {
        let d = dy / dx;

        // Returns the angle in radians
        d.atan()

    } else if dy > 0.0 {
            PI / 2.0
        
    }  else {
            PI
    
    }
}