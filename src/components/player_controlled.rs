use bevy::{
    prelude::{Component, Input, KeyCode, Query, Res, Transform, Vec3, With},
    time::Time,
};

pub const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct PlayerControlled;

pub fn add_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerControlled>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let direction: Vec3 = handle_keyboard_inputs(keyboard_input);

        player_transform.translation += direction * time.delta_seconds() * PLAYER_SPEED
    }
}

fn handle_keyboard_inputs(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0)
    }

    normalize_direction(direction)
}

fn normalize_direction(direction: Vec3) -> Vec3 {
    if direction.length() > 1.0 {
        direction.normalize()
    } else {
        direction
    }
}
