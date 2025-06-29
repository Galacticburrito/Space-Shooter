use ai_ship_tree::AiShipTreePlugin;
use bevy::prelude::*;
mod ai_ship_tree;
mod setup;

pub struct AiPlugin {}

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((setup::SetupPlugin {}, AiShipTreePlugin {}));
    }
}

#[derive(Component)]
pub struct Ai {}
