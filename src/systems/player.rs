use bevy::{prelude::*, sprite};

use crate::{
    bundles,
    components::{attributes, markers},
};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circumradius = attributes::Circumradius(6.0);
    let mesh = sprite::Mesh2dHandle(meshes.add(Circle::new(circumradius.0)));
    let material = materials.add(Color::rgb(0.0, 0.8, 0.6));
    let transform = Transform::from_xyz(0.0, 0.0, -2.0);

    let material_mesh_bundle = sprite::MaterialMesh2dBundle {
        mesh,
        material,
        transform,
        ..default()
    };

    commands.spawn(bundles::Player {
        material_mesh_bundle,
        circumradius,
        ..default()
    });
}

pub fn movement_input(
    mut player: Query<&mut attributes::Movement, (With<markers::Player>, Without<markers::Enemy>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut player_movement = player.single_mut();
    player_movement.direction = Vec2::ZERO;

    let action_up =
        keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);
    let action_down =
        keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);
    let action_left =
        keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);
    let action_right =
        keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);

    if action_right {
        player_movement.direction.x += 1.0;
    }
    if action_left {
        player_movement.direction.x -= 1.0;
    }
    if action_up {
        player_movement.direction.y += 1.0;
    }
    if action_down {
        player_movement.direction.y -= 1.0;
    }
}
