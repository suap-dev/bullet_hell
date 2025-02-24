use bevy::{math::vec2, prelude::*};
use rand::Rng;
use std::f32::consts::TAU;

use crate::{
    bundles,
    components::{attributes, markers},
    config::ENEMIES_NUMBER,
    Borders,
};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    borders: Res<Borders>,
) {
    let mut rng = rand::rng();

    for _ in 0..ENEMIES_NUMBER {
        let circumradius = attributes::Circumradius(rng.random_range(5.0..10.0));
        let material_mesh_bundle = {
            let mesh =
                Mesh2d(meshes.add(RegularPolygon::new(circumradius.0, rng.random_range(3..6))));
            let material = MeshMaterial2d(materials.add(Color::srgb(
                rng.random_range(0.4..0.6),
                rng.random_range(0.1..0.2),
                rng.random_range(0.4..0.8),
            )));
            let transform = Transform::from_xyz(
                rng.random_range(borders.left..borders.right) * 0.95,
                rng.random_range(borders.bottom..borders.top) * 0.95,
                rng.random_range(-1.0..1.0),
            )
            .with_rotation(Quat::from_rotation_z(rng.random_range(0.0..TAU)));

            (mesh, material, transform)
        };

        // TODO: handle material_mesh_bundle situation properly
        commands
            .spawn(bundles::Enemy {
                circumradius,
                movement: attributes::Movement::from_velocity(vec2(
                    rng.random_range(-20.0..20.0),
                    rng.random_range(-20.0..20.0),
                )),
                hitpoints: attributes::HitPoints(rng.random_range(9.0..=40.0)),
                los_range: attributes::LineOfSightRange(rng.random_range(100.0..300.0)),
                angular_velocity: attributes::AngularVelocity(rng.random_range(-2.0..2.0)),
                ..default()
            })
            .insert(material_mesh_bundle);
    }
}

pub fn player_seeking(
    mut query: Query<
        (
            &mut attributes::Movement,
            &Transform,
            &attributes::LineOfSightRange,
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
