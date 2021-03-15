use ggez::{event, graphics, nalgebra as na};
use ggez::conf::WindowSetup;
use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::input::mouse;

struct MainState {
    x: f32,
    y: f32,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState { x: 0.0, y: 0.0};
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if is_key_pressed(ctx, KeyCode::D) || is_key_pressed(ctx, KeyCode::Right) {
            self.x += 10.0;
        } if is_key_pressed(ctx, KeyCode::A) || is_key_pressed(ctx, KeyCode::Left) {
            self.x -= 10.0;
        } if is_key_pressed(ctx, KeyCode::W) || is_key_pressed(ctx, KeyCode::Up) {
            self.y -= 10.0;
        } if is_key_pressed(ctx, KeyCode::S) || is_key_pressed(ctx, KeyCode::Down) {
            self.y += 10.0;
        }
        
        if mouse::button_pressed(&ctx, mouse::MouseButton::Left) {
            //println!("Hello mouse!");
            
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        
        let rect = graphics::Rect::new(self.x, self.y, 25.0, 25.0);

        let rect_to_draw = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::from_rgb(255, 0, 0),
        )?;
        
        graphics::draw(ctx, &rect_to_draw, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let cb = cb.window_setup(WindowSetup::default());
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
    
