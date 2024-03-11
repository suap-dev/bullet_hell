use bevy::{prelude::*, sprite};
use rand::Rng;
use std::{f32::consts::TAU, ops::Neg};

use crate::{
    bundles,
    components::{attributes, markers},
};

const NR_OF_OBJECTS: usize = 100;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            rng.gen_range(-400.0..400.0),
            rng.gen_range(-300.0..300.0),
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

#[allow(clippy::type_complexity)]
pub fn update(
    window: Query<&Window>,
    time: Res<Time>,
    mut query: Query<
        (
            // Entity,
            &mut Transform,
            &mut attributes::Movement,
            &attributes::LineOfSightRange,
            &attributes::AngularVelocity,
            &attributes::Circumradius,
        ),
        (With<markers::Enemy>, Without<markers::Player>),
    >,
    player_transform: Query<&Transform, (With<markers::Player>, Without<markers::Enemy>)>,
) {
    let player_position = player_transform.single().translation;

    // TODO: borders - could they be a resource?
    let (right_border, left_border, top_border, bottom_border) = {
        let resolution = &window.single().resolution;

        let right_border = resolution.width() / 2.0;
        let top_border = resolution.height() / 2.0;

        (
            right_border,
            right_border.neg(),
            top_border,
            top_border.neg(),
        )
    };

    // let mut closest: (Entity, f32) = (Entity::from_raw(0), f32::MAX);

    // TODO: rewrite it so it uses transform arithmetics and uses angular velocity in a cooler way
    for (/*entity,*/ mut transform, mut movement, los_range, angular_velocity, circumradius) in
        &mut query
    {
        // if dist_squared < closest.1 {
        //     closest = (entity, dist_squared);
        // }

        // TODO: make it a separate system "seek_player"
        let position = transform.translation;
        let dist_squared = position.distance_squared(player_position);
        if dist_squared < los_range.0.powi(2) {
            movement.set_direction((player_position - transform.translation).xy());
        }

        // TODO: make it a separate system, more generic, that could also update player and possibly let you chose the offset of out of bounds calculation
        teleport_if_out_of_bounds(
            &mut transform.translation,
            circumradius.0,
            left_border,
            right_border,
            bottom_border,
            top_border,
        );

        // TODO: not sure if we want this to randomly rotate all the time. maybe some more situation based rotation?
        transform.rotate(Quat::from_rotation_z(
            angular_velocity.0 * time.delta_seconds(),
        ));
    }
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
