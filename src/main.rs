mod game;

use ggez::{event, graphics};
use ggez::conf::{NumSamples, WindowSetup};
use ggez::graphics::{DrawParam, Rect};
use ggez::mint::Point2;
use ggez::timer::check_update_time;

use game::{collision, tick, Player, Projectile};

pub const WORLD_WIDTH: f32 = 1_000.0;
pub const WORLD_HEIGHT: f32 = 1_000.0;

struct MainState {
    players: [Player; 8],
    projectiles: Vec<Projectile>,
    map: Vec<Rect>,
    origin: Point2<f32>,
    zoom: f32,
    
}

impl MainState {
    fn new(mut num_of_players: u8, map: Vec<Rect>) -> MainState {
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
    
        MainState {
           players,
           projectiles: Vec::new(),
           map,
           origin: Point2 {x: 400.0, y: 300.0},
           zoom: 1.0,
           
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // Please see https://docs.rs/ggez/0.5.1/ggez/timer/fn.check_update_time.html for why I'm doing updates like this
        // Basically, the game will run 60 frames every second on average
        
        while check_update_time(ctx, 60) {
            self.players = tick(self.players, &mut self.projectiles, &self.map, ctx);
            
            self.origin.x = self.players[0].x - 400.0;
            self.origin.y = self.players[0].y - 300.0;
            
            if self.origin.x < 0.0 {
                self.origin.x = 0.0;
                
            } else if self.origin.x > WORLD_WIDTH {
                self.origin.x = WORLD_WIDTH;
                
            }
            if self.origin.y < 0.0 {
                self.origin.y = 0.0;
                
            } else if self.origin.y > WORLD_HEIGHT {
                self.origin.y = WORLD_HEIGHT;
                
            }
        
        }
            
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, (0, 0, 0).into());
        
        let screen_coords = graphics::screen_coordinates(&ctx);
        
        for player in &self.players {
            if player.health > 0 {
            
                // Only draw players that are in the screen
                // Recycling the collision function since I am basically just seeing if the two rectangles intersect
                if collision(&Rect::new(player.x, player.y, 15.0, 15.0), &Rect::new(self.origin.x, self.origin.y, screen_coords.w, screen_coords.h)) {
                
                    // Draw each player as a filled rectangle
                    let rect = graphics::Rect::new((player.x - self.origin.x) * self.zoom, (player.y - self.origin.y) * self.zoom, 15.0 * self.zoom, 15.0 * self.zoom);

                    let rect_to_draw = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        player.color,
                    )?;
                    
                    graphics::draw(ctx, &rect_to_draw, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) )?;
                
                }
            
            }
        
        }
        
        for projectile in &self.projectiles {
            if collision(&Rect::new(projectile.x, projectile.y, projectile.w, projectile.h), &Rect::new(self.origin.x, self.origin.y, screen_coords.w, screen_coords.h)) {

                let rect = graphics::Rect::new((projectile.x - self.origin.x) * self.zoom, (projectile.y - self.origin.y) * self.zoom, projectile.w * self.zoom, projectile.h * self.zoom);

                let rect_to_draw = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    (255, 255, 255).into(),
                )?;
                
                graphics::draw(ctx, &rect_to_draw, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) )?;
            }
        }
        
        for object in &self.map {
           if collision(&Rect::new(object.x, object.y, object.w, object.h), &Rect::new(self.origin.x, self.origin.y, screen_coords.w, screen_coords.h)) {
            
                let rect = graphics::Rect::new((object.x - self.origin.x) * self.zoom, (object.y - self.origin.y) * self.zoom, object.w * self.zoom, object.h * self.zoom);
            
                let rect_to_draw = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    (255, 255, 255).into(),
                )?;
                
                graphics::draw(ctx, &rect_to_draw, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) )?;
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
    
    let (ctx, event_loop) = cb.build()?;
    
    let state = MainState::new(2, vec![Rect::new(0.0, 0.0, 50.0, 50.0)]);
    event::run(ctx, event_loop, state)
    
}
    
