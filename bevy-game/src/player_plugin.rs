use bevy::{app::Plugin, audio::Volume, prelude::*, window::PrimaryWindow};
use rand::Rng;
use std::time::Duration;

use crate::{Enemy, Images, Sounds};
#[derive(Debug)]
pub enum PlayerState {
    Free,
    Rotating,
    Shooting,
    Reloading,
}
#[derive(Component)]
pub struct RiffleModeEnabled;
#[derive(Component)]
pub struct RiffleModeDisabled;

#[derive(Component)]
pub struct RiffleMode;

#[derive(Component)]
pub struct BulletCount;
#[derive(Component)]
pub struct Player {
    /// rotation speed in radians per second
    pub rotation_speed: f32,
    pub state: PlayerState,
    pub timer: Timer,
    pub shooting_couldown: Timer,
    pub bullet_count: i8,
    pub riffle_mode: bool,
    pub kill_count: i8,
    pub used_shift: bool,
}
#[derive(Component)]
pub struct Bullet {
    pub velocity: Vec3,
    width: f32,
}
impl Bullet {
    pub fn spawn(
        commands: &mut Commands,
        texture: Handle<Image>,
        sound_effect: Handle<AudioSource>,
        player_transform: &Transform,
    ) {
        let rotation = player_transform.rotation.to_euler(EulerRot::XYZ).2;
        let bullet_velocity = Vec3::new(rotation.cos(), rotation.sin(), 0.0) * 2000.0;
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.)
                        + player_transform.translation
                        + player_transform.rotation * Vec3::new(35.0, -10.0, 0.0),
                    rotation: player_transform.rotation,
                    ..Default::default()
                },
                ..Default::default()
            },
            Bullet {
                velocity: bullet_velocity,
                width: 21.,
            },
        ));
        commands.spawn(AudioBundle {
            source: sound_effect,
            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.3)),
            ..default()
        });
    }
    pub fn hit_enemy(
        &self,
        enemy: &Enemy,
        enemy_transform: &Transform,
        bullet_transform: &Transform,
    ) -> bool {
        let distance = Vec3::new(
            enemy_transform.translation.x - bullet_transform.translation.x,
            enemy_transform.translation.y - bullet_transform.translation.y,
            0.,
        )
        .length();
        if distance <= (enemy.width() / 2. + self.width() / 2.) {
            true
        } else {
            false
        }
    }
    pub fn width(&self) -> f32 {
        self.width
    }
}

pub fn update_bullet_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Bullet, Entity)>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window.single();
    for (mut transform, mut bullet, entity) in query.iter_mut() {
        if transform.translation.x >= window.width() / 2.0 + 5.0
            || transform.translation.x <= -window.width() / 2.0 - 5.0
            || transform.translation.y >= window.height() / 2.0 + 5.0
            || transform.translation.y <= -window.height() / 2.0 - 5.0
        {
            commands.entity(entity).despawn();
        }

        transform.translation += bullet.velocity * time.delta_seconds();
        if bullet.velocity.clone().length() > 1800.0 {
            bullet.velocity /= 1.2
        } else {
            bullet.velocity /= 1.04
        };
    }
}
/*pub fn update_aimhelper_position(
    time: Res<Time>,
    mut aim_helper_query: Query<&mut Transform, (With<AimHelper>, Without<Player>)>,
    mut player_query: Query<&Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        let rotation = player_transform.rotation.to_euler(EulerRot::XYZ).2;
        let window = window.single();
        for mut transform in aim_helper_query.iter_mut() {
            if rotation.cos().abs() > rotation.sin().abs() {
                transform.translation=Vec3::new(rotation.cos().signum()*795./2.0,rotation.sin()*(795./2.0)/rotation.cos().abs(),4.)
            } else {
                transform.translation=Vec3::new(rotation.cos()*(795./2.0)/rotation.sin().abs(),rotation.sin().signum()*795./2.0,4.)
            }
        }
    }
}*/

impl Player {
    pub fn update_kill_count(&mut self, text: &mut Text) {
        if self.riffle_mode {
            self.kill_count = -1;
        } else if self.kill_count < 2 {
            self.kill_count += 1;
            text.sections[1].value = self.kill_count.to_string();
        }
    }

    pub fn rotate(&mut self, side: f32, fast: bool) {
        if let PlayerState::Reloading = self.state {
            self.rotation_speed = f32::to_radians(10.0) * side;
        } else if !self.is_shooting() {
            self.timer = Timer::new(
                Duration::from_millis(if fast && self.riffle_mode { 700 } else { 500 }),
                TimerMode::Once,
            );
            self.state = PlayerState::Rotating;
            self.rotation_speed = f32::to_radians(if fast && self.riffle_mode {
                100.0
            } else {
                60.0
            }) * side;
        }
    }
    pub fn is_rotating(&self) -> bool {
        if let PlayerState::Rotating = self.state {
            if !self.timer.finished() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn is_shooting(&self) -> bool {
        if let PlayerState::Shooting = self.state {
            if !self.timer.finished() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn is_reloading(&self) -> bool {
        if let PlayerState::Reloading = self.state {
            if !self.timer.finished() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn shoot(&mut self, state: bool) {
        info!("reloading");
        // info!("{}", self.riffle_mode);
        self.timer = Timer::new(
            Duration::from_millis(if self.riffle_mode && state { 50 } else { 250 }),
            TimerMode::Once,
        );
        self.shooting_couldown = Timer::new(
            Duration::from_millis(if self.riffle_mode && state { 150 } else { 400 }),
            TimerMode::Once,
        );
        self.state = PlayerState::Shooting;
        self.rotation_speed = f32::to_radians(100.0);
        self.bullet_count -= 1;
    }
    pub fn reload(&mut self) {
        self.timer = Timer::new(Duration::from_millis(1000), TimerMode::Once);
        self.state = PlayerState::Reloading;
    }
    pub fn handle_state(
        mut player_query: Query<(&mut Player, &mut Transform)>,
        time: Res<Time>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
    ) {
        if let Ok((mut player, mut transform)) = player_query.get_single_mut() {
            let mut rng = rand::thread_rng();
            if player.is_rotating() || player.is_reloading() {
                transform.rotate_z(player.rotation_speed * time.delta_seconds());
                player.rotation_speed /= 1.1;
            } else if player.is_shooting() {
                player.rotation_speed /= 1.1;
                transform.rotate_z(
                    rng.gen_range(
                        if player.riffle_mode && keyboard_input.pressed(KeyCode::ShiftLeft) {
                            -1.0..1.
                        } else {
                            -1.5..0.4
                        },
                    ) * player.rotation_speed
                        * time.delta_seconds(),
                );
            }
        };
    }
    pub fn is_cooldown(&self) -> bool {
        !self.shooting_couldown.finished()
    }
    pub fn update_timer(
        mut player_query: Query<(&mut Player, &mut Handle<Image>)>,
        mut riffle_true_query: Query<
            &mut Visibility,
            (
                With<RiffleModeEnabled>,
                Without<BulletCount>,
                Without<Player>,
            ),
        >,
        mut riffle_false_query: Query<
            (&mut Visibility, &mut Text),
            (
                With<RiffleModeDisabled>,
                Without<RiffleModeEnabled>,
                Without<BulletCount>,
                Without<Player>,
            ),
        >,
        mut ammo_query: Query<
            &mut Text,
            (
                Without<Player>,
                Without<RiffleModeEnabled>,
                With<BulletCount>,
            ),
        >,
        time: Res<Time>,
        images: Res<Images>,
    ) {
        if let Ok((mut player, mut texture)) = player_query.get_single_mut() {
            if !player.timer.finished() {
                player.timer.tick(time.delta());
            } else {
                if player.kill_count == 2 || player.riffle_mode {
                    let mut riffle_true_visibility = riffle_true_query.single_mut();
                    let (mut riffle_false_visibility, mut riffle_false_text) =
                        riffle_false_query.single_mut();
                    let check = if let PlayerState::Reloading = player.state {
                        let used_shift = player.used_shift;
                        player.used_shift = false;
                        !(used_shift || !player.riffle_mode)
                    } else {
                        if player.kill_count == 2 {player.bullet_count=std::cmp::max(player.bullet_count, 10)}
                        player.kill_count == 2 || player.riffle_mode
                    };
                    player.riffle_mode = check;

                    player.update_kill_count(&mut *riffle_false_text);

                    *riffle_true_visibility = if player.riffle_mode {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                    *riffle_false_visibility = if !player.riffle_mode {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
                if let PlayerState::Free = player.state {
                } else {
                    // info!("{:?}", player.state);
                }
                if let PlayerState::Reloading = player.state {
                    *texture = images.player.clone();
                    // info!( "count :{}  riffle:{}",player.kill_count, player.riffle_mode   );

                    player.bullet_count = 20;
                    ammo_query.single_mut().sections[2].value = player.bullet_count.to_string();
                    ammo_query.single_mut().sections[2]
                        .style
                        .color
                        .set_alpha(1.);
                }
                player.state = PlayerState::Free;
            }
            if player.is_cooldown() {
                player.shooting_couldown.tick(time.delta());
            }
        };
    }
    pub fn input_check(
        time: Res<Time>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<(&mut Player, &mut Transform, &mut Handle<Image>)>,
        mut ammo_query: Query<&mut Text, (Without<Player>, With<BulletCount>)>,
        images: Res<Images>,
        sounds: Res<Sounds>,
        mut commands: Commands,
    ) {
        if let Ok((mut player, mut transform, mut texture)) = player_query.get_single_mut() {
            if (keyboard_input.just_pressed(KeyCode::KeyR)
                || (keyboard_input.just_pressed(KeyCode::Space) && player.bullet_count == 0))
                && !player.is_shooting()
                && !player.is_reloading()
            {
                if player.bullet_count == 20 {
                    commands.spawn(AudioBundle {
                        source: sounds.gun_empty.clone(),
                        settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(2.)),
                        ..default()
                    });
                } else {
                    player.reload();
                    commands.spawn(AudioBundle {
                        source: sounds.gun_reload.clone(),
                        settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.5)),
                        ..default()
                    });
                    *texture = images.reload.clone();
                }
            }
            if ((!player.riffle_mode && keyboard_input.just_pressed(KeyCode::Space))
                || (player.riffle_mode && keyboard_input.pressed(KeyCode::Space)))
                && !player.is_shooting()
                && !player.is_cooldown()
                && !player.is_reloading()
            {
                if player.bullet_count > 0 {
                    let state = keyboard_input.pressed(KeyCode::ShiftLeft);
                    player.shoot(state);
                    if state {
                        player.used_shift = true
                    }
                    ammo_query.single_mut().sections[2].value = player.bullet_count.to_string();
                    if player.bullet_count == 0 {
                        ammo_query.single_mut().sections[2]
                            .style
                            .color
                            .set_alpha(0.5);
                    }

                    Bullet::spawn(
                        &mut commands,
                        images.bullet.clone(),
                        sounds.gun_shoot.clone(),
                        &transform,
                    );
                }
            }
            if (player.timer.finished()
                || player.is_reloading()
                || player.timer.elapsed().as_millis() < 100)
                && !player.is_shooting()
            {
                if (keyboard_input.pressed(KeyCode::ArrowLeft)
                    || keyboard_input.pressed(KeyCode::KeyA))
                    && !(keyboard_input.pressed(KeyCode::ArrowRight)
                        || keyboard_input.pressed(KeyCode::KeyD))
                {
                    player.rotate(1.0, keyboard_input.pressed(KeyCode::ShiftLeft));

                    transform.rotate_z(player.rotation_speed * time.delta_seconds());
                }

                if !(keyboard_input.pressed(KeyCode::ArrowLeft)
                    || keyboard_input.pressed(KeyCode::KeyQ))
                    && (keyboard_input.pressed(KeyCode::ArrowRight)
                        || keyboard_input.pressed(KeyCode::KeyD))
                {
                    player.rotate(-1.0, keyboard_input.pressed(KeyCode::ShiftLeft));
                    transform.rotate_z(player.rotation_speed * time.delta_seconds());
                }
            }
        }
        // update the player rotation around the Z axis (perpendicular to the 2D plane of the screen)
    }
}

#[derive(Component)]
pub struct HealthCount {
    pub health: i8,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            FixedUpdate,
            (
                Player::input_check,
                Player::handle_state,
                Player::update_timer,
                update_bullet_position,
            ),
        );
    }
}
