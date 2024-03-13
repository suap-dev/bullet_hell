use bevy::{ecs::bundle::Bundle, sprite};

use crate::components::{attributes, markers};

#[derive(Bundle, Default)]
pub struct Player {
    pub marker: markers::Player,
    pub material_mesh_bundle: sprite::MaterialMesh2dBundle<sprite::ColorMaterial>,
    pub circumradius: attributes::Circumradius,
    pub movement: attributes::Movement,
    pub nearest_enemy: attributes::NearestEnemy,
}
