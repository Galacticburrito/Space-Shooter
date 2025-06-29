use crate::schedule::UpdateSchedule;
use bevy::prelude::*;
use std::{collections::VecDeque, time::Duration};

pub struct RecordPlugin {}

impl Plugin for RecordPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Record<GlobalTransform>>();
    }
}

#[derive(Clone, Reflect)]
pub struct RecordEntry<T: Component> {
    pub val: T,
    /// time elapsed since beginning of game
    pub time_stamp: Duration,
}

impl<T: Component> RecordEntry<T> {
    pub fn new(component: T, time: &Res<Time>) -> Self {
        let time_stamp = time.elapsed();
        RecordEntry {
            val: component,
            time_stamp,
        }
    }
}

/// stores past data (most recent to least) about the specified component
/// NOTE: don't forget to add [record_update] to app!  
#[derive(Component, Reflect)]
pub struct Record<T: Component> {
    /// contains past data from Update schedule
    pub update_deq: VecDeque<RecordEntry<T>>,
    /// contains past data from FixedUpdate schedule
    pub fixed_deq: VecDeque<RecordEntry<T>>,
    /// amount of time RecordEntrys  are kept
    /// defaults to 0.5 secs
    history_len: f32,
}

impl<T: Component + Clone> Record<T> {
    fn new(init_val: &T, time: &Res<Time>) -> Self {
        let mut update_deq = VecDeque::new();
        update_deq.push_front(RecordEntry::new(init_val.clone(), time));

        let mut fixed_deq = VecDeque::new();
        fixed_deq.push_front(RecordEntry::new(init_val.clone(), time));

        Record {
            update_deq,
            fixed_deq,
            history_len: 0.5,
        }
    }

    /// give the correct VecDeque based on UpdateSchedule
    pub fn deq(&self, schedule: &UpdateSchedule) -> &VecDeque<RecordEntry<T>> {
        if *schedule == UpdateSchedule::Update {
            &self.update_deq
        } else {
            &self.fixed_deq
        }
    }

    /// mut wrapper around [Record::deq]
    pub fn deq_mut(&mut self, schedule: &UpdateSchedule) -> &mut VecDeque<RecordEntry<T>> {
        if *schedule == UpdateSchedule::Update {
            &mut self.update_deq
        } else {
            &mut self.fixed_deq
        }
    }

    /// give newest `amount` data
    /// WARN: no guarantee returned vector is size of `amount`!
    pub fn newest_number(&self, amount: usize, schedule: &UpdateSchedule) -> Vec<&RecordEntry<T>> {
        self.deq(schedule).iter().take(amount).collect()
    }

    /// get all elements added to records within last `seconds_before` seconds
    /// updates history_len to be at least this long, if needed
    pub fn newest_within_secs(
        &mut self,
        seconds_before: f32,
        schedule: &UpdateSchedule,
        time: &Res<Time>,
    ) -> Vec<&RecordEntry<T>> {
        if self.history_len < seconds_before {
            self.history_len = seconds_before;
        }

        self.deq(schedule)
            .iter()
            .filter(|entry| {
                time.elapsed() - entry.time_stamp <= Duration::from_secs_f32(seconds_before)
            })
            .collect()
    }

    pub fn elements_at(
        &self,
        indexes: &[usize],
        schedule: &UpdateSchedule,
    ) -> Vec<&RecordEntry<T>> {
        indexes.iter().map(|i| &self.deq(schedule)[*i]).collect()
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
    time: Res<Time>,
) {
    for (entity, opt_record, component) in &mut query {
        let Some(mut record) = opt_record else {
            commands
                .entity(entity)
                .insert(Record::new(component, &time));
            continue;
        };

        update(&mut record, component, &UpdateSchedule::Update, &time);
    }
}

/// same as [record_update]
/// NOTE: best to put in the component's module, so don't define this more than once for the same
/// component
/// WARN: add both this and record_update!
pub fn record_fixed_update<T: Component + Clone>(
    mut query: Query<(Entity, Option<&mut Record<T>>, &T)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, opt_record, component) in &mut query {
        let Some(mut record) = opt_record else {
            commands
                .entity(entity)
                .insert(Record::new(component, &time));
            continue;
        };

        update(&mut record, component, &UpdateSchedule::FixedUpdate, &time);
    }
}

fn update<T: Component + Clone>(
    record: &mut Record<T>,
    component: &T,
    schedule: &UpdateSchedule,
    time: &Res<Time>,
) {
    let history_len = 20;
    record
        .deq_mut(schedule)
        .push_front(RecordEntry::new(component.clone(), time));

    if record.deq_mut(schedule).len() > history_len {
        record.deq_mut(schedule).pop_back();
    }
}
