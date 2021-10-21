#![feature(destructuring_assignment)]
#![feature(variant_count)]
#![feature(const_fn_floating_point_arithmetic)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

pub mod player_attr;
#[cfg(feature = "graphics")]
pub mod graphics;

use bevy::core::Timer;
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

#[cfg(feature = "graphics")]
pub use graphics::*;

pub use player_attr::*;

pub struct DistanceTraveled(pub f32);

#[derive(Clone, Debug, PartialEq)]
pub struct Speed(pub f32);

#[derive(Copy, Clone)]
pub struct Health(pub f32);

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Damage(pub f32);

#[derive(Copy, Clone, PartialEq)]
pub struct PlayerID(pub u8);

// Projectile stuff
// The value inside the struct is the player id of the person who shot it, so that player is immune from collisions
#[derive(PartialEq)]
pub struct ProjectileIdent(pub u8);

// Gun stuff
pub struct TimeSinceLastShot(pub Timer);

pub struct CanMelee(pub Timer);

pub struct DestructionTimer(pub Timer);

pub struct DashingInfo {
    pub time_till_can_dash: Timer,
    pub time_till_stop_dash: Timer,
    pub dashing: bool,
}

pub struct TimeSinceStartReload {
    pub timer: Timer,
    pub reloading: bool,

}

pub struct AmmoInMag(pub u8);

pub struct Bursting(pub bool);

pub struct MaxAmmo(pub u8);

pub struct MaxDistance(pub f32);

pub struct RecoilRange(pub f32);

pub struct ReloadTime(pub f32);

pub struct ReloadEvent(pub u8);

pub struct AbilityEvent(pub u8);

pub struct DespawnWhenDead {
    pub health: f32,
    pub coords: Vec2,

}

// A timer for when a player wins a match, so the game knows when to return to the main menu
pub struct PlayerContinueTimer(pub Timer);

pub struct DamageTextTimer(pub Timer);

#[derive(Copy, Clone, PartialEq)]
pub enum InGameSettings {
    Settings,
    Customize
}

pub struct CustomizeHelpText;

pub struct SlowedDown(pub Timer);

pub struct GameRelated;

pub struct ResScale(pub f32);

pub struct NumOfBots(pub u8);

pub struct LogEvent(pub String);

pub struct DeathEvent(pub u8);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Connecting,
    MainMenu,
    GameMenu,
    ContinuePlaying,
    CustomizePlayerMenu,
    InGame,
    Settings,
    CustomizeGame,
    DownloadMapMenu,

}

#[cfg(not(feature = "graphics"))]
pub struct Alpha {
    pub value: f32,
}

