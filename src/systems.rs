use std::f32::consts::TAU;

use bevy::{
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct AngularVelocity(f32);

pub fn spawn_mfkers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..200 {
        let mesh = Mesh2dHandle(meshes.add(RegularPolygon::new(
            rng.gen_range(5.0..10.0),
            rng.gen_range(3..7),
        )));
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

        let mfker = MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        };

        commands
            .spawn(mfker)
            .insert(Velocity(vec2(
                rng.gen_range(-20.0..20.0),
                rng.gen_range(-20.0..20.0),
            )))
            .insert(AngularVelocity(rng.gen_range(-2.0..2.0)));
    }
}

pub fn update_mfkers(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &AngularVelocity)>,
) {
    for (mut transform, velocity, angular_velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(
            angular_velocity.0 * time.delta_seconds(),
        ));
    }
}
