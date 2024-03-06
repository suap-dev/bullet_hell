use bevy::{
    asset::Assets,
    ecs::system::{Commands, Query, Res, ResMut},
    math::{primitives::RegularPolygon, Quat},
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
    components::{AngularVelocity, Circumradius, Velocity},
};

const NR_OF_OBJECTS: usize = 100;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NR_OF_OBJECTS {
        let circumradius = Circumradius(rng.gen_range(5.0..10.0));
        let mesh =
            Mesh2dHandle(meshes.add(RegularPolygon::new(circumradius.0, rng.gen_range(3..7))));
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
pub fn update(
    window: Query<&Window>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &AngularVelocity, &Circumradius)>,
) {
    let window = window.single();

    for (mut transform, velocity, angular_velocity, circumradius) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();

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
            angular_velocity.0 * time.delta_seconds(),
        ));
    }
}
