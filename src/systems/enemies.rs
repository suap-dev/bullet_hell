use bevy::{
    asset::Assets,
    ecs::{
        // entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    math::{primitives::RegularPolygon, Quat, Vec3},
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

#[allow(clippy::type_complexity)]
pub fn update(
    window: Query<&Window>,
    time: Res<Time>,
    mut query: Query<
        (
            // Entity,
            &mut Transform,
            &LineOfSightRange,
            &Velocity,
            &mut AngularVelocity,
            &Circumradius,
        ),
        (With<Enemy>, Without<Player>),
    >,
    player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let window = window.single();
    let player_transform = player_transform.single();

    // let mut closest: (Entity, f32) = (Entity::from_raw(0), f32::MAX);

    // TODO: rewrite it so it uses transform arithmetics and uses angular velocity in a cooler way
    for (/*entity,*/ mut transform, los_range, velocity, angular_velocity, circumradius) in
        &mut query
    {
        let dist_squared = transform
            .translation
            .distance_squared(player_transform.translation);

        // if dist_squared < closest.1 {
        //     closest = (entity, dist_squared);
        // }

        let velocity = {
            if dist_squared < los_range.0.powi(2) {
                (player_transform.translation - transform.translation).normalize()
                    * velocity.0.length()
            } else {
                velocity.0
            }
        };

        // TODO: right_bound and top_bound - could they be a resource?
        let right_bound = window.resolution.width() / 2.0 + circumradius.0;
        let top_bound = window.resolution.height() / 2.0 + circumradius.0;
        update_translation(
            &mut transform.translation,
            velocity,
            time.delta_seconds(),
            right_bound.neg(),
            right_bound,
            top_bound.neg(),
            top_bound,
        );

        transform.rotate(Quat::from_rotation_z(
            angular_velocity.0 * time.delta_seconds(),
        ));
    }
}

fn update_translation(
    coords: &mut Vec3,
    velocity: Vec3,
    delta_seconds: f32,
    left_bound: f32,
    right_bound: f32,
    bottom_bound: f32,
    top_bound: f32,
) {
    update_single_coord(
        &mut coords.x,
        velocity.x,
        delta_seconds,
        left_bound,
        right_bound,
    );

    update_single_coord(
        &mut coords.y,
        velocity.y,
        delta_seconds,
        bottom_bound,
        top_bound,
    );
}

fn update_single_coord(
    positional_coord: &mut f32,
    velocity_coord: f32,
    delta_seconds: f32,
    left_bound: f32,
    right_bound: f32,
) {
    *positional_coord = velocity_coord.mul_add(delta_seconds, *positional_coord);

    if *positional_coord < left_bound || right_bound < *positional_coord {
        *positional_coord = positional_coord.neg();
    }
}
