use crate::Player;

pub fn bounce(players: &[Player; 8]) -> u8 {

    if players[1].x >= 500.0  && players[1].direction ==3 {
        4
        
    } else if players[1].x <= 0.0 && players[1].direction == 4 {
        3
    
    } else {
        players[1].direction
        
    }
    
}
