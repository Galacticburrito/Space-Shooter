use crate::{AppState, Player, SystemUpdateSet, record::Record, schedule::UpdateSchedule};
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

/// TODO: make smoother camera using Record
fn follow_cam(
    mut cam_query: Query<&mut Transform, With<Camera2d>>,
    mut player_query: Query<
        &mut Record<GlobalTransform>,
        (Changed<Transform>, With<Player>, Without<Camera2d>),
    >,
    time: Res<Time>,
) -> Result<(), BevyError> {
    let mut p_record = player_query.single_mut()?;

    // average last positions over 5 secs (or as much as Record can store)
    let last_positions = p_record.newest_within_secs(5., &UpdateSchedule::Update, &time);
    let avg_position: Vec3 = last_positions
        .iter()
        .map(|record| record.val.translation())
        .sum::<Vec3>()
        / last_positions.len() as f32;

    for mut cam_transform in &mut cam_query {
        cam_transform.translation.x = avg_position.x;
        cam_transform.translation.y = avg_position.y;
    }
    Ok(())
}
