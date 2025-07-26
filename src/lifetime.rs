use bevy::prelude::*;

use crate::SystemUpdateSet;

pub struct LifetimePlugin {}

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lifetime_syst.in_set(SystemUpdateSet::Main));

        app.register_type::<Lifetime>();
    }
}

#[derive(Component, Reflect)]
pub struct Lifetime {
    pub timer: Timer,
    total_time: f32,
}

impl Lifetime {
    pub fn new(seconds: f32) -> Self {
        Lifetime {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
            total_time: seconds,
        }
    }

    /// percentage of lifetime already lived
    pub fn percent_lived(&self) -> f32 {
        self.timer.remaining_secs() / self.total_time
    }
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::new(10.)
    }
}

/// despawns entity when lifetime timer expires
fn lifetime_syst(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
