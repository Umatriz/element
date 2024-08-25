use bevy::{prelude::Component, reflect::Reflect};

pub mod player;

#[derive(Component, Reflect)]
pub struct MovementSpeed(pub f32);

impl MovementSpeed {
    pub const ONE: Self = Self(1.0);
}
