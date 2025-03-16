mod hitpoints;
mod movement;

pub use hitpoints::Hitpoints;
pub use movement::Movement;

use bevy::{ecs::component::Component, math::Vec2, time::Timer};

#[derive(Component, Default)]
pub struct Target(pub Vec2);

#[derive(Component, Default)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Default, Clone, Copy)]
pub struct Radius(pub f32);

#[derive(Component, Default)]
pub struct SightRange(pub f32);

// TODO: are we going to use this for anything?
// #[derive(Component, Default)]
// pub struct Range(pub f32);

#[derive(Component, Default)]
pub struct LifeSpan(pub Timer);

#[derive(Component, Default)]
pub struct Damage(pub f32);
