use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Projectile;

#[derive(Component, Default)]
pub struct FpsCounter;

#[derive(Component)]
pub struct PlayerHealthbar;
