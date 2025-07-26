use crate::{SystemUpdateSet, velocity::global::GlobalVelocity};

use super::{SnapshotData, Sonar, SonarDetectable, SonarPulse, sonar_pulse::SonarPulseData};
use bevy::prelude::*;

pub struct DetectionEventPlugin {}
impl Plugin for DetectionEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DetectionEvent>().add_systems(
            Update,
            (on_detected, on_first_detected, on_last_detected).in_set(SystemUpdateSet::Main),
        );
    }
}

#[derive(Event)]
pub enum DetectionEvent {
    /// entity first detected
    FirstDetected(SonarPulse, Entity),
    /// every time entity is detected
    Detected(SonarPulse, Entity),
    /// last time entity detected
    LastDetected(Entity),
}

/// each frame entity is detected, add to Sonar so up to date
fn on_detected(
    mut events: EventReader<DetectionEvent>,
    mut sonar_query: Query<&mut Sonar>,
    detectable_query: Query<(&GlobalTransform, &GlobalVelocity), With<SonarDetectable>>,
) {
    for event in events.read() {
        if let DetectionEvent::Detected(pulse, entity) = event {
            info!("detected!");
            let Ok(mut sonar) = sonar_query.get_mut(pulse.originator) else {
                continue;
            };

            let Ok((d_g_transform, d_g_velocity)) = detectable_query.get(*entity) else {
                continue;
            };

            let snapshot = SnapshotData {
                g_transform: d_g_transform.compute_transform().into(),
                g_velocity: d_g_velocity.clone(),
            };

            sonar.detected_entities.insert(*entity, snapshot);
        }
    }
}

fn on_first_detected(
    mut events: EventReader<DetectionEvent>,
    mut detectable_query: Query<&mut Visibility, With<SonarDetectable>>,
) {
    for event in events.read() {
        if let DetectionEvent::FirstDetected(_, entity) = event {
            info!("first detected!");
            let Ok(mut d_visibility) = detectable_query.get_mut(*entity) else {
                continue;
            };
            *d_visibility = Visibility::Visible;
        }
    }
}
fn on_last_detected(
    mut events: EventReader<DetectionEvent>,
    mut detectable_query: Query<&mut Visibility, With<SonarDetectable>>,
) {
    for event in events.read() {
        if let DetectionEvent::LastDetected(entity) = event {
            info!("last detected!");
            let Ok(mut d_visibility) = detectable_query.get_mut(*entity) else {
                continue;
            };
            *d_visibility = Visibility::Hidden;
        }
    }
}
