use bevy::{
    prelude::{Component, Input, KeyCode, Query, Res, Transform, Vec3, With},
    time::Time,
    window::{PrimaryWindow, Window},
};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

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

pub fn constraint_movement_to_window_size(
    mut player_query: Query<&mut Transform, With<PlayerControlled>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut translation = player_transform.translation;
        let player_sprite_size = PLAYER_SIZE / 2.0;

        let x_min = 0.0 + player_sprite_size;
        let x_max = window.width() - player_sprite_size;
        let y_min = 0.0 + player_sprite_size;
        let y_max = window.height() - player_sprite_size;

        if translation.x > x_max {
            translation.x = x_max
        } else if translation.x < x_min {
            translation.x = x_min
        }

        if translation.y > y_max {
            translation.y = y_max
        } else if translation.y < y_min {
            translation.y = y_min
        }

        println!(
            "x_min: {}\nx_max: {}\ny_min: {}\ny_max: {}",
            x_min, x_max, y_min, y_max
        );

        player_transform.translation = translation
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
