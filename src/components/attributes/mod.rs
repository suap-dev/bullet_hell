mod hitpoints;
mod movement;

pub use hitpoints::Hitpoints;
pub use movement::Movement;

use bevy::{ecs::component::Component, math::Vec2, time::Timer};

// TODO: 1) remove "Default" trait
// TODO: 2) change to Option<Vec2>
// TODO: 3) Vec2 shouldn't be a target
#[derive(Component, Default)]
pub struct Target(pub Vec2);

#[derive(Component, Default)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Clone, Copy)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct SightRange(pub f32);

// TODO: are we going to use this for anything?
// #[derive(Component, Default)]
// pub struct Range(pub f32);

#[derive(Component)]
pub struct LifeSpan(pub Timer);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Dps(pub f32);
