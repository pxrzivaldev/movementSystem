use bevy::prelude::*;
use crate::{CursorRelCamPos, get_rel_cursorpos};

const PLAYER_SPEED: f32 = 220.;
const DASH_CD: f32 = 2.;
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
struct DashCooldown(Timer);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct ActiveCooldown;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct ActiveDashDodge;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct DashTimer(Timer);

pub fn accumulate_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut AccumulatedInput, &mut Velocity), With<Player>>,
) {
    for (mut input, mut velocity) in &mut player {
        input.movement = Vec2::ZERO;

        if kb_input.pressed(KeyCode::KeyW) { input.movement.y += 1.0; }
        if kb_input.pressed(KeyCode::KeyS) { input.movement.y -= 1.0; }
        if kb_input.pressed(KeyCode::KeyA) { input.movement.x -= 1.0; }
        if kb_input.pressed(KeyCode::KeyD) { input.movement.x += 1.0; }

        velocity.0 = input.movement.normalize_or_zero() * PLAYER_SPEED;
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
