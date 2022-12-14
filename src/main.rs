use bevy::{prelude::*, window::{PresentMode, WindowMode}};
use bevy_kira_audio::AudioPlugin;

mod game;
mod main_menu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStates {
        MainMenu,
        Loading,
        Game,
}

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
        .add_state(GameStates::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(game::MainPlugin)
        .run();
}

fn killall_system(mut commands: Commands, query: Query<Entity>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}