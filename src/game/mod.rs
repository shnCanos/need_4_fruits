use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use crate::game::common_components::MainCamera;
use crate::game::common_systems::RestartEvent;
use crate::game::controls::{Dash, MouseCoordinates, Movement};
use crate::{GameStates, killall_system};

//region Import Modules
mod beatmap_plugin;
mod common_components;
mod common_systems;
mod controls;
mod fruit_plugin;
mod osu_reader;
mod player_plugin;
mod ui_plugin;
//endregion

//region Consts

//region Assets
const FRUIT_ASSETS_PATH: [&str; 2] = ["watermelon.png", "watermelon2.png"];
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
const FRUIT_SPEED: f32 = 20.;
const FRUITS_GRAVITY_UP: f32 = 0.6;
const FRUITS_GRAVITY_HOLD: f32 = 0.168;
const FRUITS_GRAVITY_FALL: f32 = 1.2;
// Beatmap
const BEATMAP_INITIAL_WAIT_TIME: f32 = 0.5;
const BEATMAP_MUSIC_OFFSET_TIME: f32 = 0.7;
const BEATMAP_FILE_NAME: &str = "beatMARIO_-_Night_of_Knights_alacat_Hard.osu";
/// How much of the screen's horizontal width is spawnable for fruits (0.0-1.0)
const EFFECTIVE_SCREEN_WIDTH_PERCENT: f32 = 0.9;
// Fruit Part
const NUMBER_OF_FRUIT_PIECES: i32 = 4; // Has to be a perfect square
const MAX_FRUIT_PIECE_SPEED: f32 = 8.;

// Player variables
// Air
const PLAYER_SPEED: f32 = 10.;
const PLAYER_GRAVITY: f32 = 0.6;
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
const DASH_SPEED: f32 = 60.;
//endregion

//endregion

//region Global structs definitions
#[derive(Default)]
struct TexturesHandles {
    fruits: Vec<Handle<Image>>,
    fruits_pieces_texture_atlas: Vec<Handle<TextureAtlas>>,
    ninja: Handle<Image>,
    aim: Handle<Image>,
    aura: Handle<Image>,
}

struct FontHandles {
    rubik_regular: Handle<Font>,
}

struct KeyboardControls {
    up: Vec<KeyCode>,
    down: Vec<KeyCode>,
    right: Vec<KeyCode>,
    left: Vec<KeyCode>,
}

pub struct GameSettings {
    pub dash_stop : bool,
    pub snap_on_cut : bool,
    pub no_death_penalty : bool
}

impl Default for GameSettings {
    fn default() -> Self {
        Self { dash_stop: false, snap_on_cut: false, no_death_penalty: false }
    }
}

pub struct Score(pub usize);

impl KeyboardControls {
    pub fn is_pressed(kb: &Res<Input<KeyCode>>, keys: &Vec<KeyCode>) -> bool {
        keys.iter().any(|x| kb.pressed(*x))
    }
    pub fn is_just_pressed(kb: &Res<Input<KeyCode>>, keys: &Vec<KeyCode>) -> bool {
        keys.iter().any(|x| kb.just_pressed(*x))
    }
}

// There are, as of now, two different functions that run in parallel when the loading GameState is active
// They have to wait for each other, and this resource is used to make sure everything is set when the game
// Starts
pub struct SectionsLoaded( pub usize ); 
//endregion

//region Main Plugin Definition
pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameStates::Loading) // Startup systems
                    .with_system(setup_system)
                    .with_system(killall_system)
            )
            .add_system_set(
                SystemSet::on_update(GameStates::Loading)
                .with_system(update_loading_screen)
            )
            .insert_resource(SectionsLoaded ( 0 ))
            .add_plugin(common_systems::CommonSystems)
            .add_plugin(controls::ControlsPlugin)
            .add_plugin(ui_plugin::UIPlugin)
            .add_plugin(beatmap_plugin::BeatmapPlugin)
            .add_plugin(player_plugin::PlayerPlugin)
            .add_plugin(fruit_plugin::FruitPlugin)

            .add_system_set(
                SystemSet::on_exit(GameStates::Game) // Startup systems
                    .with_system(leave_game_system)
                    .with_system(killall_system)
            );
    }
}
//endregion

pub fn is_game_state_criteria(
    game_state: Res<State<GameStates>>,
) -> ShouldRun {
    let game_state = game_state.current();
    if matches!(game_state, GameStates::Game) {
        return ShouldRun::Yes;
    }
    ShouldRun::No
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut restart_events: EventWriter<RestartEvent>,
    mut sections_loaded: ResMut<SectionsLoaded>,
) {
    // Spawn camera
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 10.),
            ..Camera2dBundle::new_with_far(100.0)
        })
        .insert(MainCamera);

    //region Add asset handles
    let mut fruits_pieces_texture_atlas = Vec::new();
    
    FRUIT_ASSETS_PATH.iter().for_each(|path| {
        let rows_and_columns = (NUMBER_OF_FRUIT_PIECES as f32).sqrt().ceil() as usize;
        let texture_handle = asset_server.load(*path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::splat(1024.) / rows_and_columns as f32,
            rows_and_columns,
            rows_and_columns,
        );
        fruits_pieces_texture_atlas.push(texture_atlases.add(texture_atlas));
    });

    commands.insert_resource(TexturesHandles {
        fruits: FRUIT_ASSETS_PATH
            .iter()
            .map(|x| asset_server.load(*x))
            .collect(),
        fruits_pieces_texture_atlas,
        ninja: asset_server.load(NINJA_PATH),
        aim: asset_server.load(AIM_PATH),
        aura: asset_server.load(AURA_PATH),
    });

    commands.insert_resource(
        FontHandles {
            rubik_regular: asset_server.load("fonts/Rubik-Regular.ttf"),
        }
    );
    //endregion

    // mod.rs resources
    commands.insert_resource(Score(0));
    commands.insert_resource(GameSettings::default());

    // ControlsPlugin resources
    commands.insert_resource(Movement::default());
    commands.insert_resource(MouseCoordinates::default());
    commands.insert_resource(Dash::default());

    // Restart events
    restart_events.send_default();

    // BeatmapPlugin has its own init system

    // Tell the loading screen that this section is loaded
    sections_loaded.0 += 1;
}

fn update_loading_screen(mut game_state: ResMut<State<GameStates>>, sections_loaded: Res<SectionsLoaded>) {
    if sections_loaded.0 == 2 {
        game_state.overwrite_set(GameStates::Game).unwrap();
    }
}

fn leave_game_system(
    mut commands: Commands
) {
    // Delete mod.rs resources
    commands.remove_resource::<Score>();

    // Delete ControlsPlugin resources
    commands.remove_resource::<Movement>();
    commands.remove_resource::<MouseCoordinates>();
    commands.remove_resource::<Dash>();
}
