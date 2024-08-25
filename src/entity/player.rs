use bevy::{
    app::{App, Startup, Update},
    color::Color,
    core::Name,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, KeyCode, Query, Res, Transform, With},
    reflect::Reflect,
    sprite::{Sprite, SpriteBundle},
};

use super::MovementSpeed;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Startup, spawn)
        .add_systems(Update, movement);
}

#[derive(Component, Reflect)]
pub struct Player;

pub fn spawn(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::hsv(258.0, 0.02, 0.92),
                custom_size: Some(Vec2::splat(15.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((Name::new("Player"), Player, MovementSpeed::ONE));
}

pub fn movement(
    mut player_transform: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, MovementSpeed(speed))) = player_transform.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += Vec3::Y;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction += Vec3::NEG_Y;
    }

    if keys.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

    if keys.pressed(KeyCode::KeyA) {
        direction += Vec3::NEG_X;
    }

    transform.translation += direction.normalize_or_zero() * (*speed)
}
