mod enemy;
mod player;
mod projectile;

pub use enemy::*;
pub use player::*;
pub use projectile::*;

use bevy::{
    ecs::bundle::Bundle,
    render::mesh::Mesh2d,
    sprite::{Material2d, MeshMaterial2d},
};
#[derive(Bundle, Default)]
pub struct ProtoSprite<M: Material2d> {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<M>,
}
