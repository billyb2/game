use bevy::prelude::*;
use bevy::math::const_vec3;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;

use serde::{Deserialize, Serialize};

pub const fn u8_to_color(value: [u8; 3]) -> [f32; 3] {
    let new_values: [f32; 3] = {
        let mut new_values: [f32; 3] = [0.0; 3];

        let mut i = 0;

        while i < value.len() {
            let mut v: f32 = value[i] as f32;
            v /= 255.0;

            new_values[i] = v;

            i += 1;

        }

        new_values

    };
    
    new_values
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,

}

pub struct GameMenuButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,

}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub use_ability: KeyCode,
    pub reload: KeyCode,
    pub show_score: KeyCode,
    pub dash: KeyCode,
    pub melee: KeyCode,

}

#[derive(Debug, PartialEq)]
pub struct SelectedKeyButton(pub Option<KeyBindingButtons>);


#[derive(Debug, PartialEq)]
pub enum KeyBindingButtons {
    Up,
    Down,
    Left,
    Right,
    UseAbility,
    Reload,
    ShowScore,
    Melee,
}

// The UUID is just random
#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-5c880063ba92"]
pub struct HelmetColor {
    pub value: Vec3,

}

impl HelmetColor {
    //TODO: this function is a great canidate for SIMD
    pub const fn new(value: [u8; 3]) -> Self {
        Self {
            value: const_vec3!(u8_to_color(value)),

        }

    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c881163ba92"]
pub struct InnerSuitColor {
    pub value: Vec3,

}

impl InnerSuitColor {
    pub const fn new(value: [u8; 3]) -> Self {
        Self {
            value: const_vec3!(u8_to_color(value)),

        }
    }
}

pub struct GameCamera;

pub struct AmmoText;
pub struct AbilityChargeText;
pub struct GameLogText;
pub struct HealthText;

pub struct IpText;

pub struct ScoreUI;

pub struct ChampionText;

pub struct NetConnStateText;

pub struct ProjectileMaterials {
    pub regular: Handle<ColorMaterial>,
    pub speedball: Handle<ColorMaterial>,
    pub engineer: Handle<ColorMaterial>,
    pub molotov: Handle<ColorMaterial>,
    pub molotov_fire: Handle<ColorMaterial>,
    pub molotov_liquid: Handle<ColorMaterial>,

    pub flamethrower1: Handle<ColorMaterial>,
    pub flamethrower2: Handle<ColorMaterial>,
    pub flamethrower3: Handle<ColorMaterial>,
    pub pulsewave: Handle<ColorMaterial>,
    pub beam: Handle<ColorMaterial>,
}

#[derive(Default)]
pub struct AssetsLoading {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
    pub loaded: bool,
}

// The mouse's position in world coordinates
pub struct MousePosition(pub Vec2);

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct ShaderMousePosition {
    pub value: Vec2,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c880063ba92"]
pub struct WindowSize {
    pub value: Vec2,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "463e4c8b-d554-4fc2-bc9f-4c881163ba92"]
pub struct Alpha {
    pub value: f32,
}
