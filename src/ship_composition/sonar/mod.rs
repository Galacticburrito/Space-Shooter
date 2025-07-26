use crate::{
    SystemUpdateSet, serialization::transform::SerializeableTransform,
    velocity::global::GlobalVelocity,
};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
pub mod sonar_pulse;
use sonar_pulse::{SonarPulse, SonarPulseData};
pub mod detection_event;

pub struct SonarPlugin {}

impl Plugin for SonarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            sonar_pulse::SonarPulsePlugin {},
            detection_event::DetectionEventPlugin {},
        ))
        /*.add_systems(
            Update,
            sonar_detectable_to_children.in_set(SystemUpdateSet::Main),
        )*/
        .add_observer(sonar_detectable_constructor)
        .register_type::<Sonar>()
        .register_type::<SonarDetectable>();
    }
}

#[derive(Reflect, Component, Deserialize, Debug, Clone)]
pub struct Sonar {
    pulse_data: SonarPulseData,
    detected_entities: HashMap<Entity, SnapshotData>,
}

impl Sonar {
    pub fn new(pulse_data: SonarPulseData) -> Self {
        Sonar {
            pulse_data,
            detected_entities: HashMap::new(),
        }
    }

    /// goes outwards untill hits an object, then highlights object in a color
    /// color indicates alliegance (green = ally, red = enemy, white = default)
    /// TODO: want ship parts to highlight color based on health?
    pub fn pulse(&self, entity: Entity, commands: &mut Commands) {
        commands.spawn(SonarPulse::new(entity, self.pulse_data.clone()));
    }
}

/// a snapshot of what the entity had at moment of detection
#[derive(Reflect, Debug, Clone, Deserialize)]
pub struct SnapshotData {
    g_transform: SerializeableTransform,
    g_velocity: GlobalVelocity,
}

/// detectable by sonar
#[derive(Component, Reflect, Default)]
pub struct SonarDetectable {
    detected: bool,
}

impl SonarDetectable {
    /// changes visiblity to false and adds component to all children
    pub fn new() -> Self {
        SonarDetectable { detected: false }
    }
}

/// change visibility, as well as adding component to all children
fn sonar_detectable_constructor(
    trigger: Trigger<OnAdd, SonarDetectable>,
    mut visibility_query: Query<&mut Visibility>,
    children_query: Query<Entity, With<Children>>,
    mut commands: Commands,
) {
    if let Ok(mut visibility) = visibility_query.get_mut(trigger.target()) {
        info!("changing visibility to hidden!");
        *visibility = Visibility::Hidden;
    }

    if let Ok(child) = children_query.get(trigger.target()) {
        info!("adding sonar detectable!");
        commands.entity(child).insert(SonarDetectable::new());
    }
}

fn sonar_detectable_to_children(
    query: Query<&Children, Added<SonarDetectable>>,
    mut commands: Commands,
) {
    for children in query {
        for child_entity in children {
            commands
                .entity(*child_entity)
                .insert(SonarDetectable::new());
        }
    }
}
