use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{Enemy, Images, Player};
#[derive(Component)]
pub struct AimHelper;
impl AimHelper {
    pub fn update_rotation(
        mut aim_helper_query: Query<&mut Transform, (With<AimHelper>, Without<Player>)>,
        mut player_query: Query<&Transform, With<Player>>,
    ) {
        if let Ok(player_transform) = player_query.get_single_mut() {
            if let Ok(mut transform) = aim_helper_query.get_single_mut() {
                *transform = Transform {
                    translation: Vec3::new(0., 0., 1.)
                        + player_transform.translation
                        + player_transform.rotation * Vec3::new(3., -8.0, 0.0),
                    rotation: player_transform.rotation,
                    ..Default::default()
                };
            }
        }
    }
    pub fn update_detection(
        mut aim_helper_query: Query<
            (&mut Handle<Image>, &mut Sprite),
            (With<AimHelper>, Without<Player>),
        >,
        player_query: Query<(&Transform, &Player), Without<AimHelper>>,
        enemy_query: Query<(&Transform, &Enemy), (Without<AimHelper>, Without<Player>)>,
        images: Res<Images>,
    ) {
        if let Ok((transform, player)) = player_query.get_single() {
            if let Ok((mut texture, mut sprite)) = aim_helper_query.get_single_mut() {
                if player.bullet_count == 0 || player.is_reloading(){
                    sprite.color.set_alpha(0.);
                } else {
                    sprite.color.set_alpha(0.05);
                }
                let aim_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
                let mut is_pointed = false;
                for (enemy_transform, enemy) in enemy_query.iter() {
                    let decalage = transform.rotation * Vec3::new(3., -8.0, 0.0);
                    let x = enemy_transform.translation.x - decalage.x;
                    let y = enemy_transform.translation.y - decalage.y;

                    let watr = (x * x + y * y).sqrt();
                    let enemy_angle = (y / x).atan() + if x < 0. { PI * y.signum() } else { 0. };
                    let range_angle = (enemy.width() / (2. * watr)).atan();
                    if aim_angle <= enemy_angle + range_angle
                        && aim_angle >= enemy_angle - range_angle
                    {is_pointed=true;}
                }
                if is_pointed {
                    *texture = images.aim_helper.1.clone();
                } else {
                    *texture = images.aim_helper.0.clone();
                }
            }
        }
    }
}

pub struct AimHelperPlugin;

impl Plugin for AimHelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (AimHelper::update_detection, AimHelper::update_rotation),
        );
    }
}
