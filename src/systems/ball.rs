use bevy::{
    prelude::{AssetServer, Commands, Query, Res, Transform, With},
    sprite::SpriteBundle,
    window::{PrimaryWindow, Window},
};

pub fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        texture: asset_server.load("sprites/ball.png"),
        ..Default::default()
    });
}
