use crate::AppState;
use bevy::{asset::UntypedAssetId, prelude::*};

pub struct AssetsLoadedPlugin {}

impl Plugin for AssetsLoadedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_all_assets_loaded.run_if(in_state(AppState::LoadingAssets)),
        )
        .init_resource::<AssetsLoading>();
    }
}

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<UntypedAssetId>);

/// check if all assets in `AssetsLoading` are loaded.
/// transitions to `GameReady` if all loaded.
fn check_all_assets_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    assets_loading: Res<AssetsLoading>,
    asset_server: Res<AssetServer>,
) {
    if assets_loading.0.is_empty() {
        info!("No assets to load. Transitioning to GameReady.");
        next_state.set(AppState::GameReady);
        return;
    }

    let all_loaded = assets_loading
        .0
        .iter()
        .all(|id| asset_server.is_loaded_with_dependencies(*id));

    if all_loaded {
        info!("All assets loaded. Transitioning to GameReady.");
        next_state.set(AppState::GameReady);
    }
}
