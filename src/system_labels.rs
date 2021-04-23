use bevy::prelude::*;

 //Anything that moves an object
#[derive(SystemLabel, Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct InputFromPlayer;

#[derive(SystemLabel, Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct NeedsGraphics;
