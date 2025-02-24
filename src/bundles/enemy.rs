use bevy::ecs::bundle::Bundle;

use crate::components::{attributes, markers};

#[derive(Bundle, Default)]
pub struct Enemy {
    pub marker: markers::Enemy,
    // pub material_mesh_bundle: sprite::MaterialMesh2dBundle<sprite::ColorMaterial>,
    pub circumradius: attributes::Circumradius,
    pub movement: attributes::Movement,
    pub hitpoints: attributes::HitPoints,
    pub los_range: attributes::LineOfSightRange,
    pub angular_velocity: attributes::AngularVelocity,
}
