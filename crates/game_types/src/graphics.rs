use bevy::prelude::*;
use bevy::math::const_vec3;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;

use serde::{Deserialize, Serialize};

pub const fn u8_to_color(value: [u8; 3]) -> [f32; 3] {
    [
        value[0] as f32 / 255.0,
        value[1] as f32 / 255.0,
        value[2] as f32 / 255.0,
    ]

    // In a perfect world, I could just do this
    // value.map(|v| v as f32 / 255.0)

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
pub struct HealthText;

pub struct AbilityChargeText;
pub struct GameLogText;

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

pub struct PlayerToSpectate(pub u8);