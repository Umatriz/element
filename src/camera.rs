use bevy::{
    app::{App, Startup, Update},
    prelude::{Camera2dBundle, Commands, Component, Query, Transform, With, Without},
};

use crate::entity::player::Player;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn)
        .add_systems(Update, follow_player);
}

#[derive(Component)]
pub struct Camera;

pub fn spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}

pub fn follow_player(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let Ok(player_transform) = player_transform.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_transform.get_single_mut() else {
        return;
    };

    camera_transform.translation = camera_transform
        .translation
        .lerp(player_transform.translation, 0.2);
}
