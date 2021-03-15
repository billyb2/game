mod game;

use ggez::{event, graphics, nalgebra as na};
use ggez::conf::WindowSetup;
use ggez::timer::check_update_time;

use game::{update_game, Player};

struct MainState {
    players: [Player; 8],
}

impl MainState {
    fn new(mut num_of_players: u8) -> ggez::GameResult<MainState> {
        let players: [Player; 8] ={
            let mut players: [Player; 8] = [Player::new(None, 0, false); 8];
            
            for player in players.iter_mut() {
                if num_of_players > 0 {
                    num_of_players -= 1;
                    *player = Player::new(None, 0, true);
                    
                } else {
                    *player = Player::new(None, 0, false);
                }
            }
            
            players
            
        };
    
        let s = MainState {
           players, 
        };
        
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Please see https://docs.rs/ggez/0.5.1/ggez/timer/fn.check_update_time.html for why I'm doing
        // updates like this
        // Basically, the game will run 60 frames every second on average
        while check_update_time(ctx, 60) {
            self.players = update_game(self.players, ctx);
        
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [255.0, 255.0, 255.0, 255.0].into());
        
        
        for player in &self.players {
            if player.online {
                let rect = graphics::Rect::new(player.x, player.y, 15.0, 15.0);

                let rect_to_draw = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    player.color,
                )?;
                
                graphics::draw(ctx, &rect_to_draw, (na::Point2::new(0.0, 0.0),))?;
            
            }
        
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let cb = cb.window_setup(WindowSetup::default());
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(2)?;
    event::run(ctx, event_loop, state)
}
    
