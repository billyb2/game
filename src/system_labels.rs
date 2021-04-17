use bevy::prelude::*;

 //Anything that moves an object
#[derive(SystemLabel, Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct MoveReq;
