use crate::{SystemUpdateSet, collider::Collider};
use bevy::{math::bounding::IntersectsVolume, prelude::*};

pub struct CollisionPlugin {}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, determine_collisions.in_set(SystemUpdateSet::Main));
    }
}

#[derive(Event)]
pub struct CollisionEvent(pub Entity, pub Entity);

impl CollisionEvent {
    /// determines if any of the colliding entities has requested component
    pub fn has_component<T: Component>(&self, world: &World) -> (bool, bool) {
        (
            world.get::<T>(self.0).is_some(),
            world.get::<T>(self.1).is_some(),
        )
    }

    /// gets requested component from each colliding entity
    pub fn get_component<'a, T: Component>(
        &self,
        world: &'a World,
    ) -> (Option<&'a T>, Option<&'a T>) {
        (world.get::<T>(self.0), world.get::<T>(self.1))
    }
}

fn determine_collisions(
    query: Query<(&Collider, &GlobalTransform, Entity)>,
    mut events: EventWriter<CollisionEvent>,
) {
    for [
        (collider1, g_transform_1, entity1),
        (collider2, g_transform_2, entity2),
    ] in query.iter_combinations()
    {
        if has_collided(
            collider1.convert_to_global(g_transform_1.translation()),
            collider2.convert_to_global(g_transform_2.translation()),
        ) {
            events.write(CollisionEvent(entity1, entity2));
        }
    }
}

fn has_collided(collider1: Collider, collider2: Collider) -> bool {
    match (collider1, collider2) {
        (Collider::Rectangle(aabb1), Collider::Rectangle(aabb2)) => {
            return aabb1.intersects(&aabb2);
        }

        (Collider::Rectangle(aabb), Collider::Circle(circle))
        | (Collider::Circle(circle), Collider::Rectangle(aabb)) => {
            return aabb.intersects(&circle);
        }

        (Collider::Circle(circle_1), Collider::Circle(circle_2)) => {
            return circle_1.intersects(&circle_2);
        }
    }
}
