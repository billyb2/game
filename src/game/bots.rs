#![allow(unused_assignments)]

use crate::{Player, Projectile};

// The first item of the array is the direction the player's going to move in
// The second is whether the player is shooting or not (0 or 1)
// The third is whether or not the player is using its ability (0 or 1)

pub fn bounce(players: &[Player; 8], projectiles: &[Projectile]) -> [u8; 3] {
    let mut direction: u8 = 0;
    
    // Why use an int instead of a bool you may be asking? Well if I ever add more complex functionality to shooting or using your ability, it's less code to refactor. It's also just as efficient as using a bool, since they both use a byte
    let mut use_ability: u8 = 0;
    let mut shooting: u8 = 0;

    if players[1].x >= 500.0  && players[1].direction ==3 {
        direction = 4;
        
    } else if players[1].x <= 0.0 && players[1].direction == 4 {
        direction = 3;
    
    } else {
        direction = players[1].direction;
        
    }
    
    for projectile in projectiles {
        if projectile.x + 20.0 >= players[1].x || projectile.x - 20.0 <= players[1].x{
            use_ability = 1;
            shooting = 1;
            
            break;
            
        }
    }
    
    [direction, use_ability, shooting]
    
}
