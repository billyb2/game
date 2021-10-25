#![feature(destructuring_assignment)]
#![feature(variant_count)]
#![feature(const_fn_floating_point_arithmetic)]

#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

pub mod player_attr;
#[cfg(feature = "graphics")]
pub mod graphics;

use bevy::core::Timer;
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::geometry::ColliderHandle;

#[cfg(feature = "graphics")]
pub use graphics::*;

pub use player_attr::*;

#[derive(Component)]
pub struct DistanceTraveled(pub f32);

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Speed(pub f32);

#[derive(Component, Copy, Clone)]
pub struct Health(pub f32);

#[derive(Component, Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Damage(pub f32);

#[derive(Component, Copy, Clone, PartialEq)]
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

#[derive(Component)]
pub struct DespawnWhenDead {
    pub health: f32,
    pub coords: Vec2,

}

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

pub struct SlowedDown(pub Timer);

#[derive(Component)]
pub struct GameRelated;

#[derive(Component)]
pub struct ResScale(pub f32);

pub struct NumOfBots(pub u8);

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
    InGame,
    Settings,
    CustomizeGame,
    DownloadMapMenu,

}

#[cfg(not(feature = "graphics"))]
pub struct Alpha {
    pub value: f32,
}

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
