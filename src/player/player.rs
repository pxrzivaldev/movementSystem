use crate::CursorRel;

use super::movement::{AccumulatedInput, Input, PhysicalTranslation, PreviousPhysicalTranslation};
use super::dash::DashCooldown;
use super::blink::BlinkCooldown;
use bevy::prelude::*;

pub(super) const PLAYER_SPEED: f32 = 370.;
pub(super) const DASH_CD: f32 = 2.;
pub(super) const DASH_LENGTH: f32 = 800.;
pub(super) const DASH_DURATION: f32 = 0.4;
pub(super) const BLINK_CD: f32 = 3.;
pub(super) const BLINK_LENGTH: f32 = 500.;

#[derive(Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn((
        Player,
        AccumulatedInput::default(),
        Input(Vec2::ZERO),
        PhysicalTranslation(Vec2::ZERO),
        PreviousPhysicalTranslation(Vec2::ZERO),
        CursorRel(Vec2::ZERO),
        DashCooldown(Timer::from_seconds(DASH_CD, TimerMode::Once)),
        BlinkCooldown(Timer::from_seconds(BLINK_CD, TimerMode::Once)),
        Mesh2d(meshes.add(Circle::new(15.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(0., 0., 2.),
    ));
}