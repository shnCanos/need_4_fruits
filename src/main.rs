use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use crate::common_components::MainCamera;

//region Import Modules
mod common_components;
mod fruit_plugin;
mod common_systems;
mod controls;
mod player_plugin;
mod ui_plugin;
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
const FRUIT_SPEED: f32 = 8.;
const FRUITS_GRAVITY: f32 = 0.03; // TODO: remove that
// Spawn
const DEFAULT_FRUIT_SPAWN_TIME: f32 = 1.;
const FRUIT_HORIZONTAL_MARGIN: f32 = 120.0;
// Fruit Part
const NUMBER_OF_FRUIT_PIECES: f32 = 4.; // Has to be a perfect square
const MAX_FRUIT_PIECE_SPEED: f32 = 10.;

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
const DASH_DURATION: f32 = 0.1; // The duration of a dash in seconds
const MAX_PLAYER_DASHES_MIDAIR: usize = 1;
const DASH_SPEED: f32 = 40.;
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
struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_system)
            .insert_resource(Score( 0 ))
            .add_plugin(fruit_plugin::FruitPlugin)
            .add_plugin(common_systems::CommonSystems)
            .add_plugin(controls::ControlsPlugin)
            .add_plugin(player_plugin::PlayerPlugin)
            .add_plugin(ui_plugin::UIPlugin);
    }
}
//endregion
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3, 0.2, 0.4)))

        .insert_resource(WindowDescriptor {
        width: 1280.,
        height: 720.,
        position: WindowPosition::At(Vec2 { x: 120., y: 40. }),
        resize_constraints: Default::default(),
        scale_factor_override: None,
        title: "Need 4 Fruits".to_string(),
        present_mode: PresentMode::Fifo,
        resizable: true,
        decorations: true,
        cursor_visible: false,
        cursor_locked: false,
        mode: WindowMode::Windowed,
        transparent: false,
        canvas: None,
        fit_canvas_to_parent: false
    })
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // window_res: Res<Windows>,
) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);

    //region Add asset handles

    let mut fruits_pieces_texture_atlas = Vec::new();
    FRUIT_ASSETS_PATH.iter().for_each(
        |path| {
            let rows_and_collumns = NUMBER_OF_FRUIT_PIECES.sqrt() as usize;
            let texture_handle = asset_server.load(*path);
	        let texture_atlas = TextureAtlas::from_grid(texture_handle, FRUITS_SIZE / NUMBER_OF_FRUIT_PIECES, rows_and_collumns, rows_and_collumns);
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