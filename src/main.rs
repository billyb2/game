mod game;

use ggez::{event, graphics, nalgebra as na};
use ggez::conf::{NumSamples, WindowSetup};
use ggez::timer::check_update_time;

use game::{tick, Player, Projectile};

pub const WORLD_WIDTH: f32 = 10_000.0;
pub const WORLD_HEIGHT: f32 = 10_000.0;

struct Point {
    x: f32,
    y: f32,
}

struct MainState {
    players: [Player; 8],
    projectiles: Vec<Projectile>,
    origin: Point,
    zoom: f32,
    
}

impl MainState {
    fn new(mut num_of_players: u8) -> ggez::GameResult<MainState> {
        let players: [Player; 8] ={
            let mut players: [Player; 8] = [Player::new(None, 0, 0, 0); 8];
            
            for player in players.iter_mut() {
                if num_of_players > 0 {
                    num_of_players -= 1;
                    *player = Player::new(None, 0, 100, 1);
                    
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
           origin: Point {x: 0.0, y: 0.0},
           zoom: 1.0,
           
        };
        
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Please see https://docs.rs/ggez/0.5.1/ggez/timer/fn.check_update_time.html for why I'm doing updates like this
        // Basically, the game will run 60 frames every second on average
        
        while check_update_time(ctx, 60) {
            let data = tick(self.players, &mut self.projectiles, ctx);
            self.players = data.0;
            
            self.origin.x += data.1[0];
            self.origin.y += data.1[1];
            
            /*if self.origin.x < 0.0 {
                self.origin.x = 0.0;
                
            } else if self.origin.x > WORLD_WIDTH {
                self.origin.x = WORLD_WIDTH;
                
            }
            if self.origin.y < 0.0 {
                self.origin.y = 0.0;
                
            } else if self.origin.y > WORLD_HEIGHT {
                self.origin.y = WORLD_HEIGHT;
                
            }*/
        
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::BLACK);
        
        let screen_coords = graphics::screen_coordinates(&ctx);
        
        for player in &self.players {
            if player.health > 0 {
                // Only draw players that are in the screen
                if player.x >= self.origin.x && player.x <= self.origin.x + screen_coords.w &&
                player.y >= self.origin.y && player.y <= self.origin.y + screen_coords.h {
                    // Draw each player as a filled rectangle
                    let rect = graphics::Rect::new((player.x - self.origin.x) * self.zoom, (player.y - self.origin.y) * self.zoom, 15.0 * self.zoom, 15.0 * self.zoom);

                    let rect_to_draw = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        player.color,
                    )?;
                    
                    graphics::draw(ctx, &rect_to_draw, (na::Point2::new(0.0, 0.0),))?;
                
                }
            
            }
        
        }
        
        for projectile in &self.projectiles {
            if projectile.x >= self.origin.x && projectile.x <= self.origin.x + screen_coords.w &&
            projectile.y >= self.origin.y && projectile.y <= self.origin.y + screen_coords.h {
                // Draw each projectile as a filled rectangle
                let rect = graphics::Rect::new((projectile.x - self.origin.x) * self.zoom, (projectile.y - self.origin.y) * self.zoom, 5.0 * self.zoom, 5.0 * self.zoom);

                let rect_to_draw = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    graphics::WHITE,
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
    
