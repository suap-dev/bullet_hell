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
    components::{attributes, markers},
};

const NR_OF_OBJECTS: usize = 200;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..NR_OF_OBJECTS {
        let circumradius = attributes::Circumradius(rng.gen_range(5.0..10.0));
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
            &attributes::LineOfSightRange,
            &attributes::Velocity,
            &mut attributes::AngularVelocity,
            &attributes::Circumradius,
        ),
        (With<markers::Enemy>, Without<markers::Player>),
    >,
    player_transform: Query<&Transform, (With<markers::Player>, Without<markers::Enemy>)>,
) {
    let window = window.single();
    let player_transform = player_transform.single();

    // TODO: right_bound and top_bound - could they be a resource?
    let right_border = window.resolution.width() / 2.0;
    let left_border = right_border.neg();
    let top_border = window.resolution.height() / 2.0;
    let bottom_border = top_border.neg();

    // let mut closest: (Entity, f32) = (Entity::from_raw(0), f32::MAX);

    // TODO: rewrite it so it uses transform arithmetics and uses angular velocity in a cooler way
    for (/*entity,*/ mut transform, los_range, velocity, angular_velocity, circumradius) in
        &mut query
    {
        // if dist_squared < closest.1 {
        //     closest = (entity, dist_squared);
        // }

        let dist_squared = transform
            .translation
            .distance_squared(player_transform.translation);
        let velocity = {
            if dist_squared < los_range.0.powi(2) {
                (player_transform.translation - transform.translation).normalize()
                    * velocity.0.length()
            } else {
                velocity.0
            }
        };

        apply_velocity(&mut transform.translation, velocity, time.delta_seconds());
        teleport_if_out_of_bounds(
            &mut transform.translation,
            circumradius.0,
            left_border,
            right_border,
            bottom_border,
            top_border,
        );
        transform.rotate(Quat::from_rotation_z(
            angular_velocity.0 * time.delta_seconds(),
        ));
    }
}

fn apply_velocity(translation: &mut Vec3, velocity: Vec3, delta_seconds: f32) {
    *translation += velocity * delta_seconds;
}

fn teleport_if_out_of_bounds(
    translation: &mut Vec3,
    circumradius: f32,
    left_border: f32,
    right_border: f32,
    bottom_border: f32,
    top_border: f32,
) {
    fix_coordinate_cycle(
        &mut translation.x,
        left_border - circumradius,
        right_border + circumradius,
    );
    fix_coordinate_cycle(
        &mut translation.y,
        bottom_border - circumradius,
        top_border + circumradius,
    );
}

fn fix_coordinate_cycle(coord: &mut f32, lower_bound: f32, upper_bound: f32) {
    if *coord < lower_bound || upper_bound < *coord {
        *coord = coord.neg();
    }
}
