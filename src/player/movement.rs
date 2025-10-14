use bevy::prelude::*;
use super::player::Player;

use super::player::PLAYER_SPEED;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default)]
pub struct AccumulatedInput {
    movement: Vec2,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Input(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec2);

pub fn handle_movement_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Input), With<Player>>,
) {
    for (mut accumulated, mut input) in &mut query {
        accumulated.movement = Vec2::ZERO;
        if kb_input.pressed(KeyCode::KeyW) { accumulated.movement.y += 1.0; }
        if kb_input.pressed(KeyCode::KeyS) { accumulated.movement.y -= 1.0; }
        if kb_input.pressed(KeyCode::KeyA) { accumulated.movement.x -= 1.0; }
        if kb_input.pressed(KeyCode::KeyD) { accumulated.movement.x += 1.0; }

        input.0 = accumulated.movement.normalize_or_zero();
    }
}

pub fn clear_input(mut input: Single<&mut AccumulatedInput>) {
    **input = AccumulatedInput::default();
}

pub fn advance_player_physics(
    fixed_time: Res<Time>,
    mut query: Query<(&mut PhysicalTranslation, &mut PreviousPhysicalTranslation, &Input), With<Player>>,
) {
    for (mut current, mut previous, input) in &mut query {
        previous.0 = current.0;
        current.0 += input.0 * PLAYER_SPEED * fixed_time.delta_secs();
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
