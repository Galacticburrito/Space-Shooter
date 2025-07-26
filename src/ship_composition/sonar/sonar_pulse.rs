use super::{Sonar, SonarDetectable, detection_event::DetectionEvent};
use crate::{
    SystemUpdateSet,
    collision::{self, CollisionEvent},
    collision::{
        collider::{Collider, CollisionLayer},
        collider_type::ColliderType,
    },
    color_palette::PalColor,
    graphic::Graphic,
};
use bevy::{prelude::*, time::Stopwatch};
use serde::Deserialize;

pub struct SonarPulsePlugin {}

impl Plugin for SonarPulsePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spread_sonar_pulse,
                /*sonar_pulse_hit,*/ sonar_pulse_collide,
            )
                .in_set(SystemUpdateSet::Main),
        )
        .add_observer(sonar_pulse_constructor);
    }
}

/// pulse of sonar (circle that extends outward from sonar)
/// constructor handles rest
#[derive(Component, Clone)]
pub struct SonarPulse {
    pub originator: Entity,
    pub elapsed_time: Stopwatch,
    pub data: SonarPulseData,
}

impl SonarPulse {
    pub fn new(originator: Entity, data: SonarPulseData) -> Self {
        SonarPulse {
            originator,
            elapsed_time: Stopwatch::new(),
            data,
        }
    }

    /// radius from center of pulse to inner line
    fn inner_radius(&self) -> f32 {
        self.elapsed_time.elapsed_secs() * self.data.speed
    }

    /// radius from center of pulse to outer line
    fn outer_radius(&self) -> f32 {
        self.inner_radius() + self.data.thickness
    }
}

/// unchanging pulse data, sent by the Sonar
#[derive(Reflect, Clone, Deserialize, Debug)]
pub struct SonarPulseData {
    pub thickness: f32,
    pub speed: f32,
    pub range: f32,
}

/// adds necessary visual elements upon adding SonarPulse
fn sonar_pulse_constructor(
    trigger: Trigger<OnInsert, SonarPulse>,
    pulse_query: Query<&SonarPulse>,
    transform_query: Query<&GlobalTransform, With<Sonar>>,
    mut commands: Commands,
) {
    let Ok(pulse) = pulse_query.get(trigger.target()) else {
        return;
    };

    let Ok(g_transform) = transform_query.get(pulse.originator) else {
        warn!("transform for sonar not found! Cannot crate a sonar pulse.");
        return;
    };

    let ring = Annulus::new(0., pulse.data.thickness);
    let graphic = Graphic::new(ring.into(), PalColor::White.into());
    let collider = Collider::new(ColliderType::from(ring), CollisionLayer::SonarPulse);

    commands.entity(trigger.target()).insert((
        graphic,
        Transform::from_translation(g_transform.translation()),
        collider,
    ));
}

/// make pulse spread outwards
fn spread_sonar_pulse(
    query: Query<(Entity, &mut SonarPulse, &mut Graphic)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut pulse, mut graphic) in query {
        pulse.elapsed_time.tick(time.delta());

        let outer_edge = pulse.outer_radius();

        if outer_edge > pulse.data.range {
            commands.entity(entity).despawn();
        }

        let expanded_ring = Annulus::new(
            outer_edge - pulse.data.thickness,
            outer_edge + pulse.data.thickness,
        );

        graphic.replace_shape(expanded_ring.into());
        commands.entity(entity).try_insert(Collider::new(
            ColliderType::from(expanded_ring),
            CollisionLayer::SonarPulse,
        ));
    }
}

fn sonar_pulse_collide(
    mut collision_events: EventReader<CollisionEvent>,
    detectable_query: Query<(Entity, &mut SonarDetectable), With<Collider>>,
    pulse_query: Query<&SonarPulse, With<Collider>>,
    mut event_writer: EventWriter<DetectionEvent>,
) {
    for (d_entity, mut d_detectable) in detectable_query {
        let pulse_opt = collision::collided_with_component::<SonarPulse, With<Collider>>(
            d_entity,
            &mut collision_events,
            &pulse_query,
        );

        match (d_detectable.detected, pulse_opt) {
            // (previously detected, detected now)
            (true, Some(pulse)) => {
                info!("triggered detected!");
                event_writer.write(DetectionEvent::Detected(pulse.clone(), d_entity));
            }
            (false, Some(pulse)) => {
                info!("triggered first detected!");
                event_writer.write(DetectionEvent::FirstDetected(pulse.clone(), d_entity));
                event_writer.write(DetectionEvent::Detected(pulse.clone(), d_entity));
            }
            (true, None) => {
                info!("triggered last detected!");
                event_writer.write(DetectionEvent::LastDetected(d_entity));
            }
            (false, None) => {}
        };
        d_detectable.detected = pulse_opt.is_some();
    }
}

/// launch DetectionEvent if entity is detected (in pulse)
fn sonar_pulse_hit(
    query: Query<(&SonarPulse, &GlobalTransform)>,
    mut detectable_query: Query<(Entity, &mut SonarDetectable, &GlobalTransform)>,
    mut commands: Commands,
) {
    for (pulse, pulse_g_transform) in query {
        for (d_entity, mut d_detectable, d_g_transform) in &mut detectable_query {
            let distance_away = d_g_transform
                .translation()
                .distance(pulse_g_transform.translation());

            let is_detected =
                pulse.inner_radius() < distance_away && distance_away < pulse.outer_radius();

            if is_detected {
                commands.trigger(DetectionEvent::Detected(pulse.clone(), d_entity));
            }
            match (d_detectable.detected, is_detected) {
                // (previously detected, detected now)
                (true, true) => {
                    info!("triggered detected!");
                    commands.trigger(DetectionEvent::Detected(pulse.clone(), d_entity));
                }
                (true, false) => {
                    info!("triggered first detected!");
                    commands.trigger(DetectionEvent::FirstDetected(pulse.clone(), d_entity));
                    commands.trigger(DetectionEvent::Detected(pulse.clone(), d_entity));
                }
                (false, true) => {
                    info!("triggered last detected!");
                    commands.trigger(DetectionEvent::Detected(pulse.clone(), d_entity));
                    //commands.trigger(DetectionEvent::LastDetected(pulse.clone(), d_entity));
                }
                (false, false) => {
                    info!("not before detected and not now!");
                }
            };
            d_detectable.detected = is_detected;
        }
    }
}
