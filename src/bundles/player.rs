use bevy::{ecs::bundle::Bundle, math::Vec2, sprite};

use crate::components::{attributes, markers};

#[derive(Bundle)]
pub struct Player {
    pub marker: markers::Player,
    pub material_mesh_bundle: sprite::MaterialMesh2dBundle<sprite::ColorMaterial>,
    pub circumradius: attributes::Circumradius,
    pub movement: attributes::Movement,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            marker: markers::Player,
            material_mesh_bundle: sprite::MaterialMesh2dBundle::default(),
            circumradius: attributes::Circumradius(-1.0),
            movement: attributes::Movement {
                direction: Vec2::ZERO,
                speed: 80.0,
            },
        }
    }
}
