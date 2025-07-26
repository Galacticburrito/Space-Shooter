use super::particle::{Particle, ParticleData};
use crate::{
    AppState, SystemUpdateSet, color_palette::PalColor, graphic::Graphic, lifetime::Lifetime,
    rotation, velocity::Velocity,
};
use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;
use std::time::Duration;

pub struct EmitterPlugin {}

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameReady), setup_test)
            .add_systems(
                Update,
                (spawn_particles, alter_particle_color_size).in_set(SystemUpdateSet::Main),
            )
            .register_type::<ParticleEmitter>();
    }
}

#[derive(Component, Clone, Reflect)]
#[require(Transform)]
pub struct ParticleEmitter {
    pub spawn_timer: Timer,
    pub velocity_angle_range: Range<f32>, // degrees
    pub particle_data: ParticleData,
}

impl ParticleEmitter {
    /// `particle_spawn_rate`: particles/sec
    /// `velocity_angle_range`: radians
    pub fn new(
        particle_spawn_rate: f32,
        velocity_angle_range: Range<f32>,
        particle_data: ParticleData,
    ) -> Self {
        ParticleEmitter {
            spawn_timer: Timer::new(
                Duration::from_secs_f32(1. / particle_spawn_rate),
                TimerMode::Repeating,
            ),
            velocity_angle_range,
            particle_data,
        }
    }

    pub fn alter_spawn_rate(&mut self, spawn_rate: f32) {
        let duration = if spawn_rate == 0. {
            Duration::MAX
        } else {
            Duration::from_secs_f32(1. / spawn_rate)
        };

        self.spawn_timer = Timer::new(duration, TimerMode::Repeating);
    }
}

pub fn setup_test(mut commands: Commands) {
    commands.spawn(ParticleEmitter::new(
        1.,
        -10.0..10.,
        ParticleData::new(
            Circle::new(5.).into(),
            1.,
            50.,
            (PalColor::Green, PalColor::Red),
            (1., 0.1),
        ),
    ));
}

pub fn spawn_particles(
    mut query: Query<(&mut ParticleEmitter, &Transform)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut rng = rand::rng();

    for (mut emitter, transform) in &mut query {
        emitter.spawn_timer.tick(time.delta());

        if emitter.spawn_timer.finished() {
            let particles_to_spawn = 1;
            let p_data = emitter.particle_data.clone();

            for _ in 0..particles_to_spawn {
                let deg_angle = rng.random_range(emitter.velocity_angle_range.clone());

                commands.spawn((
                    transform
                        .with_scale(Vec3::new(1., 1., 1.) * emitter.particle_data.size_start_end.0),
                    Particle(emitter.particle_data.clone()),
                    Lifetime::new(p_data.lifetime),
                    Velocity(rotation::rad_to_vec2(deg_angle.to_radians()) * p_data.speed),
                    Graphic::new(p_data.shape.clone(), p_data.color_start_end.0),
                ));
                info!("spawned particle!");
            }
        }
    }
}

/// change particle size and color over time
pub fn alter_particle_color_size(
    mut query: Query<(Entity, &Particle, &Lifetime, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, particle, lifetime, mut transform) in &mut query {
        transform.scale = (Vec3::ONE * particle.0.size_start_end.0).lerp(
            Vec3::ONE * particle.0.size_start_end.1,
            lifetime.percent_lived(),
        );
        commands.entity(entity).insert(Graphic::new(
            particle.0.shape.clone(),
            lerp_color(particle.0.color_start_end, lifetime.percent_lived()),
        ));
    }
}

fn lerp_color(color_start_end: (PalColor, PalColor), percent: f32) -> PalColor {
    if color_start_end.0 == color_start_end.1 {
        return color_start_end.0;
    }
    color_start_end.0.mix(&color_start_end.1, percent)
}
