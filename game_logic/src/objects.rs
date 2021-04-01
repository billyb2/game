use crate::map::{Map, MapObject};
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub right: bool,
    pub angle: f32,
    pub speed: f32,
    pub damage: u8,
    // 0 is just a regular bullet
    // 1 is a bullet that speeds up over time
    pub fired_from: u8, // Fixes a game-breaking bug where the bots get killed by their own bullets
    pub projectile_type: u8,
    pub distance_traveled: f32,
    pub max_distance: f32,

}

#[derive(Copy, Clone)]
pub struct Gun {
    // Once again, storing the gun model as an int since it makes it fast and easy to deal with
    // 0 is the pistol
    // 1 is the shotgun
    // 2 is the speedball
    // 3 is the burst rifle
    // 4 is the assault rifle
    pub model: u8,
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
    pub fn new(model: u8, ability: u8, player_fired_from: u8) -> Gun {
        let mut gun = Gun {
            model,
            // The time since the last shot is set as 0 so that you can start shooting as the start of the game
            time_since_last_shot: 0,
            time_since_start_reload: 0,
            reloading: false,
            reload_time: match model {
                0 => 2000,
                1 => 5000,
                2 => 3000,
                3 => 3250,
                4 => 3750,
                _ => 3000,
            },
            // Some guns have special shooting behaviors that last over the course of mutliple ticks, which shooting and projectiles_fired take advantage of
            shooting: None,
            projectiles_fired: 0,
            ammo_in_mag: match model {
                0 => 16,
                1 => 8,
                2 => 6,
                3 => 21,
                4 => 25,
                _ => 30,

            },
            max_ammo: match model {
                0 => 16,
                1 => 8,
                2 => 6,
                3 => 21,
                4 => 25,
                _ => 30,

            },
            damage: match model {
                0 => 45,
                1 => 25,
                2 => 1,
                3 => 13,
                4 => 15,
                _ => 100,
            },
            fired_from: player_fired_from,
            max_distance: match model {
                0 => 900.0,
                1 => 300.0,
                2 => 3000.0,
                3 => 1000.0,
                4 => 1000.0,
                _ => 900.0,
            }

        };

        // The engineer ability can reload in half the time
        if ability == 3 {
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

    pub fn shoot (&mut self, x: f32, y: f32, right: bool, angle: f32, ability: u8, projectiles: &mut Vec<Projectile>) {
        if self.ammo_in_mag > 0 && !self.reloading {
            //Pistol
            if self.model == 0 && current_time() >= self.time_since_last_shot + 500 {
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
                        3 => 15.0,
                        _ => 12.0,
                    },
                    damage: self.damage,
                    fired_from: self.fired_from,
                    projectile_type: 0,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,

                });

                self.ammo_in_mag -= 1;

            } else if self.model == 1 && current_time() >= self.time_since_last_shot + 1500 {
                let mut rng = thread_rng();
                let recoil_range: f32 = 0.2;

                self.time_since_last_shot = current_time();

                let mut shoot_several_bullets = |mut num_of_bullets: u8| {
                    while num_of_bullets > 0 {
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
                                angle: angle + rng.gen_range(-recoil_range..recoil_range),
                                speed: match ability {
                                    3 => 13.75,
                                    _ => 11.0,
                                },
                                projectile_type: 0,
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

            } else if self.model == 2 && current_time() >= self.time_since_last_shot + 1500 {
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
                        3 => 6.25,
                        _ => 5.0,
                    },
                    h: match ability {
                        3 => 6.25,
                        _ => 5.0,
                    },
                    right,
                    angle,
                    speed: match ability {
                        3 => 0.31,
                        _ => 0.25,
                    },
                    projectile_type: 1,
                    damage: self.damage,
                    fired_from: self.fired_from,
                    distance_traveled: 0.0,
                    max_distance: self.max_distance,

                });

                self.ammo_in_mag -= 1;
            } else if self.model == 3 {
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
                                    3 => 15.0,
                                    _ => 12.0,
                                },
                                projectile_type: 0,
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
                            3 => 15.0,
                            _ => 12.0,
                        },
                        projectile_type: 0,
                        fired_from: self.fired_from,
                        damage: self.damage,
                        distance_traveled: 0.0,
                        max_distance: self.max_distance,

                    });

                    self.ammo_in_mag -= 1;

                }

            } else if self.model == 4 && current_time() >= self.time_since_last_shot + 80 {
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
                        3 => 10.0,
                        _ => 8.0,
                    },
                    damage: self.damage,
                    fired_from: self.fired_from,
                    projectile_type: 0,
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

#[derive(Copy, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    // 0 not moving
    // 1 N
    // 2 S
    // 3 E
    // 4 W
    // 5 NE
    // 6 NW
    // 7 SE
    // 8 SW

    pub direction: u8,
    pub color: Color,

    // The ability is stored as an int in order to allow for faster code
    // If it was stored as a string, then the players couldn't be stored in an array, causing more variable memory usage
    // 0 is phase
    // 1 is stim
    // 2 is the wall
    // 3 is the engineer (should probably be default)
    pub ability: u8,
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
    pub fn new(color: Option<Color>, ability: u8, health: u8, gun: u8, player_id: u8, online: bool, x: f32, y: f32) -> Player {
        let mut rng = thread_rng();

        Player {
            x,
            y,
            direction: 0,
            color:match color {
                Some(color) => color,
                //Random color
                None => Color::from_rgba(rng.gen_range(100..255), rng.gen_range(100..255), rng.gen_range(100..255), 255),
            },
            ability,
            ability_charge: match ability {
                0 => 150,
                1 => 150,
                2 => 150,
                3 => 1,
                _ => 150,
            },
            min_ability_charge: match ability {
                // There's on average, 60 ticks per second, so 2.5 seconds need to pass to have enough charge to use your ability
                0 => 150,
                1 => 1,
                2 => 150,
                3 => 1,
                _ => 150,
            },
            max_ability_charge: match ability {
                0 => 300,
                1 => 300,
                2 => 150,
                3 => 1,
                _ => 300,
            },
            speed: 10.0,
            health,
            gun: Gun::new(gun, ability, player_id),
            online,
        }
    }

    pub fn use_ability(&mut self, map: &mut Map) {
        if self.health > 0 && self.ability_charge >= self.min_ability_charge{
            if self.ability == 0  {

                let teleport_distance = 250.0;

                //I know this is ugly, it just lets a player move if it's movement wouldn't put it out of bounds
                match self.direction {
                    1 => {
                        if !out_of_bounds(self.x, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x, self.y - teleport_distance, 15.0, 15.0), 0) {
                            self.y -= teleport_distance;

                        }
                    },
                    2 => {
                        if !out_of_bounds(self.x, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x, self.y + teleport_distance, 15.0, 15.0), 0){
                            self.y += teleport_distance;

                        }
                    },
                    3=> {
                        if !out_of_bounds(self.x + teleport_distance, self.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y, 15.0, 15.0), 0){
                            self.x += teleport_distance;

                        }
                    },
                    4 => {
                        if !out_of_bounds(self.x - teleport_distance, self.y, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x - teleport_distance, self.y, 15.0, 15.0), 0){
                            self.x -= teleport_distance;

                        }
                    },
                    5 => {
                        if !out_of_bounds(self.x + teleport_distance, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y - teleport_distance, 15.0, 15.0), 0){
                            self.x += teleport_distance;
                            self.y -= teleport_distance;

                        }
                    },
                    6 => {
                        if !out_of_bounds(self.x - teleport_distance, self.y - teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x - teleport_distance, self.y - teleport_distance, 15.0, 15.0), 0) {
                            self.x -= teleport_distance;
                            self.y -= teleport_distance;

                        }
                    },
                    7 => {
                        if !out_of_bounds(self.x + teleport_distance, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) && !map.collision(&Rect::new(self.x + teleport_distance, self.y + teleport_distance, 15.0, 15.0), 0) {
                            self.x += teleport_distance;
                            self.y += teleport_distance;

                        }
                    },
                    8 => {
                        if !out_of_bounds(self.x - teleport_distance, self.y + teleport_distance, 15.0, 15.0, map.width, map.height) &&  !map.collision(&Rect::new(self.x - teleport_distance, self.y + teleport_distance, 15.0, 15.0), 0) {
                            self.x -= teleport_distance;
                            self.y += teleport_distance;

                        }
                    },
                    _ => {},
                }

                self.ability_charge -= 150;

            } else if self.ability == 1  {
                self.speed = 20.0;

                self.ability_charge -= 1;

            } else if self.ability == 2 {
                let x = match self.direction {
                    3 | 5 | 7 => self.x + 25.0,
                    4 | 6 | 8=> self.x - 25.0,
                    _ => self.x,

                };

                let y = match self.direction {
                    1 | 5 | 6=> self.y - 25.0,
                    2 | 7 | 8 => self.y + 25.0,
                    0 => self.y - 25.0,
                    _ => self.y,

                };

                let w = match self.direction {
                    1 | 5 | 6 | 2 | 7 | 8 => 40.0,
                    _ => 20.0,

                };

                // Can't compare floats since the compiler complains, so I convert the width to a u8
                //See https://github.com/rust-lang/rust/issues/41620
                let h = match w as u8{
                    40 => 20.0,
                    _ => match self.direction {
                        3 | 5 | 7 | 4 | 6 | 8 => 40.0,
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

pub fn current_time() -> u128 {
    // Returns the time in Unix Time (the number of milliseconds since 1970)
    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    //Return the current time
    time
}

pub fn out_of_bounds(x: f32, y: f32, w: f32, h: f32, world_width: f32, world_height: f32,) -> bool {
    //Basically, if the rectangle is out of bounds, it returns true, if not it'll return false
    {
        x + w >= world_width ||
        x <= 0.0 ||
        y +h >= world_height ||
        y <= 0.0
    }

}


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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


#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
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

impl From<[f32; 4]> for Color {
    /// Turns an `[R, G, B, A] array of `f32`'s into a `Color` with no format changes.
    /// All inputs should be in the range `[0.0-1.0]`.
    fn from(val: [f32; 4]) -> Self {
        Color::new(val[0], val[1], val[2], val[3])
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
