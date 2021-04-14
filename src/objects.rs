#![allow(clippy::upper_case_acronyms)]
// Rust compiler complains about snake case, Clippy complains about non snake case :(
#![allow(non_snake_case)]

use crate::helper_functions::{current_time, out_of_bounds};
use crate::map::{Map, MapObject};
use getrandom::getrandom;

#[derive(PartialEq)]
pub enum ProjectileType {
    Regular,
    Speedball,
}

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub right: bool,
    pub angle: f32,
    pub speed: f32,
    pub damage: u8,
    pub fired_from: u8, // Fixes a game-breaking bug where the bots get killed by their own bullets
    pub projectile_type: ProjectileType,
    pub distance_traveled: f32,
    pub max_distance: f32,

}

#[derive(Copy, Clone, PartialEq)]
pub enum Model {
    Pistol,
    Shotgun,
    Speedball,
    BurstRifle,
    AssaultRifle,
}

#[derive(Copy, Clone)]
pub struct Gun {
    pub model: Model,
    // This time is stored so that the bullets per second of guns can be limited dynamically
    pub time_since_last_shot: u128,
    pub time_since_start_reload: u128,
    // Shooting's,arguments are the arguments it had previously from the last frame, used for guns that don't just shoot one bullet at a time (like the burst rifle)
    pub shooting: Option<(f32, f32, bool, f32)>,
    pub projectiles_fired: u8,
    pub reloading: bool,
    // Reload time is in miliseconds
    pub reload_time: u16,
    pub ammo_in_mag: u8,
    pub max_ammo: u8,
    pub damage: u8,
    pub fired_from: u8,
    pub max_distance: f32,

}

impl Gun {
    pub fn new(model: Model, ability: Ability, player_fired_from: u8) -> Gun {
        let mut gun = Gun {
            model,
            // The time since the last shot is set as 0 so that you can start shooting as the start of the game
            time_since_last_shot: 0,
            time_since_start_reload: 0,
            reloading: false,
            reload_time: match model {
                Model::Pistol => 2000,
                Model::Shotgun => 5000,
                Model::Speedball => 3000,
                Model::BurstRifle => 3250,
                Model::AssaultRifle => 3750,
            },
            // Some guns have special shooting behaviors that last over the course of mutliple ticks, which shooting and projectiles_fired take advantage of
            shooting: None,
            projectiles_fired: 0,
            ammo_in_mag: match model {
                Model::Pistol=> 16,
                Model::Shotgun => 8,
                Model::Speedball => 6,
                Model::BurstRifle => 21,
                Model::AssaultRifle => 25,

            },
            max_ammo: match model {
                Model::Pistol => 16,
                Model::Shotgun => 8,
                Model::Speedball => 6,
                Model::BurstRifle => 21,
                Model::AssaultRifle => 25,

            },
            damage: match model {
                Model::Pistol => 45,
                Model::Shotgun => 25,
                Model::Speedball => 1,
                Model::BurstRifle => 13,
                Model::AssaultRifle => 15,

            },
            fired_from: player_fired_from,
            max_distance: match model {
                Model::Pistol => 900.0,
                Model::Shotgun => 300.0,
                Model::Speedball => 3000.0,
                Model::BurstRifle => 1000.0,
                Model::AssaultRifle => 1000.0,

            }

        };

        // The engineer ability can reload in half the time
        if ability == Ability::Engineer {
            gun.reload_time /= 2;

        }

        gun

    }

    pub fn reload (&mut self) {
        if !self.reloading {
            // Start reloading
            self.time_since_start_reload = current_time();
            self.reloading = true;

        } else if self.time_since_start_reload + self.reload_time as u128 <= current_time() {
            self.ammo_in_mag = self.max_ammo;
            self.reloading = false;

        }
    }

    pub fn shoot (&mut self, x: f32, y: f32, right: bool, angle: f32, ability: Ability, projectiles: &mut Vec<Projectile>) {
        if self.ammo_in_mag > 0 && !self.reloading {
            //Pistol
            if self.model == Model::Pistol && current_time() >= self.time_since_last_shot + 500 {
                self.time_since_last_shot = current_time();

                projectiles.push( Projectile {
                    x: match right {
                        true => x + (angle.cos() * 25.0),
                        false => x - (angle.cos() * 15.0),
                    },
                    y: match right {
                        true => y + (angle.sin() * 25.0),
                        false => y - (angle.sin() * 5.0),
                    },
                    w: 5.0,
                    h: 5.0,
                    right,
                    angle,
                    speed: match ability {
                        Ability::Engineer => 15.0,
                        _ => 12.0,
                    },
                    damage: self.damage,
                    fired_from: self.fired_from,
                    projectile_type: ProjectileType::Regular,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,

                });

                self.ammo_in_mag -= 1;

            } else if self.model == Model::Shotgun && current_time() >= self.time_since_last_shot + 1500 {
                let recoil_range = 0.2;

                let mut random_bytes: [u8; 12] = [0; 12];
                let mut negative_rnd: [u8; 12] = [0; 12];
                getrandom(&mut random_bytes).unwrap();
                getrandom(&mut negative_rnd).unwrap();

                self.time_since_last_shot = current_time();

                let mut shoot_several_bullets = |mut num_of_bullets: u8| {
                    while num_of_bullets > 0 {
                        let recoil: f32 =
                        if negative_rnd[num_of_bullets as usize - 1] <= 128 {
                            (random_bytes[num_of_bullets as usize - 1] as f32 / 255.0) * recoil_range

                        } else {
                            -(random_bytes[num_of_bullets as usize - 1] as f32 / 255.0) * recoil_range

                        };

                        num_of_bullets -= 1;

                        projectiles.push(
                            Projectile {
                                x: match right {
                                    true => x + (angle.cos() * 25.0 ) as f32,
                                    false => x - (angle.cos() * 15.0) as f32,

                                },
                                y: match right {
                                    true => y + (angle.sin() * 25.0) as f32,
                                    false => y - (angle.sin() * 15.0) as f32,

                                },
                                w: 5.0,
                                h: 5.0,
                                right,
                                angle: angle + recoil,
                                speed: match ability {
                                    Ability::Engineer => 13.75,
                                    _ => 11.0,
                                },
                                projectile_type: ProjectileType::Regular,
                                damage: self.damage,
                                fired_from: self.fired_from,
                                distance_traveled: 0.0,
                                max_distance: self.max_distance,

                            }
                        );

                    }
                };

                shoot_several_bullets(12);

                self.ammo_in_mag -= 1;

            } else if self.model == Model::Speedball && current_time() >= self.time_since_last_shot + 1500 {
                self.time_since_last_shot = current_time();

                projectiles.push( Projectile {
                    x: match right {
                        true => x + (angle.cos() * 25.0) as f32,
                        false => x - (angle.cos() * 15.0) as f32,
                    },
                    y: match right {
                        true => y + (angle.sin() * 25.0) as f32,
                        false => y - (angle.sin() * 15.0) as f32,
                    },
                    w: match ability {
                        Ability::Engineer => 6.25,
                        _ => 5.0,
                    },
                    h: match ability {
                        Ability::Engineer => 6.25,
                        _ => 5.0,
                    },
                    right,
                    angle,
                    speed: match ability {
                        Ability::Engineer => 0.31,
                        _ => 0.25,
                    },
                    projectile_type: ProjectileType::Speedball,
                    damage: self.damage,
                    fired_from: self.fired_from,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,

                });

                self.ammo_in_mag -= 1;
            } else if self.model == Model::BurstRifle {
                if self.shooting.is_some() {
                    if current_time() >= self.time_since_last_shot + 50 {
                        self.time_since_last_shot = current_time();

                        if self.projectiles_fired != 3 {
                            self.projectiles_fired += 1;
                            projectiles.push( Projectile {
                                x: match right {
                                    true => x + (angle.cos() * 25.0) as f32,
                                    false => x - (angle.cos() * 15.0) as f32,
                                },
                                y: match right {
                                    true => y + (angle.sin() * 25.0) as f32,
                                    false => y - (angle.sin() * 15.0) as f32,
                                },
                                w: 5.0,
                                h: 5.0,
                                right,
                                angle,
                                speed: match ability {
                                    Ability::Engineer => 15.0,
                                    _ => 12.0,
                                },
                                projectile_type: ProjectileType::Regular,
                                damage: self.damage,
                                fired_from: self.fired_from,
                                distance_traveled: 0.0,
                                max_distance: self.max_distance,

                            });
                            self.ammo_in_mag -= 1;


                        } else {
                            self.projectiles_fired = 0;
                            self.shooting = None;

                        }

                    }


                } else if current_time() >= self.time_since_last_shot + 500  {
                    self.time_since_last_shot = current_time();

                    self.shooting = Some((x, y, right, angle));
                    self.projectiles_fired += 1;

                    projectiles.push( Projectile {
                        x: match right {
                            true => x + (angle.cos() * 25.0) as f32,
                            false => x - (angle.cos() * 15.0) as f32,
                        },
                        y: match right {
                            true => y + (angle.sin() * 25.0) as f32,
                            false => y - (angle.sin() * 15.0) as f32,
                        },
                        w: 5.0,
                        h: 5.0,
                        right,
                        angle,
                        speed: match ability {
                            Ability::Engineer => 15.0,
                            _ => 12.0,
                        },
                        projectile_type: ProjectileType::Regular,
                        fired_from: self.fired_from,
                        damage: self.damage,
                        distance_traveled: 0.0,
                        max_distance: self.max_distance,

                    });

                    self.ammo_in_mag -= 1;

                }

            } else if self.model == Model::AssaultRifle && current_time() >= self.time_since_last_shot + 80 {
                self.time_since_last_shot = current_time();

                projectiles.push( Projectile {
                    x: match right {
                        true => x + (angle.cos() * 25.0),
                        false => x - (angle.cos() * 15.0),
                    },
                    y: match right {
                        true => y + (angle.sin() * 25.0),
                        false => y - (angle.sin() * 5.0),
                    },
                    w: 5.0,
                    h: 5.0,
                    right,
                    angle,
                    speed: match ability {
                        Ability::Engineer => 10.0,
                        _ => 8.0,
                    },
                    damage: self.damage,
                    fired_from: self.fired_from,
                    projectile_type: ProjectileType::Regular,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,

                });

                self.ammo_in_mag -= 1;

            }

        } else {
            // Reload if no ammo is available
            self.reload();

        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    None,
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW
}

#[derive(Copy, Clone, PartialEq)]
pub enum Ability {
    Stim,
    Phase,
    Wall,
    Engineer, //Should be default
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub color: Color,

    pub ability: Ability,
    // Your ability charges every tick, and then when it hits its minimum threshold you can use it, though waiting until it hits its maximum threshold may be better, as it will increase the ability's power/duration/whatever.
    // For example, the stim ability will run longer then longer you wait for its ability to charge
    pub ability_charge: u16,
    pub min_ability_charge: u16,
    pub max_ability_charge: u16,


    pub speed: f32,

    pub gun: Gun,

    pub health: u8,
    pub online: bool,
}

impl Player {
    pub fn new(color: Option<Color>, ability: Ability, health: u8, gun: Model, player_id: u8, online: bool, coords: [f32; 2]) -> Player {
        let mut random_color: [u8; 3] = [255; 3];

        getrandom(&mut random_color).unwrap();


        Player {
            x: coords[0],
            y: coords[1],
            direction: Direction::None,
            color:match color {
                Some(color) => color,
                //Random color
                None => Color::from_rgba(random_color[0], random_color[1], random_color[2], 255),
            },
            ability,
            ability_charge: match ability {
                Ability::Phase => 150,
                Ability::Stim => 150,
                Ability::Wall => 150,
                Ability::Engineer => 1,
            },
            min_ability_charge: match ability {
                // There's on average, 60 ticks per second, so 2.5 seconds need to pass to have enough charge to use your ability
                Ability::Phase => 150,
                Ability::Stim => 1,
                Ability::Wall => 150,
                Ability::Engineer => 1,
            },
            max_ability_charge: match ability {
                Ability::Phase => 300,
                Ability::Stim => 300,
                Ability::Wall => 150,
                Ability::Engineer => 1,
            },
            speed: 10.0,
            health,
            gun: Gun::new(gun, ability, player_id),
            online,
        }
    }

    pub fn offline() -> Player {
        Player::new(None, Ability::Phase, 0, Model::Pistol, 0, false, [0.0, 0.0])

    }

    pub fn use_ability(&mut self, map: &mut Map) {
        if self.health > 0 && self.ability_charge >= self.min_ability_charge{
            if self.ability == Ability::Phase  {

                let teleport_distance = 150.0;

                //I know this is ugly, it just lets a player move if it's movement wouldn't put it out of bounds
                match self.direction {
                   Direction::N=> {
                        if !out_of_bounds(self.x, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x, self.y - teleport_distance, 15.0, 15.0), 0) {
                            self.y -= teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::S=> {
                        if !out_of_bounds(self.x, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x, self.y + teleport_distance, 15.0, 15.0), 0){
                            self.y += teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::E=> {
                        if !out_of_bounds(self.x + teleport_distance, self.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y, 15.0, 15.0), 0){
                            self.x += teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::W=> {
                        if !out_of_bounds(self.x - teleport_distance, self.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x - teleport_distance, self.y, 15.0, 15.0), 0){
                            self.x -= teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::NE=> {
                        if !out_of_bounds(self.x + teleport_distance, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y - teleport_distance, 15.0, 15.0), 0){
                            self.x += teleport_distance;
                            self.y -= teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::NW=> {
                        if !out_of_bounds(self.x - teleport_distance, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x - teleport_distance, self.y - teleport_distance, 15.0, 15.0), 0) {
                            self.x -= teleport_distance;
                            self.y -= teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::SE=> {
                        if !out_of_bounds(self.x + teleport_distance, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y + teleport_distance, 15.0, 15.0), 0) {
                            self.x += teleport_distance;
                            self.y += teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                   Direction::SW=> {
                        if !out_of_bounds(self.x - teleport_distance, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(self.x - teleport_distance, self.y + teleport_distance, 15.0, 15.0), 0) {
                            self.x -= teleport_distance;
                            self.y += teleport_distance;
                            self.ability_charge -= 150;

                        }
                    },
                    _ => {},
                }


            } else if self.ability == Ability::Stim  {
                self.speed = 20.0;

                self.ability_charge -= 1;

            } else if self.ability == Ability::Wall {
                let x = match self.direction {
                   Direction::E | Direction::NE | Direction::SE=> self.x + 25.0,
                   Direction::W| Direction::NW | Direction::SW=> self.x - 25.0,
                    _ => self.x,

                };

                let y = match self.direction {
                   Direction::N | Direction::NE | Direction::NW=> self.y - 25.0,
                   Direction::S | Direction::SE | Direction::SW=> self.y + 25.0,
                    Direction::None => self.y - 25.0,
                    _ => self.y,

                };

                let w = match self.direction {
                   Direction::N | Direction::NE | Direction::NW | Direction::E | Direction::SE | Direction::SW=> 40.0,
                    _ => 20.0,

                };

                // Can't compare floats since the compiler complains, so I convert the width to a u8
                //See https://github.com/rust-lang/rust/issues/41620
                let h = match w as u8{
                    40 => 20.0,
                    _ => match self.direction {
                       Direction::W | Direction::NW | Direction::SW | Direction::NE | Direction::SE => 40.0,
                        _ => 20.0,
                    }
                };

                let color = Color::from_rgb(0, 255, 0);

                map.objects.push(MapObject::new(Rect::new(x, y, w, h), color, Some(100), false, true));

                self.ability_charge -= 150;
            }
        }

    }

    pub fn shoot(&mut self, right: bool, angle: f32, projectiles: &mut Vec<Projectile>) {

        if self.health > 0 {
            self.gun.shoot(self.x, self.y, right, angle, self.ability, projectiles);

        }

    }

}

#[derive(Copy, Clone)]
pub struct Controls {
    pub up: char,
    pub down: char,
    pub left: char,
    pub right: char,
    pub use_ability: char,
    pub reload: char,

}

impl Controls {
    pub fn new(up: char, down: char, left: char, right: char, use_ability: char, reload: char) -> Controls {
        Controls {
            up,
            down,
            left,
            right,
            use_ability,
            reload,
        }

    }

    pub fn default() -> Controls {
        Controls::new('w', 's', 'a', 'd', 'e', 'r')

    }

}


#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

}

impl Rect{
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect{
            x,
            y,
            w,
            h,

        }

    }


}

pub struct Point2 {
    pub x: f32,
    pub y: f32,

}


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32,
}

impl Color {
    /// Create a new `Color` from four `f32`'s in the range `[0.0-1.0]`
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    /// Create a new `Color` from four `u8`'s in the range `[0-255]`
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::from((r, g, b, a))
    }

    /// Create a new `Color` from three u8's in the range `[0-255]`,
    /// with the alpha component fixed to 255 (opaque)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::from((r, g, b))
    }

    /// Return a tuple of four `u8`'s in the range `[0-255]` with the `Color`'s
    /// components.
    pub fn to_rgba(self) -> (u8, u8, u8, u8) {
        self.into()
    }

    /// Return a tuple of three `u8`'s in the range `[0-255]` with the `Color`'s
    /// components.
    pub fn to_rgb(self) -> (u8, u8, u8) {
        self.into()
    }

    /// Convert a packed `u32` containing `0xRRGGBBAA` into a `Color`
    pub fn from_rgba_u32(c: u32) -> Color {
        let c = c.to_be_bytes();

        Color::from((c[0], c[1], c[2], c[3]))
    }

    /// Convert a packed `u32` containing `0x00RRGGBB` into a `Color`.
    /// This lets you do things like `Color::from_rgb_u32(0xCD09AA)` easily if you want.
    pub fn from_rgb_u32(c: u32) -> Color {
        let c = c.to_be_bytes();

        Color::from((c[1], c[2], c[3]))
    }

    /// Convert a `Color` into a packed `u32`, containing `0xRRGGBBAA` as bytes.
    pub fn to_rgba_u32(self) -> u32 {
        let (r, g, b, a): (u8, u8, u8, u8) = self.into();

        u32::from_be_bytes([r, g, b, a])
    }

    /// Convert a `Color` into a packed `u32`, containing `0x00RRGGBB` as bytes.
    pub fn to_rgb_u32(self) -> u32 {
        let (r, g, b, _a): (u8, u8, u8, u8) = self.into();

        u32::from_be_bytes([0, r, g, b])
    }
}

impl From<[f32; 4]> for Color {
    /// Turns an `[R, G, B, A] array of `f32`'s into a `Color` with no format changes.
    /// All inputs should be in the range `[0.0-1.0]`.
    fn from(val: [f32; 4]) -> Self {
        println!("array");
        Color::new(val[0], val[1], val[2], val[3])
    }
}


impl From<(u8, u8, u8, u8)> for Color {
    /// Convert a `(R, G, B, A)` tuple of `u8`'s in the range `[0-255]` into a `Color`
    fn from(val: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = val;
        let rf = (f32::from(r)) / 255.0;
        let gf = (f32::from(g)) / 255.0;
        let bf = (f32::from(b)) / 255.0;
        let af = (f32::from(a)) / 255.0;
        Color::new(rf, gf, bf, af)
    }
}

impl From<(u8, u8, u8)> for Color {
    /// Convert a `(R, G, B)` tuple of `u8`'s in the range `[0-255]` into a `Color`,
    /// with a value of 255 for the alpha element (i.e., no transparency.)
    fn from(val: (u8, u8, u8)) -> Self {
        let (r, g, b) = val;
        Color::from((r, g, b, 255))
    }
}

impl From<(f32, f32, f32)> for Color {
    /// Convert a `(R, G, B)` tuple of `f32`'s in the range `[0.0-1.0]` into a `Color`,
    /// with a value of 1.0 to for the alpha element (ie, no transparency.)
    fn from(val: (f32, f32, f32)) -> Self {
        let (r, g, b) = val;
        Color::new(r, g, b, 1.0)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    /// Convert a `(R, G, B, A)` tuple of `f32`'s in the range `[0.0-1.0]` into a `Color`
    fn from(val: (f32, f32, f32, f32)) -> Self {
        let (r, g, b, a) = val;
        Color::new(r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8, u8) {
    /// Convert a `Color` into a `(R, G, B, A)` tuple of `u8`'s in the range of `[0-255]`.
    fn from(color: Color) -> Self {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let a = (color.a * 255.0) as u8;
        (r, g, b, a)
    }
}

impl From<Color> for (u8, u8, u8) {
    /// Convert a `Color` into a `(R, G, B)` tuple of `u8`'s in the range of `[0-255]`,
    /// ignoring the alpha term.
    fn from(color: Color) -> Self {
        let (r, g, b, _) = color.into();
        (r, g, b)
    }
}
