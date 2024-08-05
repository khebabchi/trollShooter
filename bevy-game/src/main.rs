use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowTheme};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_game::{setup, AimHelperPlugin, EnemyPlugin, PlayerPlugin, ResourcesPlugin};
use wasm_bindgen::prelude::wasm_bindgen;
fn main() {

    App::new().add_plugins(EmbeddedAssetPlugin{mode:PluginMode::ReplaceDefault})
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                       canvas: Some("#game-canvas".to_string()),
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
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AimHelperPlugin)
        .add_plugins(EnemyPlugin)
        .run();
}
#[wasm_bindgen(start)]
pub fn start() {
    main();
}
