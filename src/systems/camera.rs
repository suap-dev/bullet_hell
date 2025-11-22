use bevy::{camera::Camera2d, ecs::system::Commands};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
