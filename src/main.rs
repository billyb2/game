mod game;

use ggez::{event, graphics, nalgebra as na};
use ggez::conf::WindowSetup;
use ggez::conf::NumSamples;
use ggez::timer::check_update_time;

use game::{tick, Player, Projectile};

struct MainState {
    players: [Player; 8],
    projectiles: Vec<Projectile>,
    
}

impl MainState {
    fn new(mut num_of_players: u8) -> ggez::GameResult<MainState> {
        let players: [Player; 8] ={
            let mut players: [Player; 8] = [Player::new(None, 0, 0, 0); 8];
            
            for player in players.iter_mut() {
                if num_of_players > 0 {
                    num_of_players -= 1;
                    *player = Player::new(None, 0, 100, 0);
                    
                } else {
                    break;
                    
                }
            }
            
            players[1].direction = 3;
            
            players
            
        };
    
        let s = MainState {
           players,
           projectiles: Vec::new(),
           
        };
        
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Please see https://docs.rs/ggez/0.5.1/ggez/timer/fn.check_update_time.html for why I'm doing updates like this
        // Basically, the game will run 60 frames every second on average
        
        while check_update_time(ctx, 60) {
            self.players = tick(self.players, &mut self.projectiles, ctx);
        
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [255.0, 255.0, 255.0, 255.0].into());
        
        for player in &self.players {
            if player.health > 0 {
                // Draw each player as a filled rectangle
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
        
        for projectile in &self.projectiles {
            // Draw each player as a filled rectangle
            let rect = graphics::Rect::new(projectile.x, projectile.y, 5.0, 5.0);

            let rect_to_draw = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                graphics::BLACK,
            )?;
            
            graphics::draw(ctx, &rect_to_draw, (na::Point2::new(0.0, 0.0),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let cb = cb.window_setup(
        WindowSetup {
        title: "A game by the beacon boys".to_string(),
        // 8x antialiasing for a block game lol
        samples: NumSamples::Eight,
        // Vsync to make the framerate look pretty
        vsync: true,
        icon: "".to_string(),
        srgb: true,
    });
    
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(2)?;
    event::run(ctx, event_loop, state)
}
    
