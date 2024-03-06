use bevy::{
    ecs::bundle::Bundle,
    math::vec2,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::components::{AngularVelocity, Circumradius, Enemy, Player, Velocity};

#[derive(Bundle)]
pub struct EnemyBundle {
    pub marker: Enemy,
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub circumradius: Circumradius,
    pub velocity: Velocity,
    pub angular_velocity: AngularVelocity,
}
impl Default for EnemyBundle {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            marker: Enemy,
            material_mesh_bundle: MaterialMesh2dBundle::default(),
            circumradius: Circumradius(rng.gen_range(5.0..10.0)),
            velocity: Velocity(vec2(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0))),
            angular_velocity: AngularVelocity(rng.gen_range(-2.0..2.0)),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub circumradius: Circumradius,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            material_mesh_bundle: MaterialMesh2dBundle::default(),
            circumradius: Circumradius(-1.0),
        }
    }
}