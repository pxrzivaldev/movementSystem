use bevy::prelude::*;
use crate::CursorRelCamPos;

const PLAYER_SPEED: f32 = 220.;
pub const DASH_CD: f32 = 2.;
const DASH_LENGTH: f32 = 500.;
const DASH_DURATION: f32 = 0.2;

#[derive(Component)]
pub struct Player;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default)]
pub struct AccumulatedInput {
    movement: Vec2,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec2);

#[derive(Component)]
pub struct DashCooldown(pub Timer);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ActiveCooldown;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ActiveDash(Timer, Vec2);

pub fn accumulate_input(
    mut commands: Commands,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(
        Entity,
        &mut AccumulatedInput,
        &mut Velocity,
        &mut DashCooldown,
        &CursorRelCamPos,
        Option<&ActiveCooldown>,
        Option<&ActiveDash>
    ), With<Player>>,
) {
    for (entity, mut input, mut velocity, mut dash_cd, cursor_rel, on_cooldown, d_timer) in &mut player {
        input.movement = Vec2::ZERO;
        if kb_input.pressed(KeyCode::KeyW) { input.movement.y += 1.0; }
        if kb_input.pressed(KeyCode::KeyS) { input.movement.y -= 1.0; }
        if kb_input.pressed(KeyCode::KeyA) { input.movement.x -= 1.0; }
        if kb_input.pressed(KeyCode::KeyD) { input.movement.x += 1.0; }
        if d_timer.is_none() {
            if kb_input.pressed(KeyCode::Space) {
                if on_cooldown.is_none() {
                    
                    let mut dash_vector = cursor_rel.0; // Cursor relative to player
                    let distance = dash_vector.length();
                    if distance > DASH_LENGTH {
                        dash_vector = dash_vector.normalize() * DASH_LENGTH;
                    }
                    commands.entity(entity).insert(ActiveCooldown);
                    commands.entity(entity).insert(ActiveDash(Timer::from_seconds(DASH_DURATION, TimerMode::Once),dash_vector));
                    dash_cd.0.reset();
                }
            }
            velocity.0 = input.movement.normalize_or_zero() * PLAYER_SPEED;
            return;
        } else {
            if let Some(active_dash) = d_timer {
                velocity.0 = active_dash.1 / DASH_DURATION; // constant velocity
            }
        }
    }
}

pub fn update_dash_timer(
    time: Res<Time>,
    mut query: Query<(Entity, &mut ActiveDash), With<Player>>,
    mut commands: Commands,
) {
    for (entity, mut dash) in &mut query {
        dash.0.tick(time.delta());
        if dash.0.is_finished() {
            commands.entity(entity).remove::<ActiveDash>();
            commands.entity(entity).remove::<ActiveCooldown>();
        }
    }
}

pub fn update_dash_cooldown(
    time: Res<Time>,
    mut query: Query<&mut DashCooldown>
){
    for mut cooldown in &mut query {
        cooldown.0.tick(time.delta());
    }
}

pub fn advance_player_physics(
    fixed_time: Res<Time>,
    mut query: Query<(&mut PhysicalTranslation, &mut PreviousPhysicalTranslation, &Velocity), With<Player>>,
) {
    for (mut current, mut previous, velocity) in &mut query {
        previous.0 = current.0;
        current.0 += velocity.0 * fixed_time.delta_secs();
    }
}

pub fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &PhysicalTranslation, &PreviousPhysicalTranslation), With<Player>>,
) {
    for (mut transform, current, previous) in &mut query {
        let alpha = fixed_time.overstep_fraction();
        let rendered = previous.0.lerp(current.0, alpha);

        // Map 2D (x,y) into 3D Transform at z=0
        transform.translation = Vec3::new(rendered.x, rendered.y, 0.0);
    }
}
