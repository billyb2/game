use generational_arena::*;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::math::Vec2;

#[derive(Component, RenderResources, TypeUuid)]
#[uuid = "463e4b8a-af55-4fc2-ba9f-4c88b063ba12"]
pub struct Lights {
    pub value: [Vec2; 32],
}

impl Lights {
    pub const fn new() -> Self {
        Lights {
            value: [Vec2::ZERO; 32],

        }

    }
}


#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct LightHandle(Index);

impl LightHandle {
    fn new(index: Index) -> Self {
        LightHandle(index)
    }

    fn to_index(&self) -> Index {
        self.0

    }

}

// LightsResource and the Lights struct need to remain synced constantly, since the latter is to make GLSL happy and the former is for Rust
pub struct LightsResource {
    positions: Arena<Vec2>,

}


impl LightsResource {
    pub fn new() -> Self {
        Self {
            positions: Arena::with_capacity(32),
        }
    }

    pub fn add_light(&mut self, position: Vec2) -> LightHandle {
        LightHandle::new(self.positions.insert(position))

    }

    /// Make sure to remove or change the LightHandle for the entity as well
    pub fn remove_light(&mut self, handle: &LightHandle) {
        self.positions.remove(handle.to_index());

    }

    // Copies every used position into the slice, returning the number of bytes written
    pub fn copy_pos_into_slice(&self, slice: &mut [Vec2]) -> usize {
        let mut num_bytes_written: usize = 0;

        slice.iter_mut().zip(self.positions.iter()).for_each(|(dst, (_index, pos))| {
            *dst = *pos;

            num_bytes_written += 1;

        });

        num_bytes_written

    }

    pub fn len(&self) -> usize {
        self.positions.len()

    }

    pub fn modify_light_pos(&mut self, handle: &LightHandle) -> Option<&mut Vec2> {
       self.positions.get_mut(handle.to_index())

    }

    pub fn light_in_use(&self, handle: &LightHandle) -> bool {
        self.positions.contains(handle.to_index())

    }
}

#[derive(Component, RenderResources, TypeUuid)]
#[uuid = "463e4b8a-af55-3ac2-ba9f-4c88b063ba12"]
pub struct AmbientLightLevel {
    pub value: f32,
}

#[derive(Component, RenderResources, TypeUuid)]
#[uuid = "463f4b1a-af55-4fc2-bb9f-4c88b063ba12"]
pub struct NumLights {
    pub value: i32,
}

#[derive(Component, Clone)]
pub struct LightDestructionTimer(pub Timer);
