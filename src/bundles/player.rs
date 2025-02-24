use bevy::{ecs::bundle::Bundle, sprite::Material2d, transform::components::Transform};

use crate::{
    bundles::ProtoSprite,
    components::{attributes, markers},
};

#[derive(Bundle, Default)]
pub struct Player<M: Material2d> {
    pub marker: markers::Player,
    pub sprite: ProtoSprite<M>,
    pub transform: Transform,
    pub collision_radius: attributes::CollisionRadius,
    pub movement: attributes::Movement,
    pub nearest_enemy: attributes::NearestEnemy,
}
