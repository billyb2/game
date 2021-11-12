use arrayvec::ArrayVec;

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

    // In a perfect world, I could just do this, but map isn't const
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

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
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
    pub talk: KeyCode,

}

#[derive(Component, Debug, PartialEq)]
pub struct SelectedKeyButton(pub Option<KeyBindingButtons>);


#[derive(Component, Debug, PartialEq)]
pub enum KeyBindingButtons {
    Up,
    Down,
    Left,
    Right,
    UseAbility,
    Reload,
    ShowScore,
    Melee,
    Talk,
}

// The UUID is just random
#[derive(Component, RenderResources, TypeUuid)]
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

#[derive(Component, RenderResources, TypeUuid)]
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

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct AmmoText;
#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct AbilityChargeText;
#[derive(Component)]
pub struct GameLogText;
#[derive(Component)]
pub struct ChatText;
#[derive(Component)]
pub struct ChatLogText;
#[derive(Component)]
pub struct NameText;


#[derive(Component)]
pub struct IpText;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct ChampionText;

#[derive(Component)]
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
    pub arrow: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct AssetsLoading {
    pub vertex: Handle<Shader>,
    pub player_frag: Handle<Shader>,
    pub lighting_frag: Handle<Shader>,
    pub loaded: bool,
}

// The mouse's position in world coordinates
pub struct MousePosition(pub Vec2);

#[derive(Component, RenderResources, TypeUuid)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct ShaderMousePosition {
    pub value: Vec2,
}

// LightsResource and the Lights struct need to remain synced constantly, since the latter is to make GLSL happy and the former is for Rust
pub struct LightsResource(ArrayVec<Vec2, 32>);

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


#[derive(Component)]
pub struct LightHandle(usize);

impl LightHandle {
    fn new(index: usize) -> Self {
        LightHandle(index)
    }

    fn to_index(&self) -> usize {
        self.0
    }
}

impl LightsResource {
    pub fn new() -> Self {
        LightsResource(ArrayVec::new())
    }

    pub fn add_light(&mut self, coords: Vec2) -> LightHandle {
        self.0.push(coords);
        LightHandle::new(self.0.len() - 1)

    }

    pub fn remove_light(&mut self, handle: &LightHandle) {
        self.0.swap_remove(handle.to_index());

    }

    pub fn as_slice(&self) -> &[Vec2] {
        self.0.as_slice()

    }

    pub fn len(&self) -> usize {
        self.0.len()

    }

    pub fn modify_light(&mut self, handle: &LightHandle) -> Option<&mut Vec2> {
        self.0.get_mut(handle.to_index())

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

#[derive(Component, RenderResources, TypeUuid)]
#[uuid = "463e4c8b-d555-4fc2-ba9f-4c880063ba92"]
pub struct WindowSize {
    pub value: Vec2,
}

#[derive(Component, RenderResources, TypeUuid)]
#[uuid = "463e4c8b-d554-4fc2-bc9f-4c881163ba92"]
pub struct Alpha {
    pub value: f32,
}

pub struct PlayerToSpectate(pub u8);
