use arrayvec::ArrayVec;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::Camera;
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
pub struct LightHandle(usize);

impl LightHandle {
    fn new(index: usize) -> Self {
        LightHandle(index)
    }

    fn to_index(&self) -> usize {
        self.0
    }
}

// LightsResource and the Lights struct need to remain synced constantly, since the latter is to make GLSL happy and the former is for Rust
// TODO: Have some form of mechanism for automatically getting rid of memory fragmentation within the array, so that we canjust return a simple slice
pub struct LightsResource {
    positions: [Vec2; 32],
    used_indexes: ArrayVec<LightHandle, 32>,
    available_indexes: ArrayVec<LightHandle, 32>,

}


impl LightsResource {
    pub fn new() -> Self {
        let mut available_indexes = ArrayVec::new();

        (0..31).into_iter().for_each(|i| {
            available_indexes.push(LightHandle::new(i));

        });

        Self {
            positions: [Vec2::ZERO; 32],
            used_indexes: ArrayVec::new(),
            available_indexes,
        }
    }

    pub fn add_light(&mut self, position: Vec2) -> LightHandle {
        // Move a value from available indexes to used indexes
        let handle = self.available_indexes.pop().unwrap();

        self.used_indexes.push(handle);
        self.positions[handle.to_index()] = position;

        handle

    }

    /// Make sure to remove or change the LightHandle for the entity as well
    pub fn remove_light(&mut self, handle: &LightHandle) {
        // Finds the index of the handle, then removes it
        let index_of_handle = self.used_indexes.iter().position(|u_handle| u_handle == handle).unwrap();
        self.available_indexes.push(self.used_indexes.remove(index_of_handle));

    }

    // Copies every used position into the slice, returning the number of bytes written
    pub fn copy_pos_into_slice(&self, slice: &mut [Vec2]) -> usize {
        let mut num_bytes_written: usize = 0;

        slice.iter_mut().zip(self.used_indexes.iter()).for_each(|(dest, index)| {
            *dest = unsafe { *self.positions.get_unchecked(index.to_index()) };

            num_bytes_written += 1;

        });

        num_bytes_written

    }

    pub fn len(&self) -> usize {
        self.used_indexes.len()

    }

    pub fn modify_light_pos(&mut self, handle: &LightHandle) -> Option<&mut Vec2> {
       match self.used_indexes.contains(&handle) {
            true => Some(self.positions.get_mut(handle.to_index()).unwrap()),
            false => None,
        }

    }
    
    pub fn light_in_use(&self, handle: &LightHandle) -> bool {
        self.used_indexes.contains(&handle)

    }

    pub fn calc_shader_light_pos(&mut self, translation: Vec3, camera: &Camera, camera_transform: &GlobalTransform, windows: &Windows, wnd_size: Vec2, light_handle: &LightHandle) {
        if let Some(mut coords) = camera.world_to_screen(&windows, camera_transform, translation) {
            // Adjust the coordinates based off Bevy's camera system
            coords.y = wnd_size.y - coords.y;

            if let Some(light_coords) = self.modify_light_pos(light_handle) {
                *light_coords = coords / wnd_size;

            }
        }

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
