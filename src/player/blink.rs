use bevy::prelude::*;
use super::player::{Player, BLINK_LENGTH};
use super::movement::{Input, PhysicalTranslation};
use super::dash::ActiveDash;


#[derive(Component)]
pub struct BlinkCooldown(pub Timer);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct BlinkVector(pub Vec2);

pub fn handle_blink_input(
    mut commands: Commands,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut BlinkCooldown,
        &Input,
        Option<&ActiveDash>
    ), With<Player>>,
) {
    for (entity, mut blink_cd, input, active_dash_opt) in &mut query {
        // Only dash if cooldown finished AND no active dash
        if kb_input.just_pressed(KeyCode::Space) 
           && blink_cd.0.is_finished()
           && active_dash_opt.is_none()
        {
            if input.0.length_squared() == 0.0 {
                continue;
            }
            let blink_vector = input.0.normalize() * BLINK_LENGTH;
            commands.entity(entity).insert(BlinkVector(
                blink_vector,
            ));

            blink_cd.0.reset();
        }
    }
}

pub fn apply_blink(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PhysicalTranslation, &BlinkVector), (With<Player>, With<BlinkVector>)>,
) {
    for (entity, mut translation, blink_vector) in &mut query {
        if blink_vector.0 != Vec2::ZERO{
            translation.0 += blink_vector.0;
        }
        commands.entity(entity).remove::<BlinkVector>();
    }
}

pub fn camera_blink_snap(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, With<BlinkVector>, Without<Camera2d>)>,
) {
    let Vec3 { x, y, .. } = player.translation;
    camera.translation = Vec3::new(x, y, camera.translation.z);
}

pub fn update_blink_cooldown(
    time: Res<Time>,
    mut query: Query<&mut BlinkCooldown>
){
    for mut cooldown in &mut query {
        cooldown.0.tick(time.delta());
    }
}