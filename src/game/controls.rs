use crate::game::common_components::Aim;
use crate::game::{KeyboardControls, MainCamera, TexturesHandles, AIM_SCALE, is_game_state_criteria};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

//region This resource defines the player's movements, defined by the keyboard/controller/mouse
#[derive(Debug)]
pub struct Movement {
    pub x: f32,
    pub jump: bool,
    pub jumped: usize, // Times jumped
    pub is_fast_falling: bool,
    pub lock_x: bool,
}

pub struct MouseCoordinates {
    pub x: f32,
    pub y: f32,
}

pub struct Dash {
    // Whether player clicked the dash button.
    // This may not result in a dash, for instance
    // If the player is in a wall, if the player
    // Has no dashes left or if the player is already
    // Dashing
    pub trying_to_dash: bool,

    // Whether the player is dashing
    // (whether it passed the trying_to_dash checks)
    pub is_dashing: bool,

    // Times dashed since last wall.
    // If this is bigger or equal to MAX_PLAYER_DASHES_MIDAIR
    // Defines whether the player is dashing
    pub dashed: usize,
    // The direction in which the player is dashing
    pub direction: Vec2,

    // Timer for the player dash
    pub duration: Timer,
}

impl Default for Dash {
    fn default() -> Self {
        Dash {
            trying_to_dash: false,
            is_dashing: false,
            dashed: 0,
            direction: Vec2 { x: 0.0, y: 0.0 },
            duration: Timer::default(),
        }
    }
}

impl Dash {
    pub fn apply_time(&mut self, time: &Res<Time>) {
        self.duration.tick(time.delta());
    }
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            x: 0.0,
            jump: false,
            jumped: 0,
            is_fast_falling: false,
            lock_x: false,
        }
    }
}

impl Default for MouseCoordinates {
    fn default() -> Self {
        MouseCoordinates { x: 0.0, y: 0.0 }
    }
}
//endregion

//region Plugin boilerplate
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new() // These systems run before all the systems but are otherwise normal
                    // This ensures the systems are only ran when the current state is Game
                    .with_run_criteria(is_game_state_criteria)

                    // The actual systems
                    .with_system(cursor_system)
                    .with_system(keyboard_controls_system)
                    .with_system(dash_direction_arrows),
            );
    }
}
//endregion

fn keyboard_controls_system(keyboard: Res<Input<KeyCode>>, mut movement: ResMut<Movement>) {
    // You can add whatever controls you want to this list
    let controls = KeyboardControls {
        up: vec![KeyCode::W],   // In this case, jump
        down: vec![KeyCode::S], // In this case, fast fall
        right: vec![KeyCode::D],
        left: vec![KeyCode::A],
    };

    if KeyboardControls::is_just_pressed(&keyboard, &controls.up) {
        movement.jump = true;
    } // Jump will be turned to false once the value is read

    if KeyboardControls::is_pressed(&keyboard, &controls.down) {
        movement.is_fast_falling = true;
    } // You cancel fast falling by jumping or dashing

    let mut sides = 0.;
    if KeyboardControls::is_pressed(&keyboard, &controls.right) {
        sides += 1.;
    }
    if KeyboardControls::is_pressed(&keyboard, &controls.left) {
        sides -= 1.;
    }
    movement.x = sides;

    // dbg!(&movement);
}

fn joystick_control_system() {
    todo!()
}

fn cursor_system(
    mut commands: Commands,
    q_aim_entity: Query<Entity, With<Aim>>,
    mut q_aim_transform: Query<&mut Transform, With<Aim>>,
    textures: Res<TexturesHandles>,

    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,

    mut mouse_res: ResMut<MouseCoordinates>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // If there is no aim, spawn it
        if q_aim_entity.get_single().is_err() {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.aim.clone(),
                    transform: Transform {
                        scale: AIM_SCALE,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Aim);
        }

        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        mouse_res.x = world_pos.x;
        mouse_res.y = world_pos.y;
    } else {
        // If there is aim, despawn it
        if let Ok(aim_ent) = q_aim_entity.get_single() {
            commands.entity(aim_ent).despawn();
        }
    }

    for mut aim_tf in q_aim_transform.iter_mut() {
        let mut translation = &mut aim_tf.translation;
        translation.x = mouse_res.x;
        translation.y = mouse_res.y;
    }
}

fn dash_direction_arrows(kb: Res<Input<KeyCode>>, mut dash: ResMut<Dash>) {
    // You can add whatever controls you want to this list
    let controls = KeyboardControls {
        up: vec![KeyCode::Up],
        down: vec![KeyCode::Down],
        right: vec![KeyCode::Right],
        left: vec![KeyCode::Left],
    };

    // Convert whether the input has just been clicked to a number
    let to_num = |x| KeyboardControls::is_just_pressed(&kb, x) as i32 as f32;

    // Get inputs
    let direction = dash.direction
        + Vec2 {
            x: to_num(&controls.right) - to_num(&controls.left),
            y: to_num(&controls.up) - to_num(&controls.down),
        };

    if direction != Vec2::ZERO {
        dash.trying_to_dash = true;
        dash.direction = direction;
    }
}
