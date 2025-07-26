use bevy::prelude::*;
mod test;

pub struct UiPlugin {}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(test::TestPlugin {});
    }
}
