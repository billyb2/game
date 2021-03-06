#![feature(variant_count)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

pub mod player_attr;

use std::time::Instant;

use bevy::core::Timer;
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::geometry::ColliderHandle;

pub use player_attr::*;
#[cfg(feature = "graphics")]
pub use ui_graphics::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Component)]
pub struct DistanceTraveled(pub f32);

#[derive(Component, Clone, PartialEq)]
pub struct Speed(pub f32);

#[derive(Component, Copy, Clone)]
pub struct Health(pub f32);

#[derive(Component, Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Damage(pub f32);

#[derive(Component, Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlayerID(pub u8);

// Projectile stuff
// The value inside the struct is the player id of the person who shot it, so that player is immune from collisions
#[derive(Component, PartialEq)]
pub struct ProjectileIdent(pub u8);

// Gun stuff
#[derive(Component)]
pub struct TimeSinceLastShot(pub Timer);

#[derive(Component)]
pub struct CanMelee(pub Timer);

#[derive(Component)]
pub struct DestructionTimer(pub Timer);

#[derive(Component)]
pub struct DashingInfo {
    pub time_till_can_dash: Timer,
    pub time_till_stop_dash: Timer,
    pub dashing: bool,
}

#[derive(Component)]
pub struct TimeSinceStartReload {
    pub timer: Timer,
    pub reloading: bool,
    pub fast_reload: bool,

}

#[derive(Component)]
pub struct AmmoInMag(pub u8);

#[derive(Component)]
pub struct Bursting(pub bool);

#[derive(Component)]
pub struct MaxAmmo(pub u8);

#[derive(Component)]
pub struct MaxDistance(pub f32);

#[derive(Component)]
pub struct RecoilRange(pub f32);

#[derive(Component)]
pub struct ReloadTime(pub f32);

#[derive(Component)]
pub struct ReloadEvent(pub u8);

#[derive(Component)]
pub struct AbilityEvent(pub u8);

// A timer for when a player wins a match, so the game knows when to return to the main menu
#[derive(Component)]
pub struct PlayerContinueTimer(pub Timer);

#[derive(Component)]
pub struct DamageTextTimer(pub Timer);

#[derive(Component, Copy, Clone, PartialEq)]
pub enum InGameSettings {
    Settings,
    Customize
}

#[derive(Component)]
pub struct CustomizeHelpText;

#[derive(Component)]
pub struct GameRelated;

#[derive(Component)]
pub struct ResScale(pub f32);

pub struct NumOfBots(pub u8);

pub struct BotAlgs {
    pub current_index: usize,
    pub algs: Vec<(String, Vec<u8>)>,
}

#[derive(Component)]
pub struct LogEvent(pub String);

#[derive(Component)]
pub struct ChatEvent(pub String);

pub trait LogEv {
    fn inner(&self) -> &String;
}

impl LogEv for LogEvent {
    fn inner(&self) -> &String {
        &self.0

    }
}

impl LogEv for ChatEvent {
    fn inner(&self) -> &String {
        &self.0

    }
}

pub struct DeathEvent(pub u8);

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Connecting,
    MainMenu,
    GameMenu,
    ContinuePlaying,
    CustomizePlayerMenu,
    BotBattleMenu,
    InGame,
    Settings,
    CustomizeGame,
    DownloadMapMenu,

}

#[cfg(not(feature = "graphics"))]
#[derive(Component)]
pub struct Alpha {
    pub value: f32,
}

pub struct Typing(pub bool);

#[derive(Component)]
pub struct RigidBodyHandleWrapper(pub RigidBodyHandle);

#[derive(Component)]
pub struct ColliderHandleWrapper(pub ColliderHandle);


#[derive(Component, Clone, PartialEq)]
pub struct Size(pub Vec2);

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Self(Vec2::new(w, h))
    }

}

// A resource stating whether or not the player is hosting
pub struct Hosting(pub bool);

pub trait WriteToStringSlice {
    fn str_write(&mut self, new_str: &str);
}

impl WriteToStringSlice for String {
    fn str_write(&mut self, new_str: &str) {
        self.clear();
        self.push_str(new_str);

        debug_assert_eq!(self.as_str(), new_str);
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

#[cfg(target_arch = "x86_64")]
pub fn log(s: &str) {
    println!("{s}");
} 


/// The calculated tick rate
pub struct TickRate {
    pub last_tick: Instant,

}
