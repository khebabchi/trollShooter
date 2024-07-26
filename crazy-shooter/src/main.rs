use bevy::{
    prelude::*,
    window::WindowTheme,
};
use crazy_shooter::{setup, AimHelperPlugin, EnemyPlugin, PlayerPlugin, ResourcesPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game client".into(),
                resolution: (795., 795.).into(),
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: false,
                ..default()
            }),
            ..default()
        }),))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PlayerPlugin) 
        .add_plugins(AimHelperPlugin)
        .add_plugins(EnemyPlugin) 
        .run();
}

