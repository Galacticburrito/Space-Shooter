use crate::{AppState, Player, SystemUpdateSet};
use bevy::prelude::*;

pub struct CameraPlugin {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup);
        app.add_systems(Update, follow_cam.in_set(SystemUpdateSet::Camera));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn follow_cam(
    mut cam_query: Query<&mut Transform, With<Camera2d>>,
    player_query: Query<Ref<Transform>, (With<Player>, Without<Camera2d>)>,
) -> Result<(), BevyError> {
    if !player_query.single()?.is_changed() {
        return Ok(());
    }

    let player_pos = player_query.single()?.translation;
    for mut cam_transform in &mut cam_query {
        cam_transform.translation.x = player_pos.x;
        cam_transform.translation.y = player_pos.y;
    }
    Ok(())
}
