pub mod bots;
pub mod map;
pub mod objects;

use map::Map;
use std::f32::consts::PI;
use objects::{Ability, Direction, Player, Projectile, ProjectileType, out_of_bounds, Point2, Rect};

//mouse_pressed index are: 0: left mouse pressed, 1: middle mouse pressed, 2: right mouse pressed
pub fn tick (mut players: [Player; 20], mut projectiles: &mut Vec<Projectile>, map: &mut Map, keys_pressed: Vec<char>, mouse_pressed: [bool; 3], mouse_coords: Point2, screen_coords: Rect) -> [Player; 20] {
    // Move every player 
    for player in players.iter_mut() {
        if player.health > 0 {
            match player.direction {
                Direction::N => {
                        if !out_of_bounds(player.x, player.y - player.speed, 15.0, 15.0, map.width, map.height) && 
                            !map.collision(&Rect::new(player.x, player.y - player.speed, 15.0, 15.0), 0) {
                            player.y -= player.speed; 
                            
                        }
                    },
                
                Direction::S => {
                        if !out_of_bounds(player.x, player.y + player.speed, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x, player.y + player.speed, 15.0, 15.0), 0) {
                            player.y += player.speed; 
                        
                        }
                    },
                
                Direction::E => {
                        if !out_of_bounds(player.x + player.speed, player.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x + player.speed, player.y, 15.0, 15.0), 0) {
                            player.x += player.speed; 
                            
                        }
                    },
                
                Direction::W => {
                        if !out_of_bounds(player.x - player.speed, player.y, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(player.x - player.speed, player.y, 15.0, 15.0), 0){
                            player.x -= player.speed; 
                            
                            
                        }
                    },
                
                Direction::NE => {
                        if !out_of_bounds(player.x + player.speed, player.y - player.speed, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(player.x + player.speed, player.y - player.speed, 15.0, 15.0), 0){
                            player.y -= player.speed; player.x += player.speed; 
                            
                        }
                    },
                
                Direction::NW => {
                        if !out_of_bounds(player.x - player.speed , player.y - player.speed, 15.0, 15.0, map.width, map.height)  &&  !map.collision(&Rect::new(player.x - player.speed, player.y - player.speed, 15.0, 15.0), 0) {
                            player.x -= player.speed; 
                            player.y -= player.speed; 
                            
                        }
                    },
                
                Direction::SE => {
                        if !out_of_bounds(player.x + player.speed, player.y + player.speed, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(player.x + player.speed, player.y + player.speed, 15.0, 15.0), 0) {
                            player.x += player.speed;
                            player.y += player.speed; 
                            
                        }
                    },
                
                Direction::SW => {
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
                if player.ability == Ability::Stim && (player.speed - 20.0).abs() < f32::EPSILON {
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
        if projectiles[i].projectile_type == ProjectileType::Speedball {
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
              
    check_user_input(&mut players, &mut projectiles, map, keys_pressed, mouse_pressed, mouse_coords, screen_coords);
        
    //TODO: Multithreaded bots
    // Basically, a bot is given information on every player, info on the projectiles, and info on how the map looks (not added yet), and it outputs 4 things.
    // The first is the direction that it wants to move in
    // The second and third is whether or not it wants to shoot, and what angle it'll shoot at (if it gives an angle of 0, it won't shoot).
    // The fourth is whether or not the bot wants to use its ability
    
    let player2_info = bots::bounce(&players, &projectiles);
    
    players[1].direction = Direction::None;
    
    if player2_info.2 != 0.0 {
        players[1].shoot(player2_info.1, player2_info.2, &mut projectiles);
        
    }
    
    if player2_info.3 == 1 {
        players[1].use_ability(map);
        
    }
    
    // At the end of processing player movement, return the new player array
    players

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

// All the user input code is in here, instead of the tick fn, for readability purposes
fn check_user_input(mut players: &mut [Player; 20], mut projectiles: &mut Vec<Projectile>, map: &mut Map, keys: Vec<char>, mouse_pressed: [bool; 3], mouse_coords: Point2, screen_coords: Rect) {
    if is_key_pressed('w', &keys) && !is_key_pressed('s', &keys) {
        if is_key_pressed('d', &keys) {
            players[0].direction = Direction::NE;
        } else if is_key_pressed('a', &keys) {
            players[0].direction = Direction::NW;
        } else {
            players[0].direction = Direction::N;
        }
    } else if is_key_pressed('s', &keys) && !is_key_pressed('w', &keys) {
        if is_key_pressed('d', &keys) {
            players[0].direction = Direction::SE;
        } else if is_key_pressed('a', &keys) {
            players[0].direction = Direction::SW;
        } else {
            players[0].direction = Direction::S;
        }
    } else if is_key_pressed('d', &keys) && !is_key_pressed('a', &keys) {
        players[0].direction = Direction::E;
    } else if is_key_pressed('a', &keys) && !is_key_pressed('d', &keys) {
        players[0].direction = Direction::W;
    } else {
        players[0].direction = Direction::None;
    }
    
    if is_key_pressed('e', &keys) {
        players[0].use_ability(map);
        
    }
    
    if is_key_pressed('r', &keys) {
        players[0].gun.reload();
        
    }
        
    if mouse_pressed[0] {
        // Because of trig stuff, you need to know whether the bullet is going to move right or left as well as what angle
        let player_x = match players[0].x - screen_coords.w / 2.0 < 0.0 {
            true => players[0].x,
            false => screen_coords.w / 2.0,

        };

        let player_y = match players[0].y - screen_coords.h / 2.0 < 0.0 {
            true => players[0].y,
            false => screen_coords.h / 2.0,
        };
        
        let rad = get_angle(player_x + 7.5, player_y + 7.5, mouse_coords.x,  mouse_coords.y);
        let right = { mouse_coords.x > player_x };
    
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

fn is_key_pressed(key: char, keys_pressed: &[char]) -> bool {
     keys_pressed.contains(&key)

}
