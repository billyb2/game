#![deny(clippy::all)]

use bevy::prelude::*;

pub fn spawn_button<const BUTTON_WIDTH: Option<f32>, const BUTTON_HEIGHT: f32>(node_parent: &mut ChildBuilder, material: Handle<ColorMaterial>, text: String, font: Handle<Font>, margin: Rect<Val>) { 

    let button_width = match BUTTON_WIDTH {
        Some(w) => w,
        None => {
            let mut button_width = text.len() * 23 + 100;

            if button_width < 250 {
                button_width = 250;

            }

            button_width as f32
        
        },

    };

    node_parent.spawn_bundle(ButtonBundle {
    style: Style {
        size: Size::new(Val::Px(button_width), Val::Px(BUTTON_HEIGHT)),
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        margin,

        ..Default::default()
    },
    material,
    ..Default::default()
    })
    .with_children(|button_parent| {
        button_parent
            .spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: text,
                            style: TextStyle {
                                font,
                                font_size: 55.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()

        });
    });   
    
}