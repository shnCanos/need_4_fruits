use bevy::prelude::*;
use crate::common_components::MainCamera;

//region Import Modules
mod common_components;
mod fruit_plugin;
mod common_systems;
mod controls;
mod player_plugin;
//endregion

//region Consts

//region Assets
const FRUIT_ASSETS_PATH: [&str; 2] = [
    "watermelon.png",
    "watermelon2.png",
];
const NINJA_PATH: &str = "ninja_xente.png";
const AIM_PATH: &str = "aim.png";
const FRUITS_SCALE: Vec3 = Vec3::new(0.1, 0.1, 1.0);
const PLAYER_SCALE: Vec3 = FRUITS_SCALE;
const AIM_SCALE: Vec3 = FRUITS_SCALE;

const FRUITS_SIZE: Vec2 = Vec2::new(1000. * FRUITS_SCALE.x, 1000. * FRUITS_SCALE.y);
const PLAYER_SIZE: Vec2 = Vec2::new(1000. * PLAYER_SCALE.x, 1000. * PLAYER_SCALE.y);
//endregion

//region Game Consts
const FRUIT_SPEED: f32 = 8.;
const FRUITS_GRAVITY: f32 = 0.05;

const PLAYER_SPEED: f32 = 10.;
const PLAYER_GRAVITY: f32 = 0.6;
const PLAYER_FAST_FALLING_SPEED: f32 = -20.;
const PLAYER_GRAVITY_ON_WALL: f32 = 0.8;
const PLAYER_JUMP: f32 = 15.;
const PLAYER_HORIZONTAL_JUMP_WALL: f32 = 60.;
const PLAYER_VERTICAL_JUMP_WALL: f32 = 7.;
const JUMP_OFF_WALL_SPEED_ATTRITION: f32 = 5.;
const MAX_PLAYER_JUMPS_MIDAIR: usize = 99;
//endregion

//endregion

//region Global structs definitions
struct TexturesHandles {
    fruits: Vec<Handle<Image>>,
    ninja: Handle<Image>,
    aim: Handle<Image>,
}

struct KeyboardControls {
    jump: Vec<KeyCode>,
    fast_fall: Vec<KeyCode>,
    right: Vec<KeyCode>,
    left: Vec<KeyCode>,
}

impl KeyboardControls {
    pub fn is_pressed(kb: &Res<Input<KeyCode>>, keys: &Vec<KeyCode>) -> bool {
        let mut keys = keys.clone();
        keys.retain(|x| kb.pressed(*x));
        !keys.is_empty()
    }
    pub fn is_just_pressed(kb: &Res<Input<KeyCode>>, keys: &Vec<KeyCode>) -> bool {
        let mut keys = keys.clone();
        keys.retain(|x| kb.just_pressed(*x));
        !keys.is_empty()
    }
}

//endregion

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_plugin(fruit_plugin::FruitPlugin)
        .add_plugin(common_systems::CommonSystems)
        .add_plugin(controls::ControlsPlugin)
        .add_plugin(player_plugin::PlayerPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // window_res: Res<Windows>,
) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);

    //region Add asset handles
    commands.insert_resource(
       TexturesHandles {
           fruits: FRUIT_ASSETS_PATH.iter().map( |x| asset_server.load(*x) ).collect(),
           ninja: asset_server.load(NINJA_PATH),
           aim: asset_server.load(AIM_PATH),
       }
   );
    //endregion

    //region Insert WindowSize resource
    // let window = window_res.get_primary().unwrap();
    // commands.insert_resource(
    //     WindowSize {
    //         width: window.width(),
    //         height: window.height(),
    //     }
    // );
    //endregion

}