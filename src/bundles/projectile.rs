use bevy::{ecs::bundle::Bundle, sprite::Material2d};

use crate::{
    bundles,
    components::{attributes, markers},
};

#[derive(Bundle, Default)]
pub struct Projectile<M: Material2d> {
    pub sprite: bundles::ProtoSprite<M>,
    pub body: bundles::Body,
    pub damage: attributes::Damage,
    pub movement: attributes::Movement,
    pub lifespan: attributes::LifeSpan,
    //
    pub marker: markers::Projectile,
}
