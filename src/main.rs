#![allow(clippy::type_complexity)]

use ggez::{event,graphics};
use ggez::conf::{Backend, FullscreenType, NumSamples, WindowSetup, WindowMode};
use ggez::graphics::{DrawParam, Image, Text, TextFragment, screen_coordinates};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::mouse;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::mint::Point2;
use ggez::timer::check_update_time;

use game_logic::{collision, tick};
use game_logic::map::Map;
use game_logic::objects;
use game_logic::objects::{Player, Projectile, Rect};

use rand::thread_rng;
use rand::Rng;
use rust_embed::RustEmbed;

use std::collections::HashMap;
use std::convert::TryInto;

#[derive(RustEmbed)]
#[folder = "tiled/"]
struct Asset;


struct MainState {
    //0 the start screen
    //1 is the game
    //2 is the settings
    view: u8,
    players: [Player; 20],
    projectiles: Vec<Projectile>,
    map: Map,
    origin: Point2<f32>,
    zoom: f32,    
    rect_spritebatch: HashMap<(u8, u8, (u8, u8, u8, u8)), SpriteBatch>,
    
}

impl MainState {
    fn new(map: Map) -> MainState {
        let mut possible_player_spawns: Vec<(f32, f32)> =
        {
            let mut possible_player_spawns: Vec<(f32, f32)> = Vec::new();

            for map_object in &map.objects {
                if map_object.player_spawn {
                    possible_player_spawns.push((map_object.data.x, map_object.data.y));

                }

            }

            possible_player_spawns

        };

        // Preallocate memory for the maximum of 20 players
        let players: [Player; 20] ={
            let mut players: [Player; 20] = [Player::new(None, 0, 0, 0, 0, false, [0.0, 0.0]); 20];
            
            let mut rng = thread_rng();

            for (i, player) in players.iter_mut().enumerate() {
                if !possible_player_spawns.is_empty(){
                    let spawn_index: usize = rng.gen_range(0..possible_player_spawns.len());
                    let (spawn_x, spawn_y) = possible_player_spawns[spawn_index];

                    *player = Player::new(None, 0, 100, 3, i.try_into().unwrap(), true, [spawn_x, spawn_y]);
                    possible_player_spawns.remove(spawn_index);

                } else {
                    break;
                    
                }
            }
            
            players[1].direction = 3;
            
            players
            
        };

        MainState {
            view: 0,
            players,
            projectiles: Vec::new(),
            map,
            origin: Point2 {x: 596.0, y: 342.0},
            zoom: 1.0,
            rect_spritebatch: HashMap::new(),
           
        }
    }

    fn update_start_screen(&mut self, ctx: &mut ggez::Context) {
        while check_update_time(ctx, 60) {
            let (_, mouse_click, mouse_coords) = check_user_input(ctx);
            let screen_coords = Rect { x: 0.0, y: 0.0, w: screen_coordinates(ctx).w, h: screen_coordinates(ctx).h };

            // If the user is left clicking and their coords are within the play button bounds
            if mouse_click[0] && mouse_coords.x >= screen_coords.w / 2.0 - 45.0 && mouse_coords.x <= screen_coords.w / 2.0 + 30.0 {

                if mouse_coords.y >= screen_coords.h / 3.0 && mouse_coords.y <= screen_coords.h / 3.0 + 25.0  {
                self.view = 1;

                } else if mouse_coords.y >= screen_coords.h / 2.5 && mouse_coords.y <= screen_coords.h / 2.5 + 25.0 {
                    self.view = 2;

                }
            }
        }
    }


    fn draw_start_screen(&mut self, ctx: &mut ggez::Context) {
        graphics::clear(ctx, (0, 0, 0).into());
        let screen_coords = Rect { x: 0.0, y: 0.0, w: screen_coordinates(ctx).w, h: screen_coordinates(ctx).h };

        let title = Text::new("Necrophaser");


        let mut buttons = Vec::new();
        let button = graphics::Rect::new(screen_coords.w / 2.0 - 45.0 , screen_coords.h / 3.0, 75.0, 25.0);
        let button2 = graphics::Rect::new(screen_coords.w / 2.0 - 45.0 , screen_coords.h / 2.5, 75.0, 25.0);

        let color = graphics::Color::from_rgb(255, 255, 255);
        let vec_size = (button.w as usize) *  (button.h as usize) * 4;

        self.rect_spritebatch.entry((button.w as u8, button.h as u8, color.into())).or_insert_with(|| SpriteBatch::new(
            Image::from_rgba8(
                ctx,
                button.w as u16,
                button.h as u16,
                &generate_image_from_rgba8(color, vec_size),
            ).unwrap())
        );


        buttons.push((Point2 {x: button.x, y: button.y}, button.w as u8, button.h as u8, color.into()));

        buttons.push((Point2 {x: button2.x, y: button2.y}, button2.w as u8, button2.h as u8, color.into()));


        for (pos, w, h, color) in buttons.iter() {
            self.rect_spritebatch.get_mut(&(*w, *h, *color)).unwrap().add(DrawParam::default().dest(*pos));

        }


        for (_, spritebatch) in self.rect_spritebatch.iter_mut() {
            graphics::draw(ctx, spritebatch, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) ).unwrap();
            spritebatch.clear();

        }

        graphics::draw(ctx, &title, DrawParam::default().dest(Point2 {x : screen_coords.w / 2.0 - 50.0, y : screen_coords.h / 4.0 })).unwrap();

        graphics::draw(ctx, &Text::new(TextFragment::new("Play").color(graphics::Color::from_rgb(0, 0, 0))), DrawParam::default().dest(Point2 {x : screen_coords.w / 2.0 - 25.0, y : screen_coords.h / 3.0 })).unwrap();

        graphics::draw(ctx, &Text::new(TextFragment::new("Settings").color(graphics::Color::from_rgb(0, 0, 0))), DrawParam::default().dest(Point2 {x : screen_coords.w / 2.0 - 40.0, y : screen_coords.h / 2.5 })).unwrap();


        graphics::present(ctx).unwrap();
    }

    fn update_game(&mut self, ctx: &mut ggez::Context) {
        // Please see https://docs.rs/ggez/0.5.1/ggez/timer/fn.check_update_time.html for why I'm doing updates like this
        // Basically, the game will run 60 frames every second on average
        while check_update_time(ctx, 60) {
            let (keys_pressed, mouse_pressed, mouse_coords) = check_user_input(ctx);
            let screen_coords = Rect { x: 0.0, y: 0.0, w: graphics::window(ctx).inner_size().width as f32, h: graphics::window(ctx).inner_size().height as f32 };

            self.players = tick(self.players, &mut self.projectiles, &mut self.map, keys_pressed, mouse_pressed, mouse_coords, screen_coords);

            self.origin.x = self.players[0].x - screen_coords.w / 2.0;
            self.origin.y = self.players[0].y - screen_coords.h / 2.0;

            if self.origin.x < 0.0 {
                self.origin.x = 0.0;

            } else if self.origin.x > self.map.width - screen_coords.w {
                self.origin.x = self.map.width - screen_coords.w;

            }
            if self.origin.y < 0.0 {
                self.origin.y = 0.0;

            } else if self.origin.y > self.map.height - screen_coords.h {
                self.origin.y = self.map.height - screen_coords.h;

            }

        }

    }

    fn draw_game(&mut self, ctx: &mut ggez::Context){
        graphics::clear(ctx, (0, 0, 0).into());

        let screen_coords = Rect { x: 0.0, y: 0.0, w: graphics::window(ctx).inner_size().width as f32, h: graphics::window(ctx).inner_size().height as f32 };

        let mut map_objects: Vec<(Point2<f32>, u8, u8, (u8, u8, u8, u8))> = Vec::new();
        let mut projectiles: Vec<(Point2<f32>, u8, (u8, u8, u8, u8))> = Vec::new();

        for object in &self.map.objects {
            let u8_color: (u8, u8, u8, u8) = object.color.into();
            let color: graphics::Color = u8_color.into();

            let object = object.data;

           if collision(&Rect::new(object.x, object.y, object.w, object.h), &Rect::new(self.origin.x, self.origin.y, screen_coords.w, screen_coords.h)) {

                let rect = graphics::Rect::new((object.x - self.origin.x) * self.zoom, (object.y - self.origin.y) * self.zoom, object.w * self.zoom, object.h * self.zoom);

                 let vec_size = ((object.w as usize) *  (object.h as usize)) * 4;

                self.rect_spritebatch.entry((object.w as u8, object.h as u8, color.into())).or_insert_with(|| SpriteBatch::new(
                    Image::from_rgba8(
                        ctx,
                        object.w as u16,
                        object.h as u16,
                        &generate_image_from_rgba8(color, vec_size),
                    ).unwrap())
                );

                map_objects.push((Point2 {x: rect.x, y: rect.y}, object.w as u8, object.h as u8, color.into()));
            }
        }

        for projectile in &self.projectiles {
            if collision(&Rect::new(projectile.x, projectile.y, projectile.w, projectile.h), &Rect::new(self.origin.x, self.origin.y, screen_coords.w, screen_coords.h)) {

                let rect = graphics::Rect::new((projectile.x - self.origin.x) * self.zoom, (projectile.y - self.origin.y) * self.zoom, projectile.w * self.zoom, projectile.h * self.zoom);


                let size = projectile.w as u16;
                let vec_size = ((size as usize) *  (size as usize)) * 4;

                self.rect_spritebatch.entry((projectile.w as u8, projectile.h as u8, (255, 255, 255, 255))).or_insert_with(|| SpriteBatch::new(
                    Image::from_rgba8(
                        ctx,
                        size,
                        size,
                        &vec![255; vec_size],
                    ).unwrap())
                );

                projectiles.push((Point2 {x: rect.x, y: rect.y}, projectile.w as u8, (255, 255, 255, 255)));

            }
        }

        for (pos, w, h, color) in map_objects.iter() {
            self.rect_spritebatch.get_mut(&(*w, *h, *color)).unwrap().add(DrawParam::default().dest(*pos));

        }


        for (_, spritebatch) in self.rect_spritebatch.iter_mut() {
            graphics::draw(ctx, spritebatch, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) ).unwrap();
            spritebatch.clear();

        }

        for (pos, size, color) in projectiles.iter() {
            self.rect_spritebatch.get_mut(&(*size, *size, *color)).unwrap().add(DrawParam::default().dest(*pos));

        }

        for (_, spritebatch) in self.rect_spritebatch.iter_mut() {
            graphics::draw(ctx, spritebatch, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) ).unwrap();
            spritebatch.clear();

        }

        // The players should be drawn over all other objects
        for player in &self.players {
            if player.online {

                // Only draw players that are in the screen
                // Recycling the collision function since I am basically just seeing if the two rectangles intersect
                if collision(&Rect::new(player.x, player.y, 15.0, 15.0), &Rect::new(self.origin.x, self.origin.y, screen_coords.w, screen_coords.h)) {

                    // Draw each player as a filled rectangle
                    let rect = graphics::Rect::new((player.x - self.origin.x) * self.zoom, (player.y - self.origin.y) * self.zoom, 15.0 * self.zoom, 15.0 * self.zoom);

                    let color: (u8, u8, u8, u8) = player.color.into();

                    let rect_to_draw = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        color.into(),
                    ).unwrap();

                    graphics::draw(ctx, &rect_to_draw, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) ).unwrap();

                }

            }

        }


        let mut text_y = 0.0;
        let mut text_x_offset = 650.0;

        for (i, player) in self.players.iter().enumerate() {
            if player.health > 0 {
                let health = Text::new(format!("Player {} health: {}", i + 1, player.health));
                let ability_charge_percent = Text::new(format!("Player {} charge: {:.0}%", i + 1, player.ability_charge as f32 / player.max_ability_charge as f32 * 100.0));
                let position_x = Text::new(format!("Player {} X Position: {}", i + 1, player.x));
                let position_y = Text::new(format!("Player {} Y Position: {}", i + 1, player.y));
                let ammo = match player.gun.reloading {
                    true => Text::new(format!("Player {}: Reloading", i + 1)),
                    false => Text::new(format!("Player {}: {} / {}", i + 1, player.gun.ammo_in_mag, player.gun.max_ammo)),
                };

                //TODO: Eventually use queue text instead of multiple draw calls, since queue text is the equivalent of sprite batching
                graphics::draw(ctx, &health, DrawParam::default().dest(Point2 {x: screen_coords.w - text_x_offset, y: text_y})).unwrap();

                text_y += 25.0;

                graphics::draw(ctx, &position_x, DrawParam::default().dest(Point2 {x : screen_coords.w - text_x_offset, y : text_y})).unwrap();

                text_y += 25.0;

                graphics::draw(ctx, &position_y, DrawParam::default().dest(Point2 {x : screen_coords.w - text_x_offset, y : text_y})).unwrap();

                text_y += 25.0;

                graphics::draw(ctx, &ability_charge_percent, DrawParam::default().dest(Point2 {x : screen_coords.w - text_x_offset, y : text_y})).unwrap();

                text_y += 25.0;

                graphics::draw(ctx, &ammo, DrawParam::default().dest(Point2 {x : screen_coords.w - text_x_offset, y : text_y})).unwrap();

            }

            text_y = 0.0;
            text_x_offset -= 250.0;
        }

        graphics::present(ctx).unwrap();
    }

    fn update_settings(&mut self, ctx: &mut ggez::Context) {
        while check_update_time(ctx, 60) {

        }
    }

    fn draw_settings(&mut self, ctx: &mut ggez::Context) {
        graphics::clear(ctx, (0, 0, 0).into());



        graphics::present(ctx).unwrap();
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.view {
            0 => MainState::draw_start_screen(self, ctx),
            1 => MainState::draw_game(self, ctx),
            2 => MainState::draw_settings(self, ctx),
            _ => MainState::draw_game(self, ctx),
        };

        Ok(())

    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.view {
            0 => MainState::update_start_screen(self, ctx),
            1 => MainState::update_game(self, ctx),
            2 => MainState::update_settings(self, ctx),
            _ => MainState::update_game(self, ctx)
        };

        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("game", "William Batista + Luke Gaston")
    .window_mode (
        WindowMode {
            // Resolution is always 1080p
            width: 1152.0,
            height: 648.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 1152.0,
            max_width: 1920.0,
            min_height: 648.0,
            max_height: 1080.0,
            resizable: true,
            visible: true,
    })
    .backend (
        // MacOS doesn't support OpenGL 4.5
        if std::env::consts::OS != "macos" {
            Backend::OpenGL {
                major: 4,
                minor: 5,
            }
        } else {
            Backend::OpenGL {
                major: 3,
                minor: 2,
            }
        }
    );
    
    let cb = cb.window_setup(
        WindowSetup {
        title: "A game by the beacon boys".to_string(),
        // 8x antialiasing for a block game lol
        samples: NumSamples::Eight,
        // Vsync to make the framerate look pretty
        vsync: true,
        icon: String::new(),
        srgb: true,
    });
    
    let (ctx, event_loop) = cb.build()?;

    let map_bytes = Asset::get("map1.custom").unwrap();

    
    let state = MainState::new(Map::from_bin(&map_bytes));
    event::run(ctx, event_loop, state)
    
}

// The image size is in bytes, and generally should be equal to the length of the image * the width * 4
fn generate_image_from_rgba8(color: graphics::Color, image_size: usize) -> Vec<u8> {
    if image_size % 4 != 0 {
        //Probably should use a better solution then just straight up panicking
        panic!("Make sure you read the instructions for the image size!");

    }

    let mut bytes_vec: Vec<u8> = Vec::with_capacity(image_size);
    let rgba = color.to_rgba();

    let r = rgba.0;
    let g = rgba.1;
    let b = rgba.2;
    let a = rgba.3;

    while bytes_vec.len() != bytes_vec.capacity() {
        bytes_vec.push(r);
        bytes_vec.push(g);
        bytes_vec.push(b);
        bytes_vec.push(a);

    }

    bytes_vec
}

fn check_user_input(ctx: &ggez::Context) -> (Vec<char>, [bool; 3], objects::Point2) {
    let screen_rect = graphics::screen_coordinates(ctx);
    let size = graphics::window(ctx).inner_size();

    let mut keys_pressed: Vec<char> = Vec::new();
    let mut mouse_pressed: [bool; 3] = [false; 3];

    let mouse_coords = objects::Point2 { x: (mouse::position(ctx).x / (size.width as f32)) * screen_rect.w + screen_rect.x, y:  (mouse::position(ctx).y / (size.height as f32)) * screen_rect.h + screen_rect.y };

    if is_key_pressed(ctx, KeyCode::W) {
        keys_pressed.push('w');

    }

    if is_key_pressed(ctx, KeyCode::A) {
        keys_pressed.push('a');

    }

    if is_key_pressed(ctx, KeyCode::S) {
        keys_pressed.push('s');

    }

    if is_key_pressed(ctx, KeyCode::D) {
        keys_pressed.push('d');

    }

    if is_key_pressed(ctx, KeyCode::R) {
        keys_pressed.push('r');

    }

    if is_key_pressed(ctx, KeyCode::E) {
        keys_pressed.push('e');

    }

    if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
        mouse_pressed[0] = true;

    }

    (keys_pressed, mouse_pressed, mouse_coords)
}

