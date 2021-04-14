#![allow(unused_assignments)]
use std::f32::consts::PI;

use crate::{Direction, Player, Projectile};

// The first item of the tuple is the direction the player's going to move in
// The second item of the tuple is what direction the player is shooting in, or 0 if they aren't going to shoot
// The third is a bool of whether or not they're shooting to the left
// The fourth is whether or not the player is using its ability (0 or 1)

pub fn bounce(players: &[Player; 20], projectiles: &[Projectile]) -> (Direction, bool, f32, u8) {
    let mut direction = Direction::None;
    
    // Why use an int instead of a bool you may be asking? Well if I ever add more complex functionality to shooting or using your ability, it's less code to refactor. It's also just as efficient as using a bool, since they both use a byte
    let mut use_ability: u8 = 0;
    let mut shooting: f32 = 0.0;
    let mut right = false;

    if players[1].x >= 750.0  && players[1].direction == Direction::W {
        // Move west
        direction = Direction::W;
        
    } else if players[1].x <= 10.0 && players[1].direction == Direction::E {
        // Move east
        direction = Direction::E;
        right = true;
    
    } else {
        direction = players[1].direction;
        
    }
    
    for projectile in projectiles {
        if projectile.x + 50.0 >= players[1].x && projectile.x - 50.0 <= players[1].x || projectile.y + 50.0 >= players[1].y && projectile.y - 50.0 <= players[1].y{
            use_ability = 1;
            shooting = match direction {
                Direction::E => 0.00001,
                Direction::W => PI,
                _ => PI / 2.0,
            };
            
            break;
            
        }
    }
    
    (direction, right, shooting, use_ability)
    
}
