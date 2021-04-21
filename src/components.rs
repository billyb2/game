use bevy::core::Timer;

#[derive(Debug, PartialEq)]
pub enum MovementType {
    // Stop moving after this frame passes
    // Used for things like player movement
    SingleFrame,
    StopAfterDistance(f32),

}

#[derive(Debug, PartialEq)]
pub struct DistanceTraveled(pub f32);

#[derive(Debug, PartialEq)]
pub struct RequestedMovement {
    // Angle is in radians
    pub angle: f32,
    pub speed: f32,

}

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

#[derive(Debug, PartialEq)]
pub struct PlayerID(pub u8);



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
pub struct MaxAmmo(pub u8);

#[derive(Clone, Debug)]
pub struct ReloadTime(pub f32);

#[derive(Clone, Debug)]
pub struct ReloadEvent;
