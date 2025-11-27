use bevy::prelude::Vec2;
use bevy_enhanced_input::prelude::InputAction;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Movement;
