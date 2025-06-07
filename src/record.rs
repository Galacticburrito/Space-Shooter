use crate::schedule::UpdateSchedule;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct RecordPlugin {}

impl Plugin for RecordPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Record<GlobalTransform>>();
    }
}

/// stores past data (most recent to least) about the specified component
/// NOTE: don't forget to add record::update_record<T> to app!  
#[derive(Component, Reflect)]
pub struct Record<T: Component> {
    /// contains past data from Update schedule
    pub update_deq: VecDeque<T>,
    /// contains past data from FixedUpdate schedule
    pub fixed_deq: VecDeque<T>,
}

impl<T: Component + Clone> Record<T> {
    fn new(init_val: &T) -> Self {
        let mut update_deq = VecDeque::new();
        update_deq.push_front(init_val.clone());
        let mut fixed_deq = VecDeque::new();
        fixed_deq.push_front(init_val.clone());
        Record {
            update_deq,
            fixed_deq,
        }
    }

    /// give the correct VecDeque based on UpdateSchedule
    pub fn deq(&self, schedule: &UpdateSchedule) -> &VecDeque<T> {
        if *schedule == UpdateSchedule::Update {
            &self.update_deq
        } else {
            &self.fixed_deq
        }
    }

    pub fn deq_mut(&mut self, schedule: &UpdateSchedule) -> &mut VecDeque<T> {
        if *schedule == UpdateSchedule::Update {
            &mut self.update_deq
        } else {
            &mut self.fixed_deq
        }
    }

    pub fn newest(&self, schedule: &UpdateSchedule) -> &T {
        self.deq(schedule).front().unwrap()
    }

    pub fn newest_2(&self, schedule: UpdateSchedule) -> (&T, &T) {
        let both = self.elements_at(&[0, 1], schedule);
        (both[0], both[1])
    }

    pub fn oldest(&self, schedule: UpdateSchedule) -> &T {
        self.deq(&schedule).back().unwrap()
    }

    pub fn elements_at(&self, indexes: &[usize], schedule: UpdateSchedule) -> Vec<&T> {
        indexes.iter().map(|i| &self.deq(&schedule)[*i]).collect()
    }
}

/// records current component to Record
/// if no Record exists for given component, make empty one
/// NOTE: best to put in the component's module, so don't define this more than once for the same
/// component
/// WARN: add both this and record_fixed_update!
pub fn record_update<T: Component + Clone>(
    mut query: Query<(Entity, Option<&mut Record<T>>, &T)>,
    mut commands: Commands,
) {
    for (entity, opt_record, component) in &mut query {
        let Some(mut record) = opt_record else {
            commands.entity(entity).insert(Record::new(component));
            continue;
        };

        update(&mut record, &component, &UpdateSchedule::Update);
    }
}

/// same as record_update
/// NOTE: best to put in the component's module, so don't define this more than once for the same
/// component
/// WARN: add both this and record_update!
pub fn record_fixed_update<T: Component + Clone>(
    mut query: Query<(Entity, Option<&mut Record<T>>, &T)>,
    mut commands: Commands,
) {
    for (entity, opt_record, component) in &mut query {
        let Some(mut record) = opt_record else {
            commands.entity(entity).insert(Record::new(component));
            continue;
        };

        update(&mut record, &component, &UpdateSchedule::Update);
    }
}

fn update<T: Component + Clone>(record: &mut Record<T>, component: &T, schedule: &UpdateSchedule) {
    let history_len = 20;
    record.deq_mut(schedule).push_front(component.clone());

    if record.deq_mut(schedule).len() > history_len {
        record.deq_mut(schedule).pop_back();
    }
}
