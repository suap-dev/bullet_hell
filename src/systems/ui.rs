use bevy::{
    color::palettes::tailwind,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::components::{attributes, markers};

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
            markers::PlayerHealthbar,
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
    mut fps_text_query: Query<&mut Text, With<markers::FpsCounter>>,
) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_val) = fps_diagnostic.smoothed() {
            **fps_text_query.single_mut() = format!("{fps_val:.0}");
        }
    }
}

// TODO: consider using events here; will it even be better?
// if enemy deals damage to the player we can send an event;
// but we still need a system that will handle the event.
// at this moment we calculate the change of hp first
// and post-update we change the value of UI node.
pub fn player_healthbar(
    player_hp: Query<
        &attributes::Hitpoints,
        (With<markers::Player>, Changed<attributes::Hitpoints>),
    >,
    mut hp_bar: Query<&mut Node, With<markers::PlayerHealthbar>>,
) {
    if let Ok(hitpoints) = player_hp.get_single() {
        if let Ok(mut bar_node) = hp_bar.get_single_mut() {
            bar_node.width = Val::Percent(100. * hitpoints.current() / hitpoints.max());
        }
    }
}
