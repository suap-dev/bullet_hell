use bevy::{
    ecs::bundle::Bundle,
    math::vec3,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::components::{attributes, markers};

#[derive(Bundle)]
pub struct EnemyBundle {
    pub marker: markers::Enemy,
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub circumradius: attributes::Circumradius,
    pub los_range: attributes::LineOfSightRange,
    pub velocity: attributes::Velocity,
    pub angular_velocity: attributes::AngularVelocity,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            marker: markers::Enemy,
            material_mesh_bundle: MaterialMesh2dBundle::default(),
            circumradius: attributes::Circumradius(rng.gen_range(5.0..10.0)),
            los_range: attributes::LineOfSightRange(rng.gen_range(100.0..300.0)),
            velocity: attributes::Velocity(vec3(
                rng.gen_range(-20.0..20.0),
                rng.gen_range(-20.0..20.0),
                0.0,
            )),
            angular_velocity: attributes::AngularVelocity(rng.gen_range(-2.0..2.0)),
        }
    }
}
