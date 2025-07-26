use bevy::prelude::*;
mod assets_loaded;
pub mod blueprint;
mod component_data;
pub mod data;
pub mod global_settings;
mod raw;

pub struct TablePlugin {}

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            global_settings::GlobalSettingsPlugin {},
            data::DataPlugin {},
            blueprint::BlueprintPlugin {},
            assets_loaded::AssetsLoadedPlugin {},
        ));
    }
}
