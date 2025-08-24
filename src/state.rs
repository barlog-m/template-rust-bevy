use bevy::prelude::*;

#[derive(Resource)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
    pub left_up: Vec2,
    pub left_bottom: Vec2,
    pub right_up: Vec2,
    pub right_bottom: Vec2,
}

impl WindowSize {
    pub fn new(width: f32, height: f32) -> Self {
        WindowSize {
            width: width,
            height: height,
            left_up: vec2(width / -2.0, height / 2.0),
            left_bottom: vec2(width / -2.0, height / -2.0),
            right_up: vec2(width / 2.0, height / 2.0),
            right_bottom: vec2(width / 2.0, height / -2.0),
        }
    }
}
