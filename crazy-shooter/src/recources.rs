use bevy::prelude::*;

use crate::EnemyName;

pub struct EnemyImages {
    pub troll_hurt: Handle<Image>,
    pub troll_close: Handle<Image>,
    pub troll_far: Handle<Image>,
    pub troll_jumpscare: Handle<Image>,
    pub monster_main:Handle<Image>,
    pub monster_hurt:Handle<Image>
}
pub struct EnemySounds {
    pub troll_jumpscare: Handle<AudioSource>,
    pub troll_pre_jumpscare: Handle<AudioSource>,
    pub troll_hurt:(Handle<AudioSource>,Handle<AudioSource>),
}
#[derive(Resource)]
pub struct AppState {
    pub exit: bool,
    pub timer: Timer,
}
impl AppState {
    pub fn handle(
        mut exit: EventWriter<AppExit>,
        mut app_state: ResMut<AppState>,
        time: Res<Time>,
    ) {
        app_state.timer.tick(time.delta());
        if app_state.exit && app_state.timer.just_finished() {
            exit.send(AppExit::Success);
        }
    }
    pub fn set_exit(&mut self) {
        self.exit = true;
        self.timer = Timer::from_seconds(4., TimerMode::Once);
    }
}
#[derive(Resource)]
pub struct Images {
    pub bullet: Handle<Image>,
    pub heart: Handle<Image>,
    pub aim_helper: (Handle<Image>, Handle<Image>),
    pub background: Handle<Image>,
    pub player: Handle<Image>,
    pub reload: Handle<Image>,
    pub enemies: EnemyImages,
}

#[derive(Resource)]
pub struct Sounds {
    pub background: Handle<AudioSource>,
    pub gun_shoot: Handle<AudioSource>,
    pub gun_reload: Handle<AudioSource>,
    pub gun_empty: Handle<AudioSource>,
    pub hurt:(Handle<AudioSource>,Handle<AudioSource>,Handle<AudioSource>),
    pub enemies: EnemySounds,
}
pub fn load_recources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Images {
        bullet: asset_server.load("bullet.png"),
        heart: asset_server.load("heart.png"),
        player: asset_server.load("player.png"),
         reload: asset_server.load("reload.png"),
        aim_helper: (
            asset_server.load("aim_helper.png"),
            asset_server.load("aim_helper_red.png"),
        ),
        background: asset_server.load("background.png"),
        enemies: EnemyImages {
            troll_hurt: asset_server.load("enemies/troll_face/hurt.png"),
            troll_close: asset_server.load("enemies/troll_face/close.png"),
            troll_far: asset_server.load("enemies/troll_face/far.png"),
            troll_jumpscare: asset_server.load("enemies/troll_face/jumpscare.png"),
            monster_main: asset_server.load("enemies/monster_face/main.png"),
            monster_hurt: asset_server.load("enemies/monster_face/hurt.png"),
        },
    });
    commands.insert_resource(Sounds {
        background: asset_server.load("background.ogg"),
        gun_shoot: asset_server.load("gun_shoot.ogg"),
        gun_reload: asset_server.load("reload.ogg"),
        gun_empty: asset_server.load("empty.ogg"),
        hurt:(asset_server.load("hurt_1.ogg"),asset_server.load("hurt_2.ogg"),asset_server.load("hurt_3.ogg")),
        enemies: EnemySounds {
            troll_hurt: (asset_server.load("enemies/troll_face/hurt_1.ogg"),asset_server.load("enemies/troll_face/hurt_2.ogg")),
            troll_jumpscare: asset_server.load("enemies/troll_face/jumpscare.ogg"),
            troll_pre_jumpscare: asset_server.load("enemies/troll_face/pre_jumpscare.ogg"),
        },
    });
    commands.insert_resource(AppState {
        exit: false,
        timer: Timer::from_seconds(0., TimerMode::Once),
    });
    commands.insert_resource(Global {
        spawn_rate: 0.1,
        score: 0,
        cummulated_spawn: 1.,
        achievements: Achievements::DEFAULT,
        spawn_timer: Timer::from_seconds(1., TimerMode::Repeating),
        timer_count:1,
        difficulty: 1,
    })
}
pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_recources)
            .add_systems(Update, AppState::handle);
    }
}

#[derive(Resource)]
pub struct Global {
    pub spawn_rate: f32,
    pub cummulated_spawn: f32,
    pub timer_count:u32,
    pub difficulty:u8,
    pub spawn_timer: Timer,
    pub score: u32,
    pub achievements: Achievements,
}
impl Global {
    pub fn increase_score(&mut self,enemy_name: EnemyName, mut score_text:Mut<'_, bevy::prelude::Text>) {
        self.score += match enemy_name {
            EnemyName::TrollFace => 3,
            EnemyName::BoyFace => 1,
            EnemyName::ExcitedFace => 10,
            EnemyName::GrandPaFace => 5,
            EnemyName::CutieFace => 20,
        };
    score_text.sections[1].value=self.score.to_string();
    }    
}

pub struct SpawnRates;
impl SpawnRates {
    pub const CHILD_FACE: f32 = 0.4;
    pub const TROLL_FACE: f32 = 0.2;
    pub const EXCITED_FACE: f32 = 0.1;
    pub const GRANDPA_FACE: f32 = 0.05;
    pub const CUTIE_FACE: f32 = 0.03;
    pub const MONSTER_FACE: f32 = 0.01;
    pub const ARRAY: [f32; 5] = [
        SpawnRates::CHILD_FACE,
        SpawnRates::TROLL_FACE,
        SpawnRates::EXCITED_FACE,
        SpawnRates::GRANDPA_FACE,
        SpawnRates::CUTIE_FACE,
    ];
}

pub struct Scores;
impl Scores {
    pub fn get(name: EnemyName) -> u8 {
        match name {
            EnemyName::TrollFace => 1,
            EnemyName::BoyFace => 3,
            EnemyName::ExcitedFace => 10,
            EnemyName::GrandPaFace => 15,
            EnemyName::CutieFace => 20,
        }
    }
}

#[derive(Component)]
pub struct GlobalScoreText;

pub struct Achievements {
    pub kill_troll_face: bool,
}
impl Achievements {
    pub const DEFAULT: Achievements = Achievements {
        kill_troll_face: false,
    };
}
