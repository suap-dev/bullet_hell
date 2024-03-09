use bevy::{ecs::bundle::Bundle, sprite};

use crate::components::{attributes, markers};

#[derive(Bundle)]
pub struct Player {
    pub marker: markers::Player,
    pub material_mesh_bundle: sprite::MaterialMesh2dBundle<sprite::ColorMaterial>,
    pub circumradius: attributes::Circumradius,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            marker: markers::Player,
            material_mesh_bundle: sprite::MaterialMesh2dBundle::default(),
            circumradius: attributes::Circumradius(-1.0),
        }
    }
}
