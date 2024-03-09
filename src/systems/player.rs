use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::primitives::Circle,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
    utils::default,
};

use crate::{bundles::PlayerBundle, components::attributes};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circumradius = attributes::Circumradius(10.0);
    let mesh = Mesh2dHandle(meshes.add(Circle::new(circumradius.0)));
    let material = materials.add(Color::rgb(0.8, 0.8, 0.8));
    let transform = Transform::from_xyz(0.0, 0.0, -2.0);

    let material_mesh_bundle = MaterialMesh2dBundle {
        mesh,
        material,
        transform,
        ..default()
    };

    commands.spawn(PlayerBundle {
        material_mesh_bundle,
        circumradius,
        ..default()
    });
}
