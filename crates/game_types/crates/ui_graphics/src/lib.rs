#![feature(const_fn_floating_point_arithmetic)]
use arrayvec::ArrayVec;

use bevy::prelude::*;
use bevy::math::const_vec3;
use bevy::reflect::TypeUuid;

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
    pub normal: UiColor,
    pub hovered: UiColor,

}

pub struct GameMenuButtonMaterials {
    pub normal: UiColor,
    pub hovered: UiColor,

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
    pub regular: Handle<Image>,
    pub speedball: Handle<Image>,
    pub engineer: Handle<Image>,
    pub molotov: Handle<Image>,
    pub molotov_fire: Handle<Image>,
    pub molotov_liquid: Handle<Image>,
    pub flamethrower1: Handle<Image>,
    pub flamethrower2: Handle<Image>,
    pub flamethrower3: Handle<Image>,
    pub pulsewave: Handle<Image>,
    pub beam: Handle<Image>,
    pub arrow: Handle<Image>,
    pub used_bullet: Handle<Image>,

    pub shield_cell: Handle<Image>,
}

// The mouse's position in world coordinates
pub struct MousePosition(pub Vec2);
pub struct PlayerToSpectate(pub u8);

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
