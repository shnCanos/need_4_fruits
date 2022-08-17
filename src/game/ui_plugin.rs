use crate::game::Score;
use bevy::prelude::*;

use super::GameSettings;

//region Plugin boilerplate
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, ui_setup_system)
            .add_system(ui_post_setup_system)
            .add_system(ui_update_system)
            .add_system(button_system)
            .add_system(button_press_system);
    }
}
//endregion

//region UI Only Components
#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ToggleButton(String, bool);

#[derive(Component)]
pub enum SettingsButton {
    DashStop,
    SnapOnCut,
}
//endregion

fn ui_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Rubik-Regular.ttf");

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: font.clone(),
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

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.)),
                // region Align to Top-Right
                align_self: AlignSelf::FlexEnd,
                flex_grow: 1.0,
                justify_content: JustifyContent::FlexEnd,
                // endregion
                ..Default::default()
            },
            color: UiColor(Color::rgba(0., 0., 0., 0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(menu_background(Color::BLACK))
                .with_children(|parent| {
                    // Title
                    parent
                        // NodeBundle for centering the Text
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(20.)),
                                // region Align to Top-Right
                                align_self: AlignSelf::Center,
                                flex_grow: 1.0,
                                justify_content: JustifyContent::Center,
                                // endregion
                                ..Default::default()
                            },
                            color: UiColor(Color::rgba(0., 0., 0., 0.0)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(button_text(
                                Color::WHITE,
                                &font,
                                "Gameplay Options",
                            ));
                        });

                    // Dash Stop Button
                    parent
                        .spawn_bundle(button(Color::DARK_GRAY))
                        .with_children(|parent| {
                            parent.spawn_bundle(button_text(Color::WHITE, &font, ""));
                        })
                        .insert(ToggleButton("Dash Stop".to_string(), false))
                        .insert(SettingsButton::DashStop);

                    // Snap on Cut Button
                    parent
                        .spawn_bundle(button(Color::DARK_GRAY))
                        .with_children(|parent| {
                            parent.spawn_bundle(button_text(Color::WHITE, &font, ""));
                        })
                        .insert(ToggleButton("Snap on Cut".to_string(), true))
                        .insert(SettingsButton::SnapOnCut);
                });
        });
}

fn button_text(color: Color, font: &Handle<Font>, text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font.clone(),
            font_size: 18.,
            color,
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(3.)),
        ..Default::default()
    })
}

fn button(color: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            padding: UiRect::all(Val::Px(3.)),
            ..default_style()
        },
        color: UiColor(color),
        ..Default::default()
    }
}

fn menu_background(color: Color) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), Val::Auto),
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(3.0)),
            ..default_style()
        },
        color: UiColor(color),
        ..Default::default()
    }
}

fn button_system(
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *color = UiColor(Color::BLACK),
            Interaction::Hovered => *color = UiColor(Color::GRAY),
            Interaction::None => *color = UiColor(Color::DARK_GRAY),
        }
    }
}

fn ui_post_setup_system(
    mut buttons: Query<(&Children, &ToggleButton), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (children, toggle) in buttons.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        text.sections[0].value =
            toggle.0.clone() + &(if toggle.1 { ": On" } else { ": Off" }).to_string();
    }
}

fn button_press_system(
    mut buttons: Query<
        (&Interaction, &SettingsButton, &mut ToggleButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, settings, mut toggle) in buttons.iter_mut() {
        if *interaction == Interaction::Clicked {
            toggle.1 = !toggle.1;

            match settings {
                SettingsButton::DashStop => game_settings.dash_stop = toggle.1,
                SettingsButton::SnapOnCut => game_settings.snap_on_cut = toggle.1,
            };
        }
    }
}

fn default_style() -> Style {
    Style {
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    }
}

fn ui_update_system(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    query.for_each_mut(|mut score_text| {
        score_text.sections[0].value = format!("Combo: {}", score.0.to_string())
    });
}
