use bevy::{ecs::bundle::Bundle, pbr::PbrBundle};

use crate::components::{attributes, markers};

#[derive(Bundle)]
pub struct Player {
    pub marker: markers::Player,
    pub material_mesh_bundle: PbrBundle,

    pub circumradius: attributes::Circumradius,
    pub movement: attributes::Movement,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            marker: markers::Player,
            // material_mesh_bundle: sprite::MaterialMesh2dBundle::default(),
            material_mesh_bundle: PbrBundle::default(),
            circumradius: attributes::Circumradius(-1.0),
            movement: attributes::Movement::from_max_speed(80.0),
        }
    }
}
