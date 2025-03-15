use avian2d::prelude::{Collider, CollisionLayers, CollisionMargin};
use bevy::{math::vec2, prelude::*};

use crate::{
    bundles,
    components::{attributes, markers},
    config, resources,
};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 6.0;

    let sprite = bundles::ProtoSprite {
        mesh: Mesh2d(meshes.add(Circle::new(radius))),
        material: MeshMaterial2d(materials.add(Color::srgb(0.0, 0.8, 0.6))),
    };

    let body = bundles::Body {
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        radius: attributes::Radius(radius),
        collider: Collider::circle(radius),
        collision_layers: CollisionLayers::from_bits(0b1000, 0b1000),
        collision_margin: CollisionMargin(radius * config::DEFAULT_COLLISION_MARGIN_RATIO),
    };

    commands.spawn(bundles::Player {
        sprite,
        body,
        movement: attributes::Movement::from_max_speed(80.0),
        ..default()
    });
}

pub fn handle_input(
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

pub fn target_closest_enemy(
    mut player: Query<(&Transform, &mut attributes::Target), With<markers::Player>>,
    enemies: Query<&Transform, With<markers::Enemy>>,
) {
    let (transform, mut nearest_enemy) = player.single_mut();
    let player_position = transform.translation.xy();

    let mut distance_squared = f32::MAX;
    let mut nearest_enemy_position = vec2(0.0, 0.0);

    for enemy_transform in &enemies {
        (distance_squared, nearest_enemy_position) = {
            let to_enemy = enemy_transform.translation.xy() - player_position;
            let new_distance_squared = to_enemy.length_squared();

            if new_distance_squared < distance_squared {
                (new_distance_squared, enemy_transform.translation.xy())
            } else {
                (distance_squared, nearest_enemy_position)
            }
        }
    }

    *nearest_enemy = attributes::Target(nearest_enemy_position);
}

pub fn shoot_target(
    player: Query<(&Transform, &attributes::Target), With<markers::Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut cooldown: ResMut<resources::ShootCooldown>,
) {
    if cooldown.0.tick(time.delta()).finished() {
        let (&player_transform, &attributes::Target(target_position)) = player.single();
        let player_position = player_transform.translation.xy();

        if player_position.distance_squared(target_position) > 0.0 {
            let radius = 2.0;

            commands.spawn(bundles::Projectile {
                damage: attributes::Damage(10.0),
                sprite: bundles::ProtoSprite {
                    mesh: Mesh2d(meshes.add(Circle::new(radius))),
                    material: MeshMaterial2d(materials.add(Color::srgb(0.6, 1.0, 0.0))),
                },
                body: bundles::Body {
                    transform: Transform::from_translation(
                        player_transform.translation - Vec3::new(0., 0., 1.0),
                    ),
                    radius: attributes::Radius(radius),
                    collider: Collider::circle(radius),
                    collision_layers: CollisionLayers::new(2, 1),
                    collision_margin: CollisionMargin(
                        radius * config::DEFAULT_COLLISION_MARGIN_RATIO,
                    ),
                },
                movement: attributes::Movement::new(target_position - player_position, 200.0),
                lifespan: attributes::LifeSpan(Timer::from_seconds(1.5, TimerMode::Once)),
                ..default()
            });
        }
    }
}
