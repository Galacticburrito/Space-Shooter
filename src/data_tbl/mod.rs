use bevy::prelude::*;
mod assets_loaded;
pub mod blueprint;
mod component_data;
pub mod data;
mod raw;

pub struct TablePlugin {}

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            data::DataPlugin {},
            blueprint::BlueprintPlugin {},
            assets_loaded::AssetsLoadedPlugin {},
        ));
    }
}
