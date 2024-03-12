use bevy::{math::vec2, prelude::*, sprite};

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
    let mut direction = Vec2::ZERO;

    // TODO: separate module for input mappings?
    let action_up =
        keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);
    let action_down =
        keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);
    let action_left =
        keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);
    let action_right =
        keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);

    if action_right {
        direction.x += 1.0;
    }
    if action_left {
        direction.x -= 1.0;
    }
    if action_up {
        direction.y += 1.0;
    }
    if action_down {
        direction.y -= 1.0;
    }

    player.single_mut().set_direction(direction);
}

pub fn find_closest_enemy(
    player: Query<(&Transform, &mut attributes::NearestEnemy), With<markers::Player>>,
    enemies: Query<&Transform, With<markers::Enemy>>,
) {
    let position = player.single().0.translation.xy();

    let mut distance_squared = f32::MAX;
    let mut nearest_enemy = vec2(0.0, 0.0);

    for enemy_transform in &enemies {
        (distance_squared, nearest_enemy) = {
            let to_enemy = enemy_transform.translation.xy() - position;
            let new_distance_squared = to_enemy.length_squared();

            if new_distance_squared < distance_squared {
                (new_distance_squared, to_enemy)
            } else {
                (distance_squared, nearest_enemy)
            }
        }
    }

    player.single().1 = &attributes::NearestEnemy(nearest_enemy);
}
