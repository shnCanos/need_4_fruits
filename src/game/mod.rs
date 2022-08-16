use bevy::prelude::*;
use crate::game::common_components::MainCamera;

//region Import Modules
mod common_components;
mod fruit_plugin;
mod common_systems;
mod controls;
mod player_plugin;
mod ui_plugin;
mod osu_reader;
mod beatmap_plugin;
//endregion

//region Consts

//region Assets
const FRUIT_ASSETS_PATH: [&str; 2] = [
    "watermelon.png",
    "watermelon2.png",
];
const NINJA_PATH: &str = "ninja_xente.png";
const AIM_PATH: &str = "aim.png";
const AURA_PATH: &str = "aura_thing.png";
const FRUITS_SCALE: Vec3 = Vec3::new(0.1, 0.1, 1.0);
const PLAYER_SCALE: Vec3 = FRUITS_SCALE;
const AIM_SCALE: Vec3 = FRUITS_SCALE;

const FRUITS_SIZE: Vec2 = Vec2::new(1000. * FRUITS_SCALE.x, 1000. * FRUITS_SCALE.y);
const PLAYER_SIZE: Vec2 = Vec2::new(600. * PLAYER_SCALE.x, 600. * PLAYER_SCALE.y);
//endregion

//region Game Consts

// Fruits
// Air
const FRUIT_SPEED: f32 = 25.;
const FRUITS_GRAVITY_UP: f32 = 0.25;
const FRUITS_GRAVITY_HOLD: f32 = 0.07;
const FRUITS_GRAVITY_FALL: f32 = 0.5;
// Beatmap
const BEATMAP_INITIAL_WAIT_TIME: f32 = 0.5;
const BEATMAP_MUSIC_OFFSET_TIME: f32 = 0.7;
const BEATMAP_FILE_NAME : &str = "beatMARIO - Night of Knights (alacat) [Hard].osu";
/// How much of the screen's horizontal width is spawnable for fruits (0.0-1.0)
const EFFECTIVE_SCREEN_WIDTH_PERCENT: f32 = 0.9;
// Fruit Part
const NUMBER_OF_FRUIT_PIECES: i32 = 4; // Has to be a perfect square
const MAX_FRUIT_PIECE_SPEED: f32 = 8.;

// Player variables
// Air
const PLAYER_SPEED: f32 = 10.;
const PLAYER_GRAVITY: f32 = 0.4;
const PLAYER_FAST_FALLING_SPEED: f32 = -20.;
const MAX_PLAYER_JUMPS_MIDAIR: usize = 1;
const PLAYER_JUMP: f32 = 15.;
// Wall
const PLAYER_GRAVITY_ON_WALL: f32 = 0.8;
const PLAYER_HORIZONTAL_JUMP_WALL: f32 = 60.;
const PLAYER_VERTICAL_JUMP_WALL: f32 = 7.;
const JUMP_OFF_WALL_SPEED_ATTRITION: f32 = 5.;
// Dash
const DASH_DURATION: f32 = 0.066; // The duration of a dash in seconds
const MAX_PLAYER_DASHES_MIDAIR: usize = 1;
const DASH_SPEED: f32 = 70.;
//endregion

//endregion

//region Global structs definitions
struct TexturesHandles {
    fruits: Vec<Handle<Image>>,
    fruits_pieces_texture_atlas: Vec<Handle<TextureAtlas>>,
    ninja: Handle<Image>,
    aim: Handle<Image>,
    aura: Handle<Image>,
}

struct KeyboardControls {
    up: Vec<KeyCode>,
    down: Vec<KeyCode>,
    right: Vec<KeyCode>,
    left: Vec<KeyCode>,
}

pub struct Score( pub usize );

impl KeyboardControls {
    pub fn is_pressed(kb: &Res<Input<KeyCode>>, keys: &Vec<KeyCode>) -> bool {
        keys.iter().any(|x| kb.pressed(*x))
    }
    pub fn is_just_pressed(kb: &Res<Input<KeyCode>>, keys: &Vec<KeyCode>) -> bool {
        keys.iter().any(|x| kb.just_pressed(*x))
    }
}

//endregion

//region Main Plugin Definition
pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_system)
            .insert_resource(Score(0))
            .add_plugin(common_systems::CommonSystems)
            .add_plugin(controls::ControlsPlugin)
            .add_plugin(ui_plugin::UIPlugin)
            .add_plugin(beatmap_plugin::BeatmapPlugin)
            .add_plugin(player_plugin::PlayerPlugin)
            .add_plugin(fruit_plugin::FruitPlugin);
    }
}
//endregion


fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);

    //region Add asset handles
    let mut fruits_pieces_texture_atlas = Vec::new();
    FRUIT_ASSETS_PATH.iter().for_each(
        |path| {
            let rows_and_columns = (NUMBER_OF_FRUIT_PIECES as f32).sqrt().ceil() as usize;
            let texture_handle = asset_server.load(*path);
	        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(1024.) / rows_and_columns as f32, rows_and_columns, rows_and_columns);
	        fruits_pieces_texture_atlas.push(texture_atlases.add(texture_atlas));
        }
    );

    commands.insert_resource(
       TexturesHandles {
           fruits: FRUIT_ASSETS_PATH.iter().map( |x| asset_server.load(*x) ).collect(),
           fruits_pieces_texture_atlas, 
           ninja: asset_server.load(NINJA_PATH),
           aim: asset_server.load(AIM_PATH),
           aura: asset_server.load(AURA_PATH),
       }
   );
    //endregion
}