use bevy::{
    ecs::bundle::Bundle,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::components::{attributes, markers};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: markers::Player,
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub circumradius: attributes::Circumradius,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: markers::Player,
            material_mesh_bundle: MaterialMesh2dBundle::default(),
            circumradius: attributes::Circumradius(-1.0),
        }
    }
}
