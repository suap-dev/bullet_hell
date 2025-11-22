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
    let radius = 6.;

    let sprite = bundles::ProtoSprite {
        mesh: Mesh2d(meshes.add(Circle::new(radius))),
        material: MeshMaterial2d(materials.add(Color::srgb(0., 0.8, 0.6))),
    };

    let body = bundles::Body {
        transform: Transform::from_xyz(0., 0., -2.),
        radius: attributes::Radius(radius),
        collider: Collider::circle(radius),
        collision_layers: CollisionLayers::from_bits(0b0100, 0b0001),
        collision_margin: CollisionMargin(radius * config::DEFAULT_COLLISION_MARGIN_RATIO),
    };

    commands.spawn(bundles::Player {
        sprite,
        body,
        movement: attributes::Movement::from_max_speed(80.),
        nearest_enemy: attributes::Target::default(),
        marker: markers::Player,
        hitpoints: attributes::Hitpoints::from_max(100.),
    });
}

pub fn handle_input(
    mut player: Query<&mut attributes::Movement, With<markers::Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    // TODO: separate module for input mappings?
    // probably a resource for input collection?
    let action_up =
        keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);
    let action_down =
        keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);
    let action_left =
        keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);
    let action_right =
        keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);

    if action_right {
        direction.x += 1.;
    }
    if action_left {
        direction.x -= 1.;
    }
    if action_up {
        direction.y += 1.;
    }
    if action_down {
        direction.y -= 1.;
    }

    if let Ok(mut movement) = player.single_mut() {
        movement.set_direction(direction);
    }
}

pub fn target_closest_enemy(
    mut player: Query<(&Transform, &mut attributes::Target), With<markers::Player>>,
    enemies: Query<&Transform, With<markers::Enemy>>,
) {
    if let Ok((transform, mut nearest_enemy)) = player.single_mut() {
        let player_position = transform.translation.xy();

        let mut distance_squared = f32::MAX;
        let mut nearest_enemy_position = vec2(0., 0.);

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
}

// TODO: move projectile logic out of players module
pub fn shoot_target(
    player: Query<(&Transform, &attributes::Target), With<markers::Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut shoot_cooldown: ResMut<resources::ShootCooldown>,
) {
    shoot_cooldown.0.tick(time.delta());

    if shoot_cooldown.0.is_finished() {
        if let Ok((&player_transform, &attributes::Target(target_position))) = player.single() {
            let player_position = player_transform.translation.xy();

            if player_position.distance_squared(target_position) > 0. {
                let radius = 2.;

                commands.spawn(bundles::Projectile {
                    damage: attributes::Damage(10.),
                    sprite: bundles::ProtoSprite {
                        mesh: Mesh2d(meshes.add(Circle::new(radius))),
                        material: MeshMaterial2d(materials.add(Color::srgb(0.6, 1., 0.))),
                    },
                    body: bundles::Body {
                        transform: Transform::from_translation(
                            player_transform.translation - Vec3::new(0., 0., 1.),
                        ),
                        radius: attributes::Radius(radius),
                        collider: Collider::circle(radius),
                        collision_layers: CollisionLayers::new(0b0010, 0b0001),
                        collision_margin: CollisionMargin(
                            radius * config::DEFAULT_COLLISION_MARGIN_RATIO,
                        ),
                    },
                    movement: attributes::Movement::new(target_position - player_position, 200.),
                    lifespan: attributes::LifeSpan(Timer::from_seconds(1.5, TimerMode::Once)),
                    marker: markers::Projectile,
                });
            }
        }
    }
}

pub fn log_hp(
    player: Query<&attributes::Hitpoints, (With<markers::Player>, Changed<attributes::Hitpoints>)>,
) {
    if let Ok(hitpoints) = player.single() {
        println!("{}", hitpoints.current());
    }
}
