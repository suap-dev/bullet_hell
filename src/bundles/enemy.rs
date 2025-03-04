use bevy::{ecs::bundle::Bundle, sprite::Material2d};

use crate::{
    bundles,
    components::{attributes, markers},
};

#[derive(Bundle, Default)]
pub struct Enemy<M: Material2d> {
    pub sprite: bundles::ProtoSprite<M>,
    pub body: bundles::Body,
    pub movement: attributes::Movement,
    pub hitpoints: attributes::Hitpoints,
    pub los_range: attributes::SightRange,
    pub angular_velocity: attributes::AngularVelocity,
    //
    pub marker: markers::Enemy,
}
