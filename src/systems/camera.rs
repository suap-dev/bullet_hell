use bevy::{core_pipeline::core_2d::Camera2dBundle, ecs::system::Commands};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
