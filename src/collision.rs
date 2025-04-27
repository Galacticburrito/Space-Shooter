use crate::SystemUpdateSet;
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

pub struct CollisionPlugin {}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, determine_collisions.in_set(SystemUpdateSet::Main));
    }
}

fn setup() {}

#[derive(Component)]
pub enum Collider {
    Rectangle(Aabb2d),
    Circle(BoundingCircle),
}

impl Collider {
    pub fn new_rect(width: f32, height: f32) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        Collider::Rectangle(Aabb2d::new(Vec2::ZERO, Vec2::new(half_width, half_height)))
    }

    pub fn new_circle(radius: f32) -> Self {
        Collider::Circle(BoundingCircle::new(Vec2::ZERO, radius))
    }

    fn convert_to_global(&self, g_translation: Vec3) -> Collider {
        match self {
            Collider::Rectangle(aabb) => Collider::Rectangle(Aabb2d::new(
                aabb.center() + g_translation.xy(),
                aabb.half_size(),
            )),
            Collider::Circle(circle) => Collider::Circle(BoundingCircle::new(
                circle.center + g_translation.xy(),
                circle.radius(),
            )),
        }
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
            events.send(CollisionEvent(entity1, entity2));
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

        _ => {
            warn!("collision combination not implemented yet!");
            return false;
        }
    }
}
