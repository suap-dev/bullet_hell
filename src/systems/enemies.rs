use bevy::{prelude::*, sprite};
use rand::Rng;
use std::f32::consts::TAU;

use crate::{
    bundles,
    components::{attributes, markers},
    Borders,
};

const NR_OF_OBJECTS: usize = 100;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    borders: Res<Borders>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NR_OF_OBJECTS {
        let circumradius = attributes::Circumradius(rng.gen_range(5.0..10.0));
        let mesh = sprite::Mesh2dHandle(
            meshes.add(RegularPolygon::new(circumradius.0, rng.gen_range(3..6))),
        );
        let material = materials.add(Color::rgb(
            rng.gen_range(0.4..0.6),
            rng.gen_range(0.1..0.2),
            rng.gen_range(0.4..0.8),
        ));
        let transform = Transform::from_xyz(
            rng.gen_range(borders.left..borders.right) * 0.95,
            rng.gen_range(borders.bottom..borders.top) * 0.95,
            rng.gen_range(-1.0..1.0),
        )
        .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..TAU)));

        let material_mesh_bundle = sprite::MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        };

        commands.spawn(bundles::Enemy {
            material_mesh_bundle,
            ..default()
        });
    }
}

pub fn seek_player(
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
