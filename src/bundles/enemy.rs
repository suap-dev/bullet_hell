use bevy::{ecs::bundle::Bundle, math::vec2, sprite};
use rand::Rng;

use crate::components::{attributes, markers};

#[derive(Bundle)]
pub struct Enemy {
    pub marker: markers::Enemy,
    pub material_mesh_bundle: sprite::MaterialMesh2dBundle<sprite::ColorMaterial>,
    pub circumradius: attributes::Circumradius,
    pub movement: attributes::Movement,
    pub los_range: attributes::LineOfSightRange,
    pub angular_velocity: attributes::AngularVelocity,
}
impl Default for Enemy {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            marker: markers::Enemy,
            material_mesh_bundle: sprite::MaterialMesh2dBundle::default(),
            circumradius: attributes::Circumradius(rng.gen_range(5.0..10.0)),
            movement: attributes::Movement {
                direction: vec2(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0)),
                speed: rng.gen_range(10.0..30.0),
            },
            los_range: attributes::LineOfSightRange(rng.gen_range(100.0..300.0)),
            angular_velocity: attributes::AngularVelocity(rng.gen_range(-2.0..2.0)),
        }
    }
}
