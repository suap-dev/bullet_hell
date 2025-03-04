use bevy::{ecs::system::Resource, time::Timer};

#[derive(Resource)]
pub struct Borders {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

#[derive(Resource)]
pub struct ShootCooldown(pub Timer);
