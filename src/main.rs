use bevy::{color::palettes, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::camera_plugin;
use entity::{player::player_plugin, MovementSpeed};

pub mod camera;
pub mod entity;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Element".to_owned(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<State>()
        .add_loading_state(
            LoadingState::new(State::AssetsLoading)
                .continue_to_state(State::Game)
                .load_collection::<ImagesAssets>(),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<MovementSpeed>()
        .insert_resource(ClearColor(Color::hsv(247.0, 0.94, 0.14)))
        .add_plugins((player_plugin, camera_plugin))
        .add_systems(Update, mark_center)
        .run();
}

fn mark_center(mut gizmos: Gizmos) {
    gizmos.circle_2d(Vec2::ZERO, 5.0, palettes::css::AQUA);
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum State {
    #[default]
    AssetsLoading,
    Game,
}

#[derive(AssetCollection, Resource)]
pub struct ImagesAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 3, rows = 1,))]
    pub layout: Handle<TextureAtlasLayout>,
    #[asset(path = "characters.png")]
    pub characters: Handle<Image>,
}
