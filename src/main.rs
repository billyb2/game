//Unfortuanetly, the disadvantages of having game_logic as its own seperate crate is that it needs to have a lot of arguments to calculate all of the data needed
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use ggez::{event,graphics};
use ggez::conf::{Backend, FullscreenType, NumSamples, WindowSetup, WindowMode};
use ggez::graphics::{Align, DrawParam, Image, PxScale, Text, TextFragment, screen_coordinates};
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::input::mouse;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::mint::Point2;
use ggez::timer::check_update_time;

use game_logic::{collision, tick};
use game_logic::map::Map;
use game_logic::objects;
use game_logic::objects::{Ability, Controls, Direction, Model, Player, Projectile, Rect, current_time};

use rand::thread_rng;
use rand::Rng;
use rust_embed::RustEmbed;

use std::collections::HashMap;
use std::convert::TryInto;

#[derive(RustEmbed)]
#[folder = "tiled/"]
struct Asset;

enum ViewScreen {
    StartScreen,
    Settings,
    Game,

}

struct View {
    view_screen: ViewScreen,
    button_texts: Vec<(Point2<f32>, Text)>,
    buttons: Vec<(Point2<f32>, u8, u8, (u8, u8, u8, u8))>,
    // The selected button is an index of the buttons Vec
    selected_button: Option<Text>,
}

impl View {
    fn default() -> View {
        View {
            view_screen: ViewScreen::StartScreen,
            buttons: Vec::with_capacity(10),
            button_texts: Vec::with_capacity(10),
            selected_button: None,

        }
    }

    fn add_button(&mut self, coords: (Point2<f32>, u8, u8, (u8, u8, u8, u8)), mut text: Text) {
        self.buttons.push(coords);

        text.set_bounds(Point2 {x: coords.1 as f32, y: coords.2 as f32}, Align::Center);
        self.button_texts.push((Point2 {x: coords.0.x, y: coords.0.y}, text));
    }


}

trait GameModeIndividual {
    fn add_to_score(&mut self, player_id: u8);

    fn current_score(&self) -> [u8; 20];

    fn win_conditions_met(&self) -> bool;
}

struct Deathmatch {
    points: [u8; 20],

}

impl Deathmatch {
    fn new() -> Deathmatch {
        Deathmatch {
            points: [0; 20],
        }
    }
}

impl GameModeIndividual for Deathmatch {
    fn add_to_score(&mut self, player_id: u8) {
        self.points[player_id as usize] += 1;

    }

    fn current_score(&self) -> [u8; 20] {
        self.points

    }

    fn win_conditions_met(&self) -> bool {
        let mut winning_player = false;

        for point_count in self.points.iter() {
            if point_count >= &20 {
                winning_player = true;
                break;

            }

        }

        winning_player


    }

}

//By wrapping each game mode in an enum, we can do all the fun stuff involving traits, without storing said GameMode in a Box on the heap, or do some funky stuff with dynamic Traits
// We can also do specific actions for every game mode, like respawns, etc since enums are easy to code for all possibilities.
//See https://bennetthardwick.com/blog/dont-use-boxed-trait-objects-for-struct-internals/
enum GameMode {
    Deathmatch(Deathmatch),

}

struct GameState {
    players: [Player; 20],
    projectiles: Vec<Projectile>,
    map: Map,
    //Game log is stored by how long it's on screen, and the actual content of said logical
    //Game log isn't the right word but it's basically like what's happening in the game
    logs: Vec<(String, u128)>,
    game_mode: GameMode,
    origin: Point2<f32>,
    zoom: f32,

}

impl GameState {
    fn log(&mut self, string: String) {
        // The log can be a maximum of 9 items long
        if self.logs.len() >= 9 {
            self.logs.pop();

        }

        //Push the log value to the top of the vector
        self.logs.insert(0, (string, current_time()));

    }

}

struct MainState {
    controls: Controls,
    game_state: GameState,
    rect_spritebatch: HashMap<(u8, u8, (u8, u8, u8, u8)), SpriteBatch>,
    view: View,
    
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
            let mut players: [Player; 20] = [Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline(), Player::offline()];
            
            let mut rng = thread_rng();

            for (i, player) in players.iter_mut().enumerate() {
                if !possible_player_spawns.is_empty(){
                    let spawn_index: usize = rng.gen_range(0..possible_player_spawns.len());
                    let (spawn_x, spawn_y) = possible_player_spawns[spawn_index];

                    *player = Player::new(None, Ability::Phase, 100, Model::BurstRifle, i.try_into().unwrap(), true, [spawn_x, spawn_y]);
                    possible_player_spawns.remove(spawn_index);

                } else {
                    break;
                    
                }
            }
            
            players[1].direction = Direction::E;
            
            players
            
        };

        MainState {
            view: View::default(),
            game_state: GameState {
                players,
                projectiles: Vec::new(),
                map,
                game_mode: GameMode::Deathmatch(Deathmatch::new()),
                logs: Vec::with_capacity(10),
                origin: Point2 {x: 596.0, y: 342.0},
                zoom: 1.0,
            },
            rect_spritebatch: HashMap::new(),
            controls: Controls::default(),

        }
    }

    fn update_start_screen(&mut self, ctx: &mut ggez::Context) {
        while check_update_time(ctx, 60) {
            let (_, mouse_click, mouse_coords) = check_user_input(ctx);
            let screen_coords = Rect { x: 0.0, y: 0.0, w: screen_coordinates(ctx).w, h: screen_coordinates(ctx).h };

            // If the user is left clicking and their coords are within the play button bounds
            if mouse_click[0] && mouse_coords.x >= screen_coords.w / 2.0 - 45.0 && mouse_coords.x <= screen_coords.w / 2.0 + 30.0 {

                if mouse_coords.y >= screen_coords.h / 3.0 && mouse_coords.y <= screen_coords.h / 3.0 + 25.0  {
                self.view.view_screen = ViewScreen::Game;

                } else if mouse_coords.y >= screen_coords.h / 2.5 && mouse_coords.y <= screen_coords.h / 2.5 + 25.0 {
                    self.view.view_screen = ViewScreen::Settings;

                }
            }
        }
    }

    //TODO: Change all text from physical coords to logical
    fn draw_start_screen(&mut self, ctx: &mut ggez::Context) {
        graphics::clear(ctx, (0, 0, 0).into());
        self.view.buttons.clear();
        self.view.button_texts.clear();

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
        self.view.buttons = Vec::with_capacity(0);
        self.view.button_texts = Vec::with_capacity(0);

        // Please see https://docs.rs/ggez/0.5.1/ggez/timer/fn.check_update_time.html for why I'm doing updates like this
        // Basically, the game will run 60 frames every second on average
        while check_update_time(ctx, 60) {
            let (keys_pressed, mouse_pressed, mouse_coords) = check_user_input(ctx);
            let screen_coords = Rect { x: 0.0, y: 0.0, w: graphics::window(ctx).inner_size().width as f32, h: graphics::window(ctx).inner_size().height as f32 };

            let damaged_players = tick(&mut self.game_state.players, &mut self.game_state.projectiles, &mut self.game_state.map, keys_pressed, mouse_pressed, mouse_coords, screen_coords, self.controls);

            match &self.game_state.game_mode {
                GameMode::Deathmatch(_) => {

                    for player_id in damaged_players.iter() {
                        if self.game_state.players[*player_id as usize].health > 0 {
                            self.game_state.log(format!("Player {} took damage", player_id + 1));

                        } else {
                            self.game_state.log(format!("Player {} got murked", player_id + 1));
                            //TODO: Add points to players who kill other players

                        }

                    }


                    let mut num_of_pops: u8 = 0;

                    for (_, time)in self.game_state.logs.iter().rev() {
                        if current_time() >= time + 8000 {
                            num_of_pops += 1;

                        } else {
                            break;

                        }

                    }

                    while num_of_pops > 0 {
                        self.game_state.logs.pop();
                        num_of_pops -= 1;

                    }
                },
            };

            self.game_state.origin.x = self.game_state.players[0].x - screen_coords.w / 2.0;
            self.game_state.origin.y = self.game_state.players[0].y - screen_coords.h / 2.0;

            if self.game_state.origin.x < 0.0 {
                self.game_state.origin.x = 0.0;

            } else if self.game_state.origin.x > self.game_state.map.width - screen_coords.w {
                self.game_state.origin.x = self.game_state.map.width - screen_coords.w;

            }
            if self.game_state.origin.y < 0.0 {
                self.game_state.origin.y = 0.0;

            } else if self.game_state.origin.y > self.game_state.map.height - screen_coords.h {
                self.game_state.origin.y = self.game_state.map.height - screen_coords.h;

            }

        }

    }

    //TODO: Change all text from physical coords to logical
    fn draw_game(&mut self, ctx: &mut ggez::Context){
        graphics::clear(ctx, (0, 0, 0).into());

        let screen_coords = Rect { x: 0.0, y: 0.0, w: graphics::window(ctx).inner_size().width as f32, h: graphics::window(ctx).inner_size().height as f32 };

        let mut map_objects: Vec<(Point2<f32>, u8, u8, (u8, u8, u8, u8))> = Vec::new();
        let mut projectiles: Vec<(Point2<f32>, u8, (u8, u8, u8, u8))> = Vec::new();

        for object in &self.game_state.map.objects {
            let u8_color: (u8, u8, u8, u8) = object.color.into();
            let color: graphics::Color = u8_color.into();

            let object = object.data;

           if collision(&Rect::new(object.x, object.y, object.w, object.h), &Rect::new(self.game_state.origin.x, self.game_state.origin.y, screen_coords.w, screen_coords.h)) {

                let rect = graphics::Rect::new((object.x - self.game_state.origin.x) * self.game_state.zoom, (object.y - self.game_state.origin.y) * self.game_state.zoom, object.w * self.game_state.zoom, object.h * self.game_state.zoom);

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

        for projectile in &self.game_state.projectiles {
            if collision(&Rect::new(projectile.x, projectile.y, projectile.w, projectile.h), &Rect::new(self.game_state.origin.x, self.game_state.origin.y, screen_coords.w, screen_coords.h)) {

                let rect = graphics::Rect::new((projectile.x - self.game_state.origin.x) * self.game_state.zoom, (projectile.y - self.game_state.origin.y) * self.game_state.zoom, projectile.w * self.game_state.zoom, projectile.h * self.game_state.zoom);


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

        //Draw log text here
        for (i, (log, time)) in self.game_state.logs.iter().enumerate() {
            // I have to use clone until I figure out a better method :'(
            let alpha: u8 = {
                if current_time() >= time + 4000 {
                    //TODO: Bitwise copy here (time)
                    // This basically cahnges the opacity of the text depending on how long it's been on screen
                    let alpha = (((*time + 8000) as f64 - (current_time()) as f64) / 4000.0) * 255.0;
                    alpha as u8

                } else {
                    255

                }


            };

            let color = graphics::Color::from_rgba(255, 255, 255, alpha);

            let log_text = Text::new(TextFragment::new(String::from(log)).color(color));

            graphics::draw(ctx, &log_text, DrawParam::default().dest(Point2 {x: screen_coords.w - 50.0 - log_text.width(ctx) as f32 * 2.0, y: screen_coords.h - 100.0 - i as f32 * 25.0})).unwrap();
        }


        // The players should be drawn over all other objects
        for player in &self.game_state.players {
            if player.online {

                // Only draw players that are in the screen
                // Recycling the collision function since I am basically just seeing if the two rectangles intersect
                if collision(&Rect::new(player.x, player.y, 15.0, 15.0), &Rect::new(self.game_state.origin.x, self.game_state.origin.y, screen_coords.w, screen_coords.h)) {

                    // Draw each player as a filled rectangle
                    let rect = graphics::Rect::new((player.x - self.game_state.origin.x) * self.game_state.zoom, (player.y - self.game_state.origin.y) * self.game_state.zoom, 15.0 * self.game_state.zoom, 15.0 * self.game_state.zoom);

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

        for (i, player) in self.game_state.players.iter().enumerate() {
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

            let (keys_pressed, mouse_pressed, mouse_coords) = check_user_input(ctx);

            let strip_everything_after_colon = |text: &mut Text| {
               // This just removes the colon and everything after it from the text
                let text_vec =  text.contents();

                let text_without_colon = String::from(text_vec.split(':').collect::<Vec<&str>>()[0]);

                text.fragments_mut()[0].text = text_without_colon;
            };


            if mouse_pressed[0] {
                let mut clicked_on_button = false;

                for (i, (_, text)) in self.view.button_texts.iter_mut().enumerate() {
                    let button = &self.view.buttons[i];

                    if mouse_coords.x >= button.0.x && mouse_coords.x <= button.0.x + button.1 as f32 && mouse_coords.y >= button.0.y && mouse_coords.y <= button.0.y + button.2 as f32 {
                        // The back button is special
                        if text.contents() == *"Back" {
                            self.view.view_screen = ViewScreen::StartScreen;

                        } else {
                            clicked_on_button = true;
                            strip_everything_after_colon(text);
                            // Ew I have to use a clone :(
                            self.view.selected_button = Some((*text).clone());

                        }

                    }
                }

                if let false = clicked_on_button { self.view.selected_button = None }
            }


            if !keys_pressed.is_empty() && self.view.selected_button.is_some() {
                let button = self.view.selected_button.as_ref().unwrap();

                if button.contents() == *"Up Button" {
                    self.view.selected_button = None;
                    self.controls.up =  *keys_pressed.last().unwrap();

                } else if button.contents() == *"Down Button" {
                    self.view.selected_button = None;
                    self.controls.down =  *keys_pressed.last().unwrap();

                } else if button.contents() == *"Left Button" {
                    self.view.selected_button = None;
                    self.controls.left =  *keys_pressed.last().unwrap();

                } else if button.contents() == *"Right Button" {
                    self.view.selected_button = None;
                    self.controls.right =  *keys_pressed.last().unwrap();

                } else if button.contents() == *"Use Ability Button" {
                    self.view.selected_button = None;
                    self.controls.use_ability =  *keys_pressed.last().unwrap();

                } else if button.contents() == *"Reload Button" {
                    self.view.selected_button = None;
                    self.controls.reload =  *keys_pressed.last().unwrap();

                }
            }
        }
    }

    //TODO: Please for the love of god refactor this

    fn draw_settings(&mut self, ctx: &mut ggez::Context) {
        graphics::clear(ctx, (0, 0, 0).into());
        self.view.buttons.clear();
        self.view.button_texts.clear();

        let screen_coords = Rect { x: 0.0, y: 0.0, w: graphics::window(ctx).inner_size().width as f32, h: graphics::window(ctx).inner_size().height as f32 };
        let up_button = (
            Point2 {
                x: physical_to_logical(ctx, screen_coords.w / 2.0 - 25.0, 0.0)[0],
                y: physical_to_logical(ctx, 0.0, screen_coords.h / 4.0)[1]
            }, 75, 30, (255, 255, 255, 255));

        let down_button = (
            Point2 {
                x: physical_to_logical(ctx, screen_coords.w / 2.0 - 25.0, 0.0)[0],
                y: physical_to_logical(ctx, 0.0, screen_coords.h / 3.5 + 25.0)[1]
            }, 75, 30, (255, 255, 255, 255));

        let left_button = (
            Point2 {
                x: physical_to_logical(ctx, screen_coords.w / 2.0 - 25.0, 0.0)[0],
                y: physical_to_logical(ctx, 0.0, screen_coords.h / 3.0 + 50.0)[1]
            }, 75, 30, (255, 255, 255, 255));

        let right_button = (
            Point2 {
                x: physical_to_logical(ctx, screen_coords.w / 2.0 - 25.0, 0.0)[0],
                y: physical_to_logical(ctx, 0.0, screen_coords.h / 2.5 + 50.0)[1]
            }, 75, 30, (255, 255, 255, 255));

        let use_ability_button = (
            Point2 {
                x: physical_to_logical(ctx, screen_coords.w / 2.0 - 25.0, 0.0)[0],
                y: physical_to_logical(ctx, 0.0, screen_coords.h / 2.0 + 25.0)[1]
            }, 75, 30, (255, 255, 255, 255));

        let reload_button = (
            Point2 {
                x: physical_to_logical(ctx, screen_coords.w / 2.0 - 25.0, 0.0)[0],
                y: physical_to_logical(ctx, 0.0, screen_coords.h / 1.5 - 40.0)[1]
            }, 75, 30, (255, 255, 255, 255));


        let up_button_text = match &self.view.selected_button {
            Some(text) => {
                if text.contents() == *"Up Button" {
                    text.clone()

                } else {
                    Text::new(TextFragment::new(format!("Up Button: {}", self.controls.up.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0)))

                }

            },
            None => Text::new(TextFragment::new(format!("Up Button: {}", self.controls.up.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0))),
        };

        let down_button_text = match &self.view.selected_button {
            Some(text) => {
                if text.contents() == *"Down Button" {
                    text.clone()

                } else {
                    Text::new(TextFragment::new(format!("Down Button: {}", self.controls.down.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0)))

                }

            },
            None => Text::new(TextFragment::new(format!("Down Button: {}", self.controls.down.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0))),
        };

        let left_button_text = match &self.view.selected_button {
            Some(text) => {
                if text.contents() == *"Left Button" {
                    text.clone()

                } else {
                    Text::new(TextFragment::new(format!("Left Button: {}", self.controls.left.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0)))

                }

            },
            None => Text::new(TextFragment::new(format!("Left Button: {}", self.controls.left.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0))),
        };

        let right_button_text = match &self.view.selected_button {
            Some(text) => {
                if text.contents() == *"Right Button" {
                    text.clone()

                } else {
                    Text::new(TextFragment::new(format!("Right Button: {}", self.controls.right.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0)))

                }

            },
            None => Text::new(TextFragment::new(format!("Right Button: {}", self.controls.right.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0))),
        };

        let use_ability_button_text = match &self.view.selected_button {
            Some(text) => {
                if text.contents() == *"Use Ability Button" {
                    text.clone()

                } else {
                    Text::new(TextFragment::new(format!("Use Ability Button: {}", self.controls.use_ability.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0)))

                }

            },
            None => Text::new(TextFragment::new(format!("Use Ability Button: {}", self.controls.use_ability.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0))),
        };

        let reload_button_text = match &self.view.selected_button {
            Some(text) => {
                if text.contents() == *"Reload Button" {
                    text.clone()

                } else {
                    Text::new(TextFragment::new(format!("Reload Button: {}", self.controls.reload.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0)))

                }

            },
            None => Text::new(TextFragment::new(format!("Reload Button: {}", self.controls.reload.to_uppercase())).scale(PxScale::from(14.5)).color(graphics::Color::from_rgb(0, 0, 0))),
        };

        let back_button = (
            Point2 {
                x: 15.0,
                y: 15.0,
            }, 50, 20, (255, 255, 255, 255));

        let back_text = Text::new(TextFragment::new("Back").scale(PxScale::from(18.0)).color(graphics::Color::from_rgb(0, 0, 0)));

        self.view.add_button(up_button, up_button_text);
        self.view.add_button(down_button, down_button_text);
        self.view.add_button(left_button, left_button_text);
        self.view.add_button(right_button, right_button_text);
        self.view.add_button(use_ability_button, use_ability_button_text);
        self.view.add_button(reload_button, reload_button_text);

        self.view.add_button(back_button, back_text);

        let settings_text = TextFragment::new("Settings").scale(PxScale::from(24.0));

        let mut settings_text = Text::new(settings_text);
        settings_text.set_bounds(Point2 {x: 150.0, y: 100.0 }, Align::Center);

        let color = graphics::Color::from_rgb(255, 255, 255);
        let vec_size_back = (back_button.1 as usize) *  (back_button.2 as usize) * 4;
        let vec_size_button = up_button.1 as usize * up_button.2 as usize * 4;


        self.rect_spritebatch.entry((back_button.1 as u8, back_button.2 as u8, back_button.3)).or_insert_with(|| SpriteBatch::new(
            Image::from_rgba8(
                ctx,
                back_button.1 as u16,
                back_button.2 as u16,
                &generate_image_from_rgba8(color, vec_size_back),
            ).unwrap())
        );

        self.rect_spritebatch.entry((up_button.1 as u8, up_button.2 as u8, color.into())).or_insert_with(|| SpriteBatch::new(
            Image::from_rgba8(
                ctx,
                up_button.1 as u16,
                up_button.2 as u16,
                &generate_image_from_rgba8(color, vec_size_button),
            ).unwrap())
        );

        for (pos, w, h, color) in &self.view.buttons {
            self.rect_spritebatch.get_mut(&(*w, *h, *color)).unwrap().add(DrawParam::default().dest(*pos));

        }


        for (_, spritebatch) in self.rect_spritebatch.iter_mut() {
            graphics::draw(ctx, spritebatch, DrawParam::default().dest(Point2 {x: 0.0, y: 0.0}) ).unwrap();
            spritebatch.clear();

        }

        for (coords, text) in &self.view.button_texts {
            graphics::draw(ctx, text, DrawParam::default().dest(*coords)).unwrap();
        }

        graphics::draw(ctx, &settings_text, DrawParam::default().dest(Point2 {x : physical_to_logical(ctx, screen_coords.w / 2.0 - 50.0, 0.0)[0], y: physical_to_logical(ctx, 0.0, screen_coords.h / 5.0)[1]})).unwrap();

        graphics::present(ctx).unwrap();
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {

        match self.view.view_screen {
            ViewScreen::StartScreen => MainState::draw_start_screen(self, ctx),
            ViewScreen::Game => MainState::draw_game(self, ctx),
            ViewScreen::Settings => MainState::draw_settings(self, ctx),
        };

        Ok(())

    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        match self.view.view_screen {
            ViewScreen::StartScreen => MainState::update_start_screen(self, ctx),
            ViewScreen::Settings => MainState::update_settings(self, ctx),
            ViewScreen::Game => MainState::update_game(self, ctx),
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

//TODO: Maybe move non-drawing scripts to a secondary file?
fn check_user_input(ctx: &ggez::Context) -> (Vec<char>, [bool; 3], objects::Point2) {
    let mut keys_pressed: Vec<char> = Vec::new();
    let mut mouse_pressed: [bool; 3] = [false; 3];

    let logical_mouse_coords = physical_to_logical(ctx, mouse::position(ctx).x, mouse::position(ctx).y);

    let mouse_coords = objects::Point2 { x: logical_mouse_coords[0], y: logical_mouse_coords[1]};

    if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
        mouse_pressed[0] = true;

    }

    // Totally not autgenerated code
    if is_key_pressed(ctx, KeyCode::A) {
            keys_pressed.push('a');
    }
    if is_key_pressed(ctx, KeyCode::B) {
            keys_pressed.push('b');
    }
    if is_key_pressed(ctx, KeyCode::C) {
            keys_pressed.push('c');
    }
    if is_key_pressed(ctx, KeyCode::D) {
            keys_pressed.push('d');
    }
    if is_key_pressed(ctx, KeyCode::E) {
            keys_pressed.push('e');
    }
    if is_key_pressed(ctx, KeyCode::F) {
            keys_pressed.push('f');
    }
    if is_key_pressed(ctx, KeyCode::G) {
            keys_pressed.push('g');
    }
    if is_key_pressed(ctx, KeyCode::H) {
            keys_pressed.push('h');
    }
    if is_key_pressed(ctx, KeyCode::I) {
            keys_pressed.push('i');
    }
    if is_key_pressed(ctx, KeyCode::J) {
            keys_pressed.push('j');
    }
    if is_key_pressed(ctx, KeyCode::K) {
            keys_pressed.push('k');
    }
    if is_key_pressed(ctx, KeyCode::L) {
            keys_pressed.push('l');
    }
    if is_key_pressed(ctx, KeyCode::M) {
            keys_pressed.push('m');
    }
    if is_key_pressed(ctx, KeyCode::N) {
            keys_pressed.push('n');
    }
    if is_key_pressed(ctx, KeyCode::O) {
            keys_pressed.push('o');
    }
    if is_key_pressed(ctx, KeyCode::P) {
            keys_pressed.push('p');
    }
    if is_key_pressed(ctx, KeyCode::Q) {
            keys_pressed.push('q');
    }
    if is_key_pressed(ctx, KeyCode::R) {
            keys_pressed.push('r');
    }
    if is_key_pressed(ctx, KeyCode::S) {
            keys_pressed.push('s');
    }
    if is_key_pressed(ctx, KeyCode::T) {
            keys_pressed.push('t');
    }
    if is_key_pressed(ctx, KeyCode::U) {
            keys_pressed.push('u');
    }
    if is_key_pressed(ctx, KeyCode::V) {
            keys_pressed.push('v');
    }
    if is_key_pressed(ctx, KeyCode::W) {
            keys_pressed.push('w');
    }
    if is_key_pressed(ctx, KeyCode::X) {
            keys_pressed.push('x');
    }
    if is_key_pressed(ctx, KeyCode::Y) {
            keys_pressed.push('y');
    }
    if is_key_pressed(ctx, KeyCode::Z) {
            keys_pressed.push('z');
    }
    if is_key_pressed(ctx, KeyCode::A) {
            keys_pressed.push('a');
    }
    if is_key_pressed(ctx, KeyCode::B) {
            keys_pressed.push('b');
    }
    if is_key_pressed(ctx, KeyCode::C) {
            keys_pressed.push('c');
    }
    if is_key_pressed(ctx, KeyCode::D) {
            keys_pressed.push('d');
    }
    if is_key_pressed(ctx, KeyCode::E) {
            keys_pressed.push('e');
    }
    if is_key_pressed(ctx, KeyCode::F) {
            keys_pressed.push('f');
    }
    if is_key_pressed(ctx, KeyCode::G) {
            keys_pressed.push('g');
    }
    if is_key_pressed(ctx, KeyCode::H) {
            keys_pressed.push('h');
    }
    if is_key_pressed(ctx, KeyCode::I) {
            keys_pressed.push('i');
    }
    if is_key_pressed(ctx, KeyCode::J) {
            keys_pressed.push('j');
    }
    if is_key_pressed(ctx, KeyCode::K) {
            keys_pressed.push('k');
    }
    if is_key_pressed(ctx, KeyCode::L) {
            keys_pressed.push('l');
    }
    if is_key_pressed(ctx, KeyCode::M) {
            keys_pressed.push('m');
    }
    if is_key_pressed(ctx, KeyCode::N) {
            keys_pressed.push('n');
    }
    if is_key_pressed(ctx, KeyCode::O) {
            keys_pressed.push('o');
    }
    if is_key_pressed(ctx, KeyCode::P) {
            keys_pressed.push('p');
    }
    if is_key_pressed(ctx, KeyCode::Q) {
            keys_pressed.push('q');
    }
    if is_key_pressed(ctx, KeyCode::R) {
            keys_pressed.push('r');
    }
    if is_key_pressed(ctx, KeyCode::S) {
            keys_pressed.push('s');
    }
    if is_key_pressed(ctx, KeyCode::T) {
            keys_pressed.push('t');
    }
    if is_key_pressed(ctx, KeyCode::U) {
            keys_pressed.push('u');
    }
    if is_key_pressed(ctx, KeyCode::V) {
            keys_pressed.push('v');
    }
    if is_key_pressed(ctx, KeyCode::W) {
            keys_pressed.push('w');
    }
    if is_key_pressed(ctx, KeyCode::X) {
            keys_pressed.push('x');
    }
    if is_key_pressed(ctx, KeyCode::Y) {
            keys_pressed.push('y');
    }
    if is_key_pressed(ctx, KeyCode::Z) {
            keys_pressed.push('z');
    }
    if is_key_pressed(ctx, KeyCode::Right) {
            keys_pressed.push('→');
    }
    if is_key_pressed(ctx, KeyCode::Left) {
            keys_pressed.push('←');
    }
    if is_key_pressed(ctx, KeyCode::Up) {
            keys_pressed.push('↑');
    }
    if is_key_pressed(ctx, KeyCode::Down) {
            keys_pressed.push('↓');
    }
    if is_key_pressed(ctx, KeyCode::Space) {
            keys_pressed.push(' ');
    }

    (keys_pressed, mouse_pressed, mouse_coords)
}

fn physical_to_logical(ctx: &ggez::Context, x: f32, y: f32) -> [f32; 2] {
    let screen_rect = graphics::screen_coordinates(ctx);
    let size = graphics::window(ctx).inner_size();

    let logical_x = (x / (size.width  as f32)) * screen_rect.w + screen_rect.x;
    let logical_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;


    [logical_x, logical_y]
}
