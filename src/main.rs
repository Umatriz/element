use bevy::{color::palettes, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::camera_plugin;
use entity::{player::player_plugin, MovementSpeed};

pub mod camera;
pub mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Element".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::hsv(247.0, 0.94, 0.14)))
        .register_type::<MovementSpeed>()
        .add_plugins((player_plugin, camera_plugin))
        .add_systems(Update, mark_center)
        .run();
}

fn mark_center(mut gizmos: Gizmos) {
    gizmos.circle_2d(Vec2::ZERO, 5.0, palettes::css::AQUA);
}
