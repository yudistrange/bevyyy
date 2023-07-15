use bevy::{
    prelude::{Camera2dBundle, Commands, Query, With},
    window::{PrimaryWindow, Window},
};

pub fn add_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: bevy::prelude::Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            10.0,
        ),
        ..Default::default()
    });
}
