#![feature(const_fn_floating_point_arithmetic)]
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
    pub used_bullet: Handle<ColorMaterial>,

    pub shield_cell: Handle<ColorMaterial>,
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


pub struct PlayerToSpectate(pub u8);

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

pub trait Logs {
    fn new() -> Self;
    fn is_full(&self) -> bool;
    fn first_mut(&mut self) -> Option<&mut GameLog>;
    fn push_unchecked(&mut self, element: GameLog);
    fn retain<F>(&mut self, f: F)
        where F: FnMut(&mut GameLog) -> bool;
    fn iter(&self) -> std::slice::Iter<'_, GameLog>;

}

#[derive(Component)]
pub struct GameLogs(pub ArrayVec<GameLog, 10>);


#[derive(Component)]
pub struct ChatLogs(pub ArrayVec<GameLog, 10>);

impl Logs for GameLogs {
    fn new() -> Self {
        GameLogs(ArrayVec::new())

    }

    fn is_full(&self) -> bool {
        self.0.is_full()

    }

    fn first_mut(&mut self) -> Option<&mut GameLog> {
        self.0.first_mut()
    }

    fn push_unchecked(&mut self, element: GameLog) {
        unsafe { self.0.push_unchecked(element) }

    }

    fn retain<F>(&mut self, f: F)
        where F: FnMut(&mut GameLog) -> bool {
        self.0.retain(f)

    }

    fn iter(&self) -> std::slice::Iter<'_, GameLog> {
        self.0.iter()
    }
}

impl Logs for ChatLogs {
    fn new() -> Self {
        ChatLogs(ArrayVec::new())

    }

    fn is_full(&self) -> bool {
        self.0.is_full()

    }

    fn first_mut(&mut self) -> Option<&mut GameLog> {
        self.0.first_mut()
    }

    fn push_unchecked(&mut self, element: GameLog) {
        unsafe { self.0.push_unchecked(element) }

    }

    fn retain<F>(&mut self, f: F)
        where F: FnMut(&mut GameLog) -> bool {
        self.0.retain(f)

    }

    fn iter(&self) -> std::slice::Iter<'_, GameLog> {
        self.0.iter()
    }
}

#[derive(Clone)]
pub struct GameLog {
    pub text: TextSection,
    pub timer: Timer,

}

impl GameLog {
    pub fn new(text: String, size: Option<f32>, text_screen_time: f32, asset_server: &AssetServer) -> Self {
        GameLog {
            text: TextSection {
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    // The text size becomes smaller as the actual text becomes larger, so that it will always fit on the screen
                    font_size: size.unwrap_or(35.0 * (20.0 / text.len() as f32)),
                    color: Color::WHITE,
                },
                value: text,
            },
            timer: Timer::from_seconds(text_screen_time, false),

        }
        
    }
}
