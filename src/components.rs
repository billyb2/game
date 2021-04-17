use bevy::prelude::*;

#[derive(Debug, PartialEq)]
pub enum MovementType {
    // Stop moving after this frame passes
    // Used for things like player movement
    SingleFrame,
    StopAfterDistance(Vec2),

}

#[derive(Debug, PartialEq)]
pub struct DistanceTraveled(pub Vec2);

#[derive(Debug, PartialEq)]
pub struct RequestedMovement(pub Vec3);

#[derive(Debug, PartialEq)]
pub struct Health(pub u8);

#[derive(Debug, PartialEq)]
pub struct PlayerID(pub u8);
