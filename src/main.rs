pub mod player;


use bevy::{post_process::bloom::Bloom, window::Window, prelude::*};
//use components::Player;
use player::player::{Player, setup_player};
use player::player_movement::{advance_player_physics, handle_movement_input, clear_input, interpolate_rendered_transform};
use player::player_dash::{update_dash_timer, update_dash_cooldown, handle_dash_input, apply_dash_velocity};

// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 5.;

#[derive(Resource, Default)]
pub struct CursorWorldPos(pub Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<DidFixedTimestepRunThisFrame>()
        .init_resource::<CursorWorldPos>()
        .add_systems(Startup, (setup_scene, setup_camera, setup_player))
        .add_systems(PreUpdate, clear_fixed_timestep_flag)
        .add_systems(Update, get_cursor_pos)
        .add_systems(FixedPreUpdate, (set_fixed_time_step_flag, get_rel_cursor))
        .add_systems(FixedUpdate, (advance_player_physics, update_dash_timer, update_dash_cooldown))
        .add_systems(
            RunFixedMainLoop,
            (
                (
                    handle_movement_input,
                    handle_dash_input,
                    apply_dash_velocity,
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                (
                    clear_input.run_if(did_fixed_timestep_run_this_frame),
                    interpolate_rendered_transform,
                    update_camera,
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
            ),
        )
        .run();
}
 
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Grid background (e.g., 20x20 cells)
    let grid_width = 20;
    let grid_height = 20;
    let cell_size = 100.0;

    let line_material = materials.add(Color::srgb(0.3, 0.3, 0.3));

    for x in 0..=grid_width {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, grid_height as f32 * cell_size))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(x as f32 * cell_size - (grid_width as f32 * cell_size) / 2.0, 0.0, 0.0),
        ));
    }

    for y in 0..=grid_height {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(grid_width as f32 * cell_size, 1.0))),
            MeshMaterial2d(line_material.clone()),
            Transform::from_xyz(0.0, y as f32 * cell_size - (grid_height as f32 * cell_size) / 2.0, 0.0),
        ));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct DidFixedTimestepRunThisFrame(bool);


/// Reset the flag at the start of every frame.
fn clear_fixed_timestep_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = false;
}

fn set_fixed_time_step_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = true;
}

fn did_fixed_timestep_run_this_frame(
    did_fixed_timestep_run_this_frame: Res<DidFixedTimestepRunThisFrame>,
) -> bool {
    did_fixed_timestep_run_this_frame.0
}


// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

#[derive(Component)]
pub struct CursorRel(pub Vec2);

// Getting mouse inputs relative to world
fn get_cursor_pos(
    windows: Query<&Window>,
    camera_q: Single<&Transform, With<Camera2d>>,
    mut cursor_res: ResMut<CursorWorldPos>,
) {
    // Use .single() and handle the Result
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return, // early return if no window found
    };

    let camera_transform = camera_q.into_inner();

    if let Some(cursor_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let cursor_ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
        let world_offset = cursor_ndc * 0.5 * window_size * Vec2::new(1.0, -1.0);

        cursor_res.0 = camera_transform.translation.truncate() + world_offset;
    }
}

// Vector from Player to Cursor
fn get_rel_cursor(
    cursor_res: Res<CursorWorldPos>,
    cursor_rel: Single<&mut CursorRel>,
    player_q: Single<&Transform, With<Player>>,
){
    let player_transform = player_q.into_inner();
    let mut rel = cursor_rel.into_inner();

    // Relative vector from camera to cursor
    rel.0 = cursor_res.0 - player_transform.translation.truncate();
}
