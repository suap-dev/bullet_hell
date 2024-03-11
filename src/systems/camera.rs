use std::f32::consts::TAU;

use bevy::{
    core_pipeline::{core_2d::Camera2dBundle, core_3d::Camera3dBundle},
    ecs::system::Commands,
    math::{Quat, Vec3},
    transform::components::Transform,
    utils::default,
};

pub fn setup(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn(Camera3dBundle::default());
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 800.0)
            .looking_at(Vec3::ZERO, Vec3::Z)
            .with_rotation(Quat::from_rotation_z((0.0) * TAU)),
        // exposure: Exposure::from_physical_camera(**parameters),
        ..default()
    });
}
