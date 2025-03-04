use bevy::{ecs::bundle::Bundle, sprite::Material2d};

use crate::{
    bundles,
    components::{attributes, markers},
};

#[derive(Bundle, Default)]
pub struct Player<M: Material2d> {
    pub sprite: bundles::ProtoSprite<M>,
    pub body: bundles::Body,
    pub movement: attributes::Movement,
    pub nearest_enemy: attributes::Target,
    //
    pub marker: markers::Player,
}
