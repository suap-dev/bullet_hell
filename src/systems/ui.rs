use bevy::{
    color::palettes::tailwind,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::components::markers::{self, FpsCounter};

pub fn spawn(mut commands: Commands) {
    let border = commands
        .spawn((
            Node {
                width: Val::Percent(40.),
                height: Val::Percent(3.),
                margin: UiRect::all(Val::Px(30.)),
                align_self: AlignSelf::FlexEnd,
                justify_self: JustifySelf::Center,
                padding: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BackgroundColor::from(tailwind::EMERALD_800),
            BorderRadius::all(Val::Px(5.)),
            BoxShadow {
                color: Color::from(tailwind::STONE_950),
                x_offset: Val::Px(4.),
                y_offset: Val::Px(7.),
                spread_radius: Val::Px(3.),
                blur_radius: Val::Px(3.),
            },
        ))
        .id();

    let bar = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor::from(tailwind::EMERALD_300),
            BorderRadius::all(Val::Px(4.)),
        ))
        .id();

    commands.entity(border).add_child(bar);
    commands.spawn((
        markers::FpsCounter,
        Text::new("Hello."),
        Node {
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
    ));
}

pub fn fps_counter(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FpsCounter>>,
) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_val) = fps_diagnostic.smoothed() {
            **fps_text_query.single_mut() = format!("{fps_val:.0}");
        }
    }
}
