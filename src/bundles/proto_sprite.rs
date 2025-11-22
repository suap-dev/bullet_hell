use bevy::{
    ecs::bundle::Bundle,
    prelude::Mesh2d,
    sprite_render::{Material2d, MeshMaterial2d},
};

#[derive(Bundle, Default)]
pub struct ProtoSprite<M: Material2d> {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<M>,
}
