use std::f32::consts::PI;

use aim_helper_plugin::AimHelper;
use bevy::{audio::Volume, prelude::*, sprite::Anchor};
///////// exports
mod aim_helper_plugin;
mod enemy_plugin;
mod notification_plugin;
mod player_plugin;
mod recources;
pub use aim_helper_plugin::AimHelperPlugin;
pub use enemy_plugin::{Enemy, EnemyName, EnemyPlugin};
pub use notification_plugin::*;
pub use player_plugin::*;
pub use recources::*;
pub fn setup(mut commands: Commands, images: Res<Images>, sounds: Res<Sounds>) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: images.background.clone(),
        ..default()
    });
    // player controlled
    commands.spawn((
        SpriteBundle {
            texture: images.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 2.),
                ..Default::default()
            },
            ..default()
        },
        AudioBundle {
            source: sounds.background.clone(),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.1)),
            ..default()
        },
        Player {
            // meters per second
            rotation_speed: f32::to_radians(360.0), // degrees per second
            state: PlayerState::Free,
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            shooting_couldown: Timer::from_seconds(0.3, TimerMode::Once),
            bullet_count: 8,
            riffle_mode: true,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: images.aim_helper.0.clone(),
            transform: Transform {
                translation: Vec3::new(3., -8.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                ..Default::default()
            },
            ..default()
        },
        AimHelper,
    ));
    commands.spawn(SpriteBundle {
        texture: images.heart.clone(),
        transform: Transform {
            translation: Vec3::new(-370., 375.0, 10.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.8),
            ..Default::default()
        },
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: images.bullet.clone(),
        transform: Transform {
            translation: Vec3::new(-305., 375.0, 10.0),
            rotation: Quat::from_rotation_z(PI / 3.),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.8),
            ..Default::default()
        },
        ..default()
    });
    commands.spawn((
        Text2dBundle {
            text_anchor: Anchor::TopLeft,
            text: Text::from_sections([
                TextSection::new(
                    "5",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "     ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "8",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "/8",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE.with_alpha(0.3),
                        ..default()
                    },
                ),
            ]),

            transform: Transform {
                translation: Vec3::new(-350., 385., 10.),
                ..default()
            },
            ..default()
        },
        HealthCount { health: 5 },
        BulletCount,
    ));

    commands.spawn((
        Text2dBundle {
            text_anchor: Anchor::TopLeft,
            text: Text::from_sections([
                TextSection::new(
                    "Score : ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ]),
            transform: Transform {
                translation: Vec3::new(-380., 360., 10.),
                ..default()
            },
            ..default()
        },
        GlobalScoreText,
    ));
    commands.spawn((
        Text2dBundle {
            visibility:Visibility::Visible,
            text_anchor: Anchor::TopLeft,

            text: Text::from_sections([
                TextSection::new(
                    "Hold Shift",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    " and ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE.with_alpha(0.3),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Rotate",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::new(
                    " to\nActivate ",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE.with_alpha(0.3),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Riffle Mode",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            ]),
            transform: Transform {
                translation: Vec3::new(100., -340., 10.),
                ..default()
            },
            ..default()
        },
        RiffleModeState,
    ));
}
