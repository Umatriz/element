use bevy::{
    app::{App, Update},
    core::Name,
    input::ButtonInput,
    math::Vec3,
    prelude::{
        in_state, Commands, Component, IntoSystemConfigs, KeyCode, OnEnter, Query, Res, Transform,
        With,
    },
    reflect::Reflect,
    sprite::{SpriteBundle, TextureAtlas},
};

use crate::{ImagesAssets, State};

use super::MovementSpeed;

pub fn player_plugin(app: &mut App) {
    app.add_systems(OnEnter(State::Game), spawn)
        .add_systems(Update, movement.run_if(in_state(State::Game)));
}

#[derive(Component, Reflect)]
pub struct Player;

pub fn spawn(mut commands: Commands, images_assets: Res<ImagesAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: images_assets.characters.clone(),
            ..Default::default()
        })
        .insert(TextureAtlas {
            layout: images_assets.layout.clone(),
            index: 2,
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
