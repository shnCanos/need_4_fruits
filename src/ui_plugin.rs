use crate::Score;
use bevy::prelude::*;

//region Plugin boilerplate
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ui_setup_system)
            .add_system(ui_update_system);
    }
}
//endregion

//region Player Only Components
#[derive(Component)]
pub struct ScoreText;
//endregion

fn ui_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/Rubik-Regular.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::CENTER)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(ScoreText);
}

fn ui_update_system(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    query.for_each_mut(|mut score_text| {
        score_text.sections[0].value = format!("Combo: {}", score.0.to_string())
    });
}
