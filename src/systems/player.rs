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
    let circumradius = 6.0;

    let material_mesh_bundle = sprite::MaterialMesh2dBundle {
        mesh: sprite::Mesh2dHandle(meshes.add(Circle::new(circumradius))),
        material: materials.add(Color::rgb(0.0, 0.8, 0.6)),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    };

    commands.spawn(bundles::Player {
        material_mesh_bundle,
        circumradius: attributes::Circumradius(circumradius),
        movement: attributes::Movement::from_max_speed(80.0),
        ..default()
    });
}

pub fn movement_input(
    mut player: Query<&mut attributes::Movement, With<markers::Player>>,
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

pub fn find_nearest_enemy(
    mut player: Query<(&Transform, &mut attributes::NearestEnemy), With<markers::Player>>,
    enemies: Query<&Transform, With<markers::Enemy>>,
) {
    let (transform, mut nearest_enemy) = player.single_mut();
    let position = transform.translation.xy();

    let mut distance_squared = f32::MAX;
    let mut to_nearest_enemy = vec2(0.0, 0.0);

    for enemy_transform in &enemies {
        (distance_squared, to_nearest_enemy) = {
            let to_enemy = enemy_transform.translation.xy() - position;
            let new_distance_squared = to_enemy.length_squared();

            if new_distance_squared < distance_squared {
                (new_distance_squared, to_enemy)
            } else {
                (distance_squared, to_nearest_enemy)
            }
        }
    }

    *nearest_enemy = attributes::NearestEnemy(to_nearest_enemy);
}

// TODO: tidy this up, please... I was really tired when I coded this :|
pub fn shoot_nearest_enemy(
    nearest_enemy: Query<(&Transform, &attributes::NearestEnemy), With<markers::Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (transform, nearest_enemy) = nearest_enemy.single();
    let (position, to_nearest_enemy) = (transform.translation.xy(), nearest_enemy.0);

    if to_nearest_enemy.length_squared() > 0.0 {
        let circumradius = 2.0;

        let material_mesh_bundle = sprite::MaterialMesh2dBundle {
            mesh: sprite::Mesh2dHandle(meshes.add(Circle::new(circumradius))),
            material: materials.add(Color::rgb(0.6, 1.0, 0.0)),
            transform: Transform::from_translation(position.extend(-1.0)),
            ..default()
        };

        commands.spawn(bundles::Projectile {
            material_mesh_bundle,
            damage: attributes::Damage(10.0),
            circumradius: attributes::Circumradius(circumradius),
            movement: attributes::Movement::new(to_nearest_enemy, 200.0),
            marker: markers::Projectile,
            lifespan: attributes::LifeSpan(Timer::from_seconds(1.5, TimerMode::Once)),
            ..default()
        });
    }
}
