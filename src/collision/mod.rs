use crate::SystemUpdateSet;
use bevy::{ecs::query::QueryFilter, math::bounding::IntersectsVolume, prelude::*};
pub mod collider;
use collider::Collider;
pub mod collider_type;
use collider_type::ColliderType;

pub struct CollisionPlugin {}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, determine_collisions.in_set(SystemUpdateSet::Main))
            .register_type::<Collider>();
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
        if !collider1.can_collide_with(collider2) {
            // the collision layers cannot collide
            continue;
        }
        if has_collided(
            collider1
                .bounding
                .convert_to_global(g_transform_1.translation()),
            collider2
                .bounding
                .convert_to_global(g_transform_2.translation()),
        ) {
            events.write(CollisionEvent(entity1, entity2));
        }
    }
}

fn has_collided(collider1: ColliderType, collider2: ColliderType) -> bool {
    match (collider1, collider2) {
        (ColliderType::Rectangle(aabb1), ColliderType::Rectangle(aabb2)) => {
            aabb1.intersects(&aabb2)
        }

        (ColliderType::Rectangle(aabb), ColliderType::Circle(circle))
        | (ColliderType::Circle(circle), ColliderType::Rectangle(aabb)) => aabb.intersects(&circle),

        (ColliderType::Circle(circle1), ColliderType::Circle(circle2)) => {
            circle1.intersects(&circle2)
        }

        (ColliderType::Ring(inner1, outer1), ColliderType::Ring(inner2, outer2)) => {
            //TODO: fix! not work right
            outer1.intersects(&outer2) && !inner1.intersects(&inner2)
        }

        (ColliderType::Ring(inner, outer), ColliderType::Circle(circle))
        | (ColliderType::Circle(circle), ColliderType::Ring(inner, outer)) => {
            // outer.intersects(&circle) && !inner.intersects(&circle)
            if !outer.intersects(&circle) {
                return false;
            }

            // if circle fully within ring (not touching it), then not collision
            let circle_fully_in_inner =
                inner.center.distance(circle.center) + circle.radius() <= inner.radius();
            !circle_fully_in_inner
        }

        (ColliderType::Ring(inner, outer), ColliderType::Rectangle(aabb))
        | (ColliderType::Rectangle(aabb), ColliderType::Ring(inner, outer)) => {
            // outer.intersects(&rectangle) && !inner.intersects(&rectangle)
            if !outer.intersects(&aabb) {
                return false;
            }

            // if rectangle fully within ring (not touching it), then not collision
            let inner_collider = inner.circle.into();
            let rect_corners_in_inner = contains_point(&inner_collider, aabb.min)
                && contains_point(&inner_collider, aabb.max)
                && contains_point(&inner_collider, Vec2::new(aabb.min.x, aabb.max.y))
                && contains_point(&inner_collider, Vec2::new(aabb.max.x, aabb.min.y));
            !rect_corners_in_inner
        }
    }
}

/// WARN: for now, only BoundingCircle implimented!
fn contains_point(collider: &ColliderType, point: Vec2) -> bool {
    match collider {
        ColliderType::Circle(circle) => circle.center.distance(point) <= circle.radius(),
        ColliderType::Rectangle(aabb) => {
            // inside rect if x is between min.x and max.x, and y is between min.y and max.y
            point.x >= aabb.min.x
                && point.x <= aabb.max.x
                && point.y >= aabb.min.y
                && point.y <= aabb.max.y
        }
        _ => todo!("impliment point detection for collidertype!"),
    }
}

/// if entity has collided with the component, return it, otherwise return None
pub fn collided_with_component<'a, T: Component, Q: QueryFilter>(
    entity: Entity,
    events: &mut EventReader<CollisionEvent>,
    query: &'a Query<&T, Q>,
) -> Option<&'a T> {
    for collision in events.read() {
        let (entity1, entity2) = (collision.0, collision.1);
        // info!("collision: {} <-> {}", entity1, entity2);

        let other_entity = if entity1 == entity {
            entity2
        } else if entity2 == entity {
            entity1
        } else {
            // entity not part of collision event
            continue;
        };

        if let Ok(t) = query.get(other_entity) {
            return Some(t);
        }
    }
    None
}

/// see if entity has collided with any other entity that has given component
pub fn has_collided_with_component<T: Component>(
    entity: Entity,
    events: &mut EventReader<CollisionEvent>,
    world: &World,
) -> bool {
    for collision in events.read() {
        let (entity1, entity2) = (collision.0, collision.1);

        let other_entity = if entity1 == entity {
            entity2
        } else if entity2 == entity {
            entity1
        } else {
            // entity not part of collision event
            continue;
        };

        if world.get::<T>(other_entity).is_some() {
            return true;
        }
    }
    false
}
