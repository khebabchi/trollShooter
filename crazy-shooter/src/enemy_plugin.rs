use crate::{AppState, Bullet, Global, GlobalScoreText, HealthCount, Images, Player, Sounds, SpawnRates};
use bevy::{
    audio::Volume,
    ecs::{bundle, system::EntityCommands},
    prelude::*,
};
use rand::{distributions::WeightedIndex, prelude::*};
////////////////
fn perpendicular_vector(vec: Vec3) -> Vec3 {
    let Vec3 { x, y, z } = vec;
    Vec3::new(-y, x, z) // Counterclockwise rotation by π/2
}

//////////////////////////
#[derive(Clone, Copy, Debug)]
pub enum EnemyName {
    TrollFace,
    BoyFace,
    ExcitedFace,
    GrandPaFace,
    CutieFace,
}
impl EnemyName {
    const ARRAY: [EnemyName; 5] = [
        EnemyName::TrollFace,
        EnemyName::BoyFace,
        EnemyName::ExcitedFace,
        EnemyName::GrandPaFace,
        EnemyName::CutieFace,
    ];
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
    pub fn new(name: EnemyName) -> Enemy {
        Enemy {
            width: match name {
                EnemyName::TrollFace => 64.,
                _ => todo!(),
            },
            health: match name {
                EnemyName::TrollFace => 3,
                _ => todo!(),
            },
            name,
            timer: Timer::from_seconds(0., TimerMode::Once),
            speed: 50.,
            is_hurt: false,
            states: vec![],
            velocity: Vec3::new(0., 0., 0.),
        }
    }

    pub fn sprite(enemy_name: EnemyName, images: &Res<Images>) -> Handle<Image> {
        match enemy_name {
            EnemyName::TrollFace => images.enemies.troll_far.clone(),
            EnemyName::BoyFace => todo!(),
            EnemyName::ExcitedFace => todo!(),
            EnemyName::GrandPaFace => todo!(),
            EnemyName::CutieFace => todo!(),
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn name(&self) -> EnemyName {
        self.name
    }

    pub fn push_back(
        &mut self,
        duration: f32,
        velocity:Vec3
    ) {
        self.timer = Timer::from_seconds(duration, TimerMode::Once);
        self.speed = -300.;
        self.is_hurt = true;
        self.velocity = velocity;
        
    }

    ////////////////////////////////////////////////
    ////////////////////////////////////////////////
    ////////////////////////////////////////////////

    pub fn handle_sprite(mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>) {
        for (mut sprite, transform) in enemy_query.iter_mut() {
            sprite.flip_y = transform.translation.x.is_sign_positive();
        }
    }

    fn handle_timer(mut enemy_query: Query<&mut Enemy>, time: Res<Time>) {
        for mut enemy in enemy_query.iter_mut() {
            enemy.timer.tick(time.delta());
            if enemy.timer.finished() {
                enemy.is_hurt = false;
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
                if enemy.timer.finished() {
                    if enemy.speed != 50. {
                        enemy.speed = 50.;
                    }
                    enemy.velocity = -transform.translation.normalize();
                    transform.translation += enemy.velocity * time.delta_seconds() * enemy.speed;
                } else {
                    let velocity = -transform.translation.normalize();
                    transform.translation +=
                        enemy.velocity * time.delta_seconds() * enemy.speed;
                    enemy.speed /= 1.15;
                }
            } else {
                if let Ok(mut transform) = jumpscare_query.get_single_mut() {
                    let mut rng = rand::thread_rng();
                    let mut rng1 = rand::thread_rng();

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
    ) {
        let pi = std::f32::consts::PI;
        commands.spawn((
            SpriteBundle {
                texture: Enemy::sprite(enemy_name, images),
                transform: Transform {
                    translation,
                    rotation: Quat::from_rotation_z(
                        (translation.y / translation.x).atan() - pi / 2.,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            Enemy::new(enemy_name),
        ));
    }

    pub fn handle_collision(
        mut commands: Commands,
        mut enemy_query: Query<(Entity, &mut Enemy, &mut Transform), Without<Bullet>>,
        bullet_query: Query<(Entity, &Bullet, &Transform), Without<Enemy>>,
        mut score_text_query: Query<
            &mut Text,
            (With<GlobalScoreText>, Without<Enemy>, Without<Bullet>),
        >,
        sounds: Res<Sounds>,
        mut global: ResMut<Global>,
    ) {
        for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            for (bullet_entity, bullet, bullet_transform) in bullet_query.iter() {
                if bullet.hit_enemy(&enemy, &enemy_transform, bullet_transform) {
                    commands.entity(bullet_entity).despawn();
                    
                    if enemy.health == 1 {
                        global.increase_score(enemy.name, score_text_query.single_mut());
                        commands.entity(enemy_entity).despawn();
                        commands.spawn(AudioBundle {
                            source: sounds.enemies.troll_hurt.1.clone(),
                            settings: PlaybackSettings::DESPAWN,
                            ..default()
                        });
                    } else {
                        enemy.health -= 1;
                        global.score+=1;
                        let velocity=(-enemy_transform.translation.clone().normalize()-2.*Vec3::new(enemy_transform.translation.x-bullet_transform.translation.x,enemy_transform.translation.y-bullet_transform.translation.y,0.).normalize()).normalize();
                        enemy.push_back(0.7, velocity);
                        commands.spawn(AudioBundle {
                            source: sounds.enemies.troll_hurt.0.clone(),
                            settings: PlaybackSettings::DESPAWN,
                            ..default()
                        });
                    }
                }
            }
        }
    }

    fn handle_by_name(
        mut enemy_query: Query<(Entity, &mut Transform, &mut Handle<Image>, &mut Enemy),Without<HealthCount>>,
        mut health_query:Query<&HealthCount>,
        
        time: Res<Time>,
        images: Res<Images>,
        sounds: Res<Sounds>,
        mut commands: Commands,
    ) {
        for (entity, mut transform, mut texture, mut enemy) in enemy_query.iter_mut() {
            match enemy.name {
                EnemyName::TrollFace => {
                    let mut rng = rand::thread_rng();
                    let x = transform.translation.x;
                    let y = transform.translation.y;

                    if (x * x + y * y).sqrt() < 150. && health_query.single().health==1{
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
                _ => todo!("Not implemented : handle_by_name {:?}", enemy.name),
            }
        }
    }
    pub fn random_spawn(
        mut global: ResMut<Global>,
        mut commands: Commands,
        images: Res<Images>,
        time: Res<Time>,
    ) {
        let mut rng = rand::thread_rng();
        global.spawn_timer.tick(time.delta());
        if global.spawn_timer.finished() {
            global.timer_count += 1;
            let mut t = global.timer_count;
            while t != 1 && t % 2 == 0 {
                t /= 2;
            }
            if t == 1 {
               // global.spawn_rate *= 1.2;
                global.difficulty+=1;
                info!("difficiulty : {}  |  spawn rate : {}",global.difficulty,global.spawn_rate)
            }
            global.cummulated_spawn += global.spawn_rate;
            while global.cummulated_spawn > 1. {
                global.cummulated_spawn -= 1.;
                // randomize spawn location :
                let section = rng.gen_range(1..=4);
                let (wide, tite) = (if rng.gen_range(0..=1)==1 {rng.gen_range(-500.0..-100.0)}else{rng.gen_range(100.0..500.0)}, rng.gen_range(400.0..500.0));
                let translation = match section {
                    1 => Vec3::new(wide, tite, 2.),
                    2 => Vec3::new(wide, -tite, 2.),
                    3 => Vec3::new(tite, wide, 2.),
                    4 => Vec3::new(-tite, wide, 2.),
                    _ => todo!("Error : range out of bounds"),
                };
                // Create a weighted index based on the spawn rates
                let weighted_index = WeightedIndex::new(SpawnRates::ARRAY).unwrap();

                // Randomly choose a monster index
                let chosen_index = weighted_index.sample(&mut rng);

                // Get the chosen monster
                let chosen_monster = EnemyName::ARRAY[chosen_index];
                Enemy::spawn(EnemyName::TrollFace, &mut commands, &images, translation);
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
                Enemy::handle_sprite,
                Enemy::update_position,
                Enemy::handle_by_name,
                Enemy::handle_timer,
                Enemy::random_spawn,
            ),
        );
    }
}
