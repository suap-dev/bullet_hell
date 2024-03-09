use bevy::{prelude::*, sprite};

use crate::{bundles, components::attributes};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circumradius = attributes::Circumradius(10.0);
    let mesh = sprite::Mesh2dHandle(meshes.add(Circle::new(circumradius.0)));
    let material = materials.add(Color::rgb(0.8, 0.8, 0.8));
    let transform = Transform::from_xyz(0.0, 0.0, -2.0);

    let material_mesh_bundle = sprite::MaterialMesh2dBundle {
        mesh,
        material,
        transform,
        ..default()
    };

    commands.spawn(bundles::Player {
        material_mesh_bundle,
        circumradius,
        ..default()
    });
}
