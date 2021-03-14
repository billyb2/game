use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

struct MainState {
    x: f32,
    going_right: bool,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { x: 0.0, going_right: true};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
    
        if self.x < 500.0 && self.going_right {
            self.x += 1.0;
            
        } else if self.x == 500.0 && self.going_right {
            self.going_right = false;
            
        } else if self.x > 0.0 && !self.going_right {
            self.x -= 1.0
            
        } else if self.x == 0.0 && !self.going_right {
            self.going_right = true;
            
        }
        
        println!("{}", self.x);
        
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        
        let rect = graphics::Rect::new(self.x, 100.0, 100.0, 100.0);

        let rect_to_draw = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::WHITE,
        )?;
        
        graphics::draw(ctx, &rect_to_draw, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
    
