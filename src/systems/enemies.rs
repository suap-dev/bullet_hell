use bevy::{
    asset::Assets,
    ecs::{
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    math::{primitives::RegularPolygon, Quat, Vec3Swizzles},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    time::Time,
    transform::components::Transform,
    utils::default,
    window::Window,
};
use rand::Rng;
use std::{f32::consts::TAU, ops::Neg};

use crate::{
    bundles::EnemyBundle,
    components::{AngularVelocity, Circumradius, Enemy, LineOfSightRange, Player, Velocity},
};

const NR_OF_OBJECTS: usize = 200;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NR_OF_OBJECTS {
        let circumradius = Circumradius(rng.gen_range(5.0..10.0));
        let mesh =
            Mesh2dHandle(meshes.add(RegularPolygon::new(circumradius.0, rng.gen_range(3..6))));
        let material = materials.add(Color::rgb(
            rng.gen_range(0.2..0.8),
            rng.gen_range(0.0..0.2),
            rng.gen_range(0.2..0.8),
        ));
        let transform = Transform::from_xyz(
            rng.gen_range(-400.0..400.0),
            rng.gen_range(-300.0..300.0),
            rng.gen_range(-1.0..1.0),
        )
        .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..TAU)));

        let material_mesh_bundle = MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        };

        commands.spawn(EnemyBundle {
            material_mesh_bundle,
            ..default()
        });
    }
}

// apply position change and also let them go through a wall to the other side of the scene
#[allow(clippy::type_complexity)]
pub fn update(
    window: Query<&Window>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &LineOfSightRange,
            &Velocity,
            &AngularVelocity,
            &Circumradius,
        ),
        (With<Enemy>, Without<Player>),
    >,
    player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let window = window.single();
    let player_transform = player_transform.single();

    for (mut transform, los_range, velocity, angular_velocity, circumradius) in &mut query {
        let velocity = {
            let towards_player = player_transform.translation - transform.translation;
            if towards_player.length() < los_range.0 {
                (towards_player.normalize() * velocity.0.length()).xy()
            } else {
                velocity.0
            }
        };

        let angular_velocity = angular_velocity.0;

        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();

        let out_of_bounds_offset_x = window.resolution.width() / 2.0 + circumradius.0;
        if transform.translation.x > out_of_bounds_offset_x
            || transform.translation.x < out_of_bounds_offset_x.neg()
        {
            transform.translation.x = transform.translation.x.neg();
        }

        let out_of_bounds_offset_y = window.resolution.height() / 2.0 + circumradius.0;
        if transform.translation.y > out_of_bounds_offset_y
            || transform.translation.y < out_of_bounds_offset_y.neg()
        {
            transform.translation.y = transform.translation.y.neg();
        }

        transform.rotate(Quat::from_rotation_z(
            angular_velocity * time.delta_seconds(),
        ));
    }
}
