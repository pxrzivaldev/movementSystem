use bevy::prelude::*;
use super::player::Player;
use super::player_movement::Velocity;

use super::player::DASH_DURATION;
use super::player::DASH_LENGTH;

#[derive(Component)]
pub struct DashCooldown(pub Timer);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct HasDashCooldown;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ActiveDash(Timer, Vec2);

pub fn handle_dash_input(
    mut commands: Commands,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut DashCooldown,
        &Velocity,
        Option<&ActiveDash>
    ), With<Player>>,
) {
    for (entity, mut dash_cd, velocity, active_dash_opt) in &mut query {
        // Only dash if cooldown finished AND no active dash
        if kb_input.just_pressed(KeyCode::Space) 
           && dash_cd.0.is_finished()
           && active_dash_opt.is_none()
        {
            let mut dash_vector = velocity.0;
            if dash_vector.length_squared() == 0.0 {
                continue; // skip dash entirely on ZERO velocity
            }
            dash_vector = dash_vector.normalize() * DASH_LENGTH;

            commands.entity(entity).insert(ActiveDash(
                Timer::from_seconds(DASH_DURATION, TimerMode::Once),
                dash_vector,
            ));

            dash_cd.0.reset();
        }
    }
}

pub fn apply_dash_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut ActiveDash), (With<Player>, With<ActiveDash>)>,
) {
    for (mut velocity, mut active_dash) in &mut query {
        // Update timer
        active_dash.0.tick(time.delta());

        // Apply constant dash velocity
        velocity.0 = active_dash.1 / DASH_DURATION;
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
            commands.entity(entity).remove::<HasDashCooldown>();
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