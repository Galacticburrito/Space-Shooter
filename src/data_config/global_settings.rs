use super::assets_loaded::AssetsLoading;
use crate::AppState;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

// TODO: change this implimentation to use bevy common assets!
pub struct GlobalSettingsPlugin {}

impl Plugin for GlobalSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<GlobalSettings>::new(&[".ron"]))
            .add_systems(OnEnter(AppState::LoadingAssets), load_global_settings)
            .add_systems(OnExit(AppState::LoadingAssets), insert_global_settings);
    }
}

#[derive(Resource, Default)]
pub struct GlobalSettingsHandle(Handle<GlobalSettings>);

#[derive(Asset, Clone, Resource, Reflect, Deserialize, Debug)]
pub struct GlobalSettings {
    pub gravity_const: f32,
    pub velocity_max: f32,
    pub angular_velocity_max: f32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            gravity_const: 10.,
            velocity_max: 200.,
            angular_velocity_max: 20.,
        }
    }
}

fn load_global_settings(
    asset_server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut commands: Commands,
) {
    let path = "global_settings.ron";

    let handle = asset_server.load::<GlobalSettings>(path);
    assets_loading.0.push(handle.id().untyped());
    commands.insert_resource(GlobalSettingsHandle(handle));
}

pub(super) fn insert_global_settings(
    handle: Res<GlobalSettingsHandle>,
    assets: Res<Assets<GlobalSettings>>,
    mut commands: Commands,
) {
    let Some(settings) = assets.get(&handle.0) else {
        error!("global settings handle not found! Initializing default.");
        commands.init_resource::<GlobalSettings>();
        return;
    };
    commands.insert_resource(settings.clone());
    commands.remove_resource::<GlobalSettingsHandle>();
}
