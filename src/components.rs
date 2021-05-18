#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use bevy::core::Timer;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum MovementType {
    // Stop moving after this frame passes
    // Used for things like player movement
    SingleFrame,
    StopAfterDistance(f32),

}

#[derive(Debug, PartialEq)]
pub struct DistanceTraveled(pub f32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestedMovement {
    // Angle is in radians
    pub angle: f32,
    pub speed: f32,
    // How much damage the bullets do to players or the environment

}

#[derive(Clone, Debug, PartialEq)]
pub struct Speed(pub f32);

impl RequestedMovement {
    pub fn new(angle: f32, speed: f32) -> RequestedMovement {
        RequestedMovement {
            angle,
            speed,

        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Health(pub u8);

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Damage(pub u8);

#[derive(Debug, PartialEq)]
pub struct PlayerID(pub u8);

// Projectile stuff
// The value inside the struct is the player id of the person who shot it, so that player is immune from collisions
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectileIdent(pub u8);

// Gun stuff
#[derive(Clone, Debug)]
pub struct TimeSinceLastShot(pub Timer);

#[derive(Clone, Debug)]
pub struct TimeSinceStartReload {
    pub timer: Timer,
    pub reloading: bool,

}

#[derive(Clone, Debug)]
pub struct AmmoInMag(pub u8);

#[derive(Clone, Debug)]
pub struct Bursting(pub bool);

#[derive(Clone, Debug)]
pub struct MaxAmmo(pub u8);

#[derive(Clone, Debug)]
pub struct MaxDistance(pub f32);

#[derive(Clone, Debug)]
pub struct RecoilRange(pub f32);

#[derive(Clone, Debug)]
pub struct ReloadTime(pub f32);

#[derive(Clone, Debug)]
pub struct ReloadEvent;

#[derive(Clone, Debug)]
pub struct AbilityEvent(pub u8);
