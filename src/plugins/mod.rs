use avian2d::PhysicsPlugins;
use bevy::{
    DefaultPlugins,
    app::PluginGroupBuilder,
    prelude::*,
    window::{EnabledButtons, WindowResolution},
};

use crate::config;

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        config::WINDOW_LOGICAL_WIDTH,
                        config::WINDOW_LOGICAL_HEIGHT,
                    ),
                    resizable: false,
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            })
            .add_group(PhysicsPlugins::default())
    }
}
