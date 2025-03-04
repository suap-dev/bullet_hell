use avian2d::prelude::{Collider, CollisionLayers, CollisionMargin};
use bevy::{math::vec2, prelude::*};
use rand::Rng;
use std::f32::consts::TAU;

use crate::{
    bundles,
    components::{attributes, markers},
    config, resources,
};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    borders: Res<resources::Borders>,
) {
    let mut rng = rand::rng();

    for _ in 0..config::ENEMY_COUNT {
        let radius = rng.random_range(5.0..10.0);
        // let radius = attributes::Radius(rng.random_range(5.0..10.0));
        let sprite = bundles::ProtoSprite {
            mesh: Mesh2d(meshes.add(RegularPolygon::new(radius, rng.random_range(3..6)))),
            material: MeshMaterial2d(materials.add(Color::srgb(
                rng.random_range(0.4..0.6),
                rng.random_range(0.1..0.2),
                rng.random_range(0.4..0.8),
            ))),
        };
        let transform = Transform::from_xyz(
            rng.random_range(borders.left..borders.right) * 0.95,
            rng.random_range(borders.bottom..borders.top) * 0.95,
            rng.random_range(-1.0..1.0),
        )
        .with_rotation(Quat::from_rotation_z(rng.random_range(0.0..TAU)));

        let body = bundles::Body {
            transform,
            radius: attributes::Radius(radius),
            collider: Collider::circle(radius),
            collision_layers: CollisionLayers::new(0b0001, 0b0010),
            collision_margin: CollisionMargin(radius * config::DEFAULT_COLLISION_MARGIN_RATIO),
        };

        commands.spawn(bundles::Enemy {
            body,
            sprite,
            movement: attributes::Movement::from_velocity(vec2(
                rng.random_range(-20.0..20.0),
                rng.random_range(-20.0..20.0),
            )),
            hitpoints: attributes::Hitpoints(rng.random_range(9.0..=40.0)),
            los_range: attributes::SightRange(rng.random_range(100.0..300.0)),
            angular_velocity: attributes::AngularVelocity(rng.random_range(-2.0..2.0)),
            ..default()
        });
    }
}

pub fn seek_and_follow_player(
    mut query: Query<
        (
            &mut attributes::Movement,
            &Transform,
            &attributes::SightRange,
        ),
        With<markers::Enemy>,
    >,
    player_transform: Query<&Transform, With<markers::Player>>,
) {
    let player_position = player_transform.single().translation.xy();

    for (mut movement, transform, los_range) in &mut query {
        let position = transform.translation.xy();
        let to_player = player_position - position;

        if to_player.length_squared() < los_range.0.powi(2) {
            movement.set_direction(to_player);
        }
    }
}
