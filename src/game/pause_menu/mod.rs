

fn spawn_pause_menu_system(

) {
    // Pause Menu
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.)),
                // region Align to Top-Right
                align_self: AlignSelf::Center,
                flex_grow: 0.75,
                justify_content: JustifyContent::Center,
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