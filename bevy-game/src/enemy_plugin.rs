use std::f32::consts::PI;

use crate::{
    AppState, Bullet, BulletCount, Global, GlobalScoreText, HealthCount, Images, Player,
    RiffleModeDisabled, Sounds,
};
use bevy::{audio::Volume, prelude::*};
use rand::prelude::*;
////////////////
fn perpendicular_vector(vec: Vec3) -> Vec3 {
    let Vec3 { x, y, z } = vec;
    Vec3::new(-y, x, z) // Counterclockwise rotation by π/2
}

//////////////////////////
#[derive(Clone, Debug)]
pub enum EnemyName {
    TrollFace,
    BoyFace(Option<Timer>),
    ExcitedFace,
    Monster(Vec3),
}
#[derive(Component)]
pub struct Jumpscare;
#[derive(Component)]
pub struct Enemy {
    width: f32,
    name: EnemyName,
    health: i8,
    speed: f32,
    timer: Timer,
    is_hurt: bool,
    pub states: Vec<bool>,
    pub velocity: Vec3,
}
impl Enemy {
    pub fn new(name: EnemyName, velocity: Vec3) -> Enemy {
        Enemy {
            width: match name {
                EnemyName::TrollFace | EnemyName::Monster(_) => 64.,
                _ => todo!(),
            },
            health: match name {
                EnemyName::TrollFace => 3,
                EnemyName::Monster(_) => 2,
                _ => todo!(),
            },

            timer: Timer::from_seconds(0., TimerMode::Once),
            speed: match name {
                EnemyName::TrollFace => 50.,
                EnemyName::Monster(_) => 100.,
                _ => todo!(),
            },
            name,
            is_hurt: false,
            states: vec![],
            velocity,
        }
    }

    pub fn sprite(enemy_name: EnemyName, images: &Res<Images>) -> Handle<Image> {
        match enemy_name {
            EnemyName::TrollFace => images.enemies.troll_far.clone(),
            EnemyName::BoyFace(_) => todo!(),
            EnemyName::ExcitedFace => todo!(),
            EnemyName::Monster(_) => images.enemies.monster_main.clone(),
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn name(&self) -> EnemyName {
        self.name.clone()
    }

    pub fn push_back(&mut self, duration: f32, velocity: Vec3) {
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
        self.speed = -300.;
        self.is_hurt = true;
        self.velocity = velocity;
    }

    ////////////////////////////////////////////////
    ////////////////////////////////////////////////
    ////////////////////////////////////////////////

    fn handle_timer(mut enemy_query: Query<&mut Enemy>, time: Res<Time>) {
        for mut enemy in enemy_query.iter_mut() {
            enemy.timer.tick(time.delta());
            if enemy.timer.finished() {
                enemy.is_hurt = false;
            }
        }
    }

    pub fn update_health(mut healthcount_query: Query<(&HealthCount, &mut Text)>) {
        if let Ok((count, mut text)) = healthcount_query.get_single_mut() {
            text.sections[0].style.color = if count.health == 8 {
                Color::LinearRgba(LinearRgba {
                    red: 0.,
                    green: 1.,
                    blue: 0.,
                    alpha: 1.,
                })
            } else if count.health == 1 {
                Color::LinearRgba(LinearRgba {
                    red: 1.,
                    green: 0.2,
                    blue: 0.2,
                    alpha: 1.,
                })
            } else {
                Color::WHITE
            }
        }
    }
    ////////////////////////////////////////////////
    ////////////////////////////////////////////////
    ////////////////////////////////////////////////

    pub fn update_position(
        time: Res<Time>,
        mut enemy_query: Query<(&mut Transform, &mut Enemy, Entity), Without<Jumpscare>>,
        mut jumpscare_query: Query<&mut Transform, With<Jumpscare>>,
        mut healthcount_query: Query<(&mut HealthCount, &mut Text)>,
        mut app_state: ResMut<AppState>,
        mut commands: Commands,
        images: Res<Images>,
        sounds: Res<Sounds>,
    ) {
        for (mut transform, mut enemy, entity) in enemy_query.iter_mut() {
            if Vec2::new(transform.translation.x, transform.translation.y).length()
                > enemy.width() / 2. + 5.
            {
                enemy.timer.tick(time.delta());
                match enemy.name.clone() {
                    EnemyName::Monster(ref destination) => {
                        if (transform.translation - *destination).length() < 2.0 {
                            commands.entity(entity).despawn();
                            info!("despawn");
                        }
                        if enemy.timer.finished() {
                            if enemy.speed != 50. {
                                enemy.speed = 50.;
                            }
                            enemy.velocity = (*destination - transform.translation).normalize();
                            transform.translation +=
                                enemy.velocity * time.delta_seconds() * enemy.speed;
                        } else {
                            transform.translation +=
                                enemy.velocity * time.delta_seconds() * enemy.speed;
                            enemy.speed /= 1.15;
                        }
                    }
                    _ => {
                        if enemy.timer.finished() {
                            if enemy.speed != 50. {
                                enemy.speed = 50.;
                            }
                            enemy.velocity = -transform.translation.normalize();
                            transform.translation +=
                                enemy.velocity * time.delta_seconds() * enemy.speed;
                        } else {
                            transform.translation +=
                                enemy.velocity * time.delta_seconds() * enemy.speed;
                            enemy.speed /= 1.15;
                        }
                    }
                }
            } else {
                if let Ok(_) = jumpscare_query.get_single_mut() {
                    

                    // scale of end screen
                } else {
                    let (mut health_count, mut text) = healthcount_query.single_mut();

                    if health_count.health == 1 {
                        commands.spawn((
                            SpriteBundle {
                                texture: images.enemies.troll_jumpscare.clone(),
                                transform: Transform {
                                    translation: Vec3::new(0., 0., 5.),
                                    ..Default::default()
                                },
                                ..default()
                            },
                            AudioBundle {
                                source: sounds.enemies.troll_jumpscare.clone(),
                                settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.1)),
                                ..default()
                            },
                            Jumpscare,
                        ));
                        app_state.set_exit();
                    } else {
                        commands.spawn(AudioBundle {
                            source: match health_count.health {
                                2 => sounds.hurt.2.clone(),
                                3 => sounds.hurt.1.clone(),
                                _ => sounds.hurt.0.clone(),
                            },
                            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(0.5)),
                            ..default()
                        });
                        health_count.health -= 1;
                        text.sections[0].value = health_count.health.to_string();
                    }
                }
                commands.entity(entity).despawn();
            }
        }
    }

    pub fn spawn(
        enemy_name: EnemyName,
        commands: &mut Commands,
        images: &Res<Images>,
        translation: Vec3,
        rotation: Quat,
        flip_x: bool,
        flip_y: bool,
    ) {
        let mut sprite = Sprite::default();
        sprite.flip_x = flip_x;
        sprite.flip_y = flip_y;
        match enemy_name.clone() {
            EnemyName::TrollFace => {
                commands.spawn((
                    SpriteBundle {
                        sprite,
                        texture: Enemy::sprite(enemy_name.clone(), images),
                        transform: Transform {
                            translation,
                            rotation,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Enemy::new(enemy_name.clone(), Vec3::ZERO),
                ));
            }
            EnemyName::Monster(ref destination) => {
                commands.spawn((
                    SpriteBundle {
                        sprite,
                        texture: Enemy::sprite(enemy_name.clone(), images),
                        transform: Transform {
                            translation,
                            rotation,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Enemy::new(enemy_name.clone(), (*destination - translation).normalize()),
                ));
            }
            _ => todo!(),
        }
    }

    pub fn handle_collision(
        mut commands: Commands,
        mut enemy_query: Query<(Entity, &mut Enemy, &mut Transform), Without<Bullet>>,
        mut riffle_false_query: Query<
            &mut Text,
            (
                With<RiffleModeDisabled>,
                Without<BulletCount>,
                Without<HealthCount>,
            ),
        >,
        mut healthcount_query: Query<(&mut HealthCount, &mut Text)>,
        mut player_query: Query<&mut Player>,
        bullet_query: Query<(Entity, &Bullet, &Transform), Without<Enemy>>,
        mut score_text_query: Query<
            &mut Text,
            (
                With<GlobalScoreText>,
                Without<RiffleModeDisabled>,
                Without<HealthCount>,
            ),
        >,
        sounds: Res<Sounds>,
        mut global: ResMut<Global>,
    ) {
        if let Ok(mut player) = player_query.get_single_mut() {
            for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
                for (bullet_entity, bullet, bullet_transform) in bullet_query.iter() {
                    if bullet.hit_enemy(&enemy, &enemy_transform, bullet_transform) {
                        commands.entity(bullet_entity).despawn();

                        if enemy.health == 1 {
                            global
                                .increase_score(enemy.name.clone(), score_text_query.single_mut());
                            commands.entity(enemy_entity).despawn();
                            if let EnemyName::Monster(_) = enemy.name {
                                if healthcount_query.single_mut().0.health + 2 < 8 {
                                    healthcount_query.single_mut().0.health += 2;
                                } else {
                                    healthcount_query.single_mut().0.health = 8;
                                }
                                healthcount_query.single_mut().1.sections[0].value =
                                    healthcount_query.single_mut().0.health.to_string();
                            }

                            player.update_kill_count(&mut *riffle_false_query.single_mut());
                            commands.spawn(AudioBundle {
                                source: if let EnemyName::TrollFace = enemy.name {
                                    sounds.enemies.troll_hurt.1.clone()
                                } else {
                                    sounds.enemies.monster_hurt.1.clone()
                                },
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                        } else {
                            enemy.health -= 1;
                            global.score += 1;
                            let velocity = (-enemy_transform.translation.clone().normalize()
                                - 0.5
                                    * Vec3::new(
                                        enemy_transform.translation.x
                                            - bullet_transform.translation.x,
                                        enemy_transform.translation.y
                                            - bullet_transform.translation.y,
                                        0.,
                                    )
                                    .normalize())
                            .normalize();
                            enemy.push_back(0.7, velocity);

                            commands.spawn(AudioBundle {
                                source: if let EnemyName::TrollFace = enemy.name {
                                    sounds.enemies.troll_hurt.0.clone()
                                } else {
                                    sounds.enemies.monster_hurt.0.clone()
                                },
                                settings: PlaybackSettings::DESPAWN,
                                ..default()
                            });
                        }
                    }
                }
            }
        }
    }

    fn handle_by_name(
        mut enemy_query: Query<
            (&mut Transform, &mut Handle<Image>, &mut Enemy),
            Without<HealthCount>,
        >,
        health_query: Query<&HealthCount>,

        time: Res<Time>,
        images: Res<Images>,
    ) {
        for (mut transform, mut texture, enemy) in enemy_query.iter_mut() {
            match enemy.name {
                EnemyName::TrollFace => {
                    let mut rng = rand::thread_rng();
                    let x = transform.translation.x;
                    let y = transform.translation.y;

                    if (x * x + y * y).sqrt() < 150. && health_query.single().health == 1 {
                        *texture = images.enemies.troll_close.clone();
                        let shaking = perpendicular_vector(transform.translation.normalize())
                            * rng.gen_range(-20.0..20.0)
                            * 50.
                            * time.delta_seconds();
                        let more_speed =
                            -transform.translation.normalize() * 60. * time.delta_seconds();
                        transform.translation += shaking + more_speed;
                    } else {
                        *texture = if enemy.is_hurt {
                            images.enemies.troll_hurt.clone()
                        } else {
                            images.enemies.troll_far.clone()
                        }
                    }
                }
                EnemyName::Monster(_) => {
                    *texture = if enemy.is_hurt {
                        images.enemies.monster_hurt.clone()
                    } else {
                        images.enemies.monster_main.clone()
                    }
                }
                _ => todo!("Not implemented : handle_by_name {:?}", enemy.name),
            }
        }
    }
    pub fn random_spawn(
        mut global: ResMut<Global>,
        mut commands: Commands,
        images: Res<Images>,
        time: Res<Time>,
        enemy_query: Query<&Enemy>,
    ) {
        let mut rng = rand::thread_rng();
        global.spawn_timer.tick(time.delta());
        if global.spawn_timer.finished() {
            global.timer_count += 1;

            if global.timer_count % 8 == 0 {
                if global.difficulty <= 6 {
                    global.spawn_rate *= 1.1;
                } else {
                    global.spawn_rate *= 1.015;
                }

                global.difficulty += 1;
                info!(
                    "difficiulty : {}  |  spawn rate : {}",
                    global.difficulty, global.spawn_rate
                )
            }
            global.cummulated_spawn += global.spawn_rate;
            while global.cummulated_spawn > 1. {
                global.cummulated_spawn -= 1.;
                let mut is_monster = false;
                for enemy in enemy_query.iter() {
                    if let EnemyName::Monster(_) = enemy.name {
                        is_monster = true;
                    }
                }

                let mut chosen_monster = if rng.gen_range(0..100) > 90 && !is_monster {
                    EnemyName::Monster(Vec3::ZERO)
                } else {
                    EnemyName::TrollFace
                };

                let mut translation ;
                if let EnemyName::Monster(ref mut destination) = chosen_monster {
                    let section = rng.gen_range(1..=4);
                    translation = match section {
                        1 => Vec3::new(rng.gen_range(50.0..350.0), 420., 2.),
                        2 => Vec3::new(rng.gen_range(-350.0..-50.0), -420., 2.),
                        3 => Vec3::new(-420., rng.gen_range(50.0..350.0), 2.),
                        4 => Vec3::new(420., rng.gen_range(-350.0..-50.0), 2.),
                        _ => todo!("Error : range out of bounds"),
                    };
                    let a: f32 = match section {
                        1 => 400. - translation.x,
                        2 => 400. + translation.x,
                        3 => 400. - translation.y,
                        4 => 400. + translation.y,
                        _ => 400.,
                    };
                    let b_min: f32 = ((450. * 450.) - (a * a)).sqrt();
                    let b_max: f32 = ((500. * 500.) - (a * a)).sqrt();
                    info!("a = {a}  section = {section} |  [ {b_min} , {b_max} ]");
                    let b = rng.gen_range(b_min..b_max);
                    info!("b=  {b}");
                    *destination = match section {
                        1 => Vec3::new(420., 400. - b, 2.),
                        2 => Vec3::new(-420., b - 400., 2.),
                        3 => Vec3::new(b - 400., 420., 2.),
                        4 => Vec3::new(400. - b, -420., 2.),
                        _ => todo!("nothing here"),
                    };
                    let mut flip_x = false;
                    let mut flip_y = false;
                    let rotation = match section {
                        1 => Quat::from_rotation_z(-(b / a).atan() - PI / 2.),
                        2 => {
                            flip_x = true;
                            Quat::from_rotation_z(-(b / a).atan() + PI / 2.)
                        }
                        3 => Quat::from_rotation_z(-(b / a).atan()),
                        4 => {
                            flip_x = true;
                            Quat::from_rotation_z(-(b / a).atan() + PI)
                        }
                        _ => Quat::default(),
                    };

                    if rng.gen_range(1..=2) == 1 {
                        let tmp = *destination;
                        *destination = translation;
                        translation = tmp;
                        flip_y = true;
                    }
                    Enemy::spawn(
                        chosen_monster,
                        &mut commands,
                        &images,
                        translation,
                        rotation,
                        flip_x,
                        flip_y,
                    );
                } else {
                    let section = rng.gen_range(1..=4);
                    let (wide, tite) = (
                        if rng.gen_range(0..=1) == 1 {
                            rng.gen_range(-500.0..-100.0)
                        } else {
                            rng.gen_range(100.0..500.0)
                        },
                        rng.gen_range(400.0..500.0),
                    );
                    translation = match section {
                        1 => Vec3::new(wide, tite, 2.),
                        2 => Vec3::new(wide, -tite, 2.),
                        3 => Vec3::new(tite, wide, 2.),
                        4 => Vec3::new(-tite, wide, 2.),
                        _ => todo!("Error : range out of bounds"),
                    };
                    let rotation =
                        Quat::from_rotation_z((translation.y / translation.x).atan() - PI / 2.);
                    Enemy::spawn(
                        chosen_monster,
                        &mut commands,
                        &images,
                        translation,
                        rotation,
                        false,
                        translation.x.is_sign_positive(),
                    );
                }
            }
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            FixedUpdate,
            (
                Enemy::handle_collision,
                Enemy::update_position,
                Enemy::handle_by_name,
                Enemy::handle_timer,
                Enemy::random_spawn,
                Enemy::update_health,
            ),
        );
    }
}
