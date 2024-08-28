use bevy::{
    app::{App, Startup, Update},
    input::ButtonInput,
    prelude::{
        Camera2dBundle, Commands, Component, KeyCode, OrthographicProjection, Query, Res,
        Transform, With, Without,
    },
};

use crate::entity::player::Player;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn)
        .add_systems(Update, (follow_player, camera_scaling));
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

pub fn camera_scaling(
    mut camera: Query<&mut OrthographicProjection, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let mut scale = camera.scale;

    if keys.pressed(KeyCode::KeyZ) {
        scale += 0.02;
    }

    if keys.pressed(KeyCode::KeyX) {
        scale -= 0.02;
    }

    if !(0.2..=1.0).contains(&scale) {
        return;
    }

    camera.scale = scale;
}
