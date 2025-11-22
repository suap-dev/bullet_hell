pub const WINDOW_LOGICAL_WIDTH: f32 = 800.;
pub const WINDOW_LOGICAL_HEIGHT: f32 = 600.;

pub const WINDOW_PHYSICAL_WIDTH: u32 = 800;
pub const WINDOW_PHYSICAL_HEIGHT: u32 = 600;

pub const BORDER_TELEPORT_CORRECTION: f32 = 4.0;

pub const ENEMY_COUNT: usize = 100;

pub const DEFAULT_COLLISION_MARGIN_RATIO: f32 = 0.1;
pub const SHOOT_COOLDOWN: f32 = 0.5;

// "The default timestep() is 64 hertz, or 15625 microseconds
// This value was chosen because using 60 hertz has the
// potential for a pathological interaction with the monitor
// refresh rate where the game alternates between running two
// fixed timesteps and zero fixed timesteps per frame (for
// example when running two fixed timesteps takes longer than
// a frame). Additionally, the value is a power of two which
// losslessly converts into f32 and f64.""
// source: https://docs.rs/bevy/latest/bevy/time/struct.Fixed.html
