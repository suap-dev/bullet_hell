use bevy::{
    ecs::resource::Resource,
    time::{Timer, TimerMode},
};

use crate::config;

#[derive(Resource)]
pub struct Borders {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}
impl Default for Borders {
    fn default() -> Self {
        Self {
            top: config::WINDOW_LOGICAL_HEIGHT / 2.0,
            bottom: -config::WINDOW_LOGICAL_HEIGHT / 2.0,
            right: config::WINDOW_LOGICAL_WIDTH / 2.0,
            left: -config::WINDOW_LOGICAL_WIDTH / 2.0,
        }
    }
}

#[derive(Resource)]
pub struct ShootCooldown(pub Timer);
impl Default for ShootCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(
            config::SHOOT_COOLDOWN,
            TimerMode::Repeating,
        ))
    }
}
