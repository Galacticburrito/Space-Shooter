use crate::{
    Health, SystemUpdateSet,
    health::PropagateHealth,
    player::Player,
    ship_composition::engine::{Engine, EngineType},
};
use bevy::prelude::*;
use bevy_behave::prelude::*;

pub struct AiShipTreePlugin {}

impl Plugin for AiShipTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flee.in_set(SystemUpdateSet::Main))
            .add_observer(is_health_critical)
            .add_observer(has_child_component::<Health>);
    }
}

/// add behavior tree component to entity, along with logging if requested
pub fn add_ai_ship_tree(entity: Entity, commands: &mut Commands, debug: bool) {
    let tree = commands
        .spawn(match debug {
            true => BehaveTree::new(ai_ship_tree()).with_logging(true),
            false => BehaveTree::new(ai_ship_tree()),
        })
        .id();

    commands.entity(entity).add_child(tree);
}

fn ai_ship_tree() -> Tree<Behave> {
    info!("making ai tree!");
    behave!(
        Behave::Forever => {
            Behave::Sequence => {
                Behave::trigger(HasChildComponentTask::<Health>::default()),
                Behave::trigger(IsHealthCriticalTask::new(1.)),
                Behave::spawn_named("Flee", FleeTask::new(100.)),
            }
        }
    )
}

#[derive(Clone, Component)]
struct HasChildComponentTask<T: Component> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> Default for HasChildComponentTask<T> {
    fn default() -> Self {
        HasChildComponentTask {
            _marker: std::marker::PhantomData,
        }
    }
}

/// if any child has the given component, a success, otherwise failure
fn has_child_component<T: Component + Clone>(
    trigger: Trigger<BehaveTrigger<HasChildComponentTask<T>>>,
    children_query: Query<&Children>,
    component_query: Query<(), With<T>>,
    mut commands: Commands,
) {
    let ctx = trigger.ctx();

    let Ok(children) = children_query.get(ctx.target_entity()) else {
        warn!("no child component found!");
        commands.trigger(ctx.failure());
        return;
    };

    let mut found_component = false;
    for child in children {
        if component_query.contains(*child) {
            found_component = true;
            break;
        }
    }

    match found_component {
        true => {
            // a child has the component
            commands.trigger(ctx.success());
        }
        false => {
            // no child has the component
            commands.trigger(ctx.failure());
        }
    }
}

#[derive(Clone, Component)]
struct IsHealthCriticalTask {
    percent_threshold: f32,
}

impl IsHealthCriticalTask {
    fn new(percent_threshold: f32) -> Self {
        IsHealthCriticalTask { percent_threshold }
    }
}

/// returns success if ship's health is critically low, otherwise fails
fn is_health_critical(
    trigger: Trigger<BehaveTrigger<IsHealthCriticalTask>>,
    health_query: Query<&PropagateHealth>,
    mut commands: Commands,
) {
    let ctx = trigger.ctx();
    let task = trigger.inner();

    let Ok(propagate_health) = health_query.get(ctx.target_entity()) else {
        warn!("no popagate_health component found on entity!");
        commands.trigger(ctx.failure());
        return;
    };
    let mut health_critical = false;
    if propagate_health.percent() < task.percent_threshold {
        health_critical = true;
    }

    match health_critical {
        true => {
            // health is critical
            commands.trigger(ctx.success());
        }
        false => {
            // health isn't critical yet
            commands.trigger(ctx.failure());
        }
    }
}

#[derive(Clone, Component)]
struct FleeTask {
    /// how far away from enemies should ai go before done fleeing?
    safe_distance: f32,
}

impl FleeTask {
    fn new(safe_distance: f32) -> Self {
        FleeTask { safe_distance }
    }
}

/// thrust full speed away from battle
/// for now, just accelerate in current direction until safe distance from player
fn flee(
    query: Query<(&FleeTask, &Children, &GlobalTransform, &BehaveCtx)>,
    mut engines: Query<&mut Engine>,
    player: Query<&GlobalTransform, With<Player>>,
    mut commands: Commands,
) -> Result<(), BevyError> {
    for (task, children, g_transform, ctx) in query {
        for child in children {
            let Ok(mut engine) = engines.get_mut(*child) else {
                continue;
            };

            match engine.engine_type {
                EngineType::Main => {
                    engine.full_throttle();
                }
                EngineType::Thruster => {
                    // don't want to turn
                    engine.no_throttle();
                }
            }
        }

        let player_transform = player.single()?;
        if g_transform
            .translation()
            .distance(player_transform.translation())
            > task.safe_distance
        {
            // ai ship is far enough away from player, so done fleeing
            info!("done fleeing!");
            commands.trigger(ctx.success());
        }
    }
    Ok(())
}
