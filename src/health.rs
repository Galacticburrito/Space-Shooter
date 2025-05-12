use crate::SystemUpdateSet;
use bevy::prelude::*;

pub struct HealthPlugin {}

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, max_propagate_health)
            .add_systems(
                Update,
                (propagate_health, apply_damage).in_set(SystemUpdateSet::Main),
            )
            .register_type::<Health>();
    }
}

/// stable component stating current health
#[derive(Component, Reflect)]
pub struct Health {
    max: f32,
    current: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Health {
            max: amount,
            current: amount,
        }
    }
    /// apply damage, returns if killed or not
    fn damage(&mut self, amount: f32) -> bool {
        if self.current <= amount {
            self.current = 0.;
            info!("entity is now dead!");
            return true;
        }
        self.current -= amount;
        false
    }

    /// what percent of health left?
    pub fn percent(&self) -> f32 {
        self.current / self.max
    }
}

/// add to affected entities so heath system can apply the damage
#[derive(Component)]
pub struct Damage(pub f32);

/// marker component for entities with no health left
#[derive(Component)]
pub struct Killed {}

/// if any damage was added to entity, apply it and remove damage component. If killed, add
/// associated component
fn apply_damage(
    mut query: Query<(Entity, &Damage, &mut Health), Without<Killed>>,
    mut commands: Commands,
) {
    for (entity, damage, mut health) in &mut query {
        if health.damage(damage.0) {
            commands.entity(entity).insert(Killed {});
        }
        commands.entity(entity).remove::<Damage>();
    }
}

/// total health determined by health of children (1 layer down)
/// Health component itself not needed
#[derive(Component)]
pub struct PropagateHealth {
    current: f32,
    max: f32,
}

//TODO: make propagate logic ignore Killed

/// sum up total health of children (1 layer down), so get total max
fn max_propagate_health(
    mut parent: Query<(&mut PropagateHealth, &Children)>,
    health_query: Query<&Health>,
) {
    for (mut parent_health, children) in &mut parent {
        parent_health.max = 0.;
        for &child in children {
            if let Ok(child_health) = health_query.get(child) {
                parent_health.max += child_health.current;
            }
        }
    }
}

/// sum up health of children (1 layer down), then assign that to PropagateHealth
fn propagate_health(
    mut parent: Query<(&mut PropagateHealth, &Children)>,
    health_query: Query<&Health>,
) {
    for (mut parent_health, children) in &mut parent {
        parent_health.current = 0.;
        for &child in children {
            if let Ok(child_health) = health_query.get(child) {
                parent_health.current += child_health.current;
            }
        }
    }
}
