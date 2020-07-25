use super::callback::Callback;
use super::linked_list::{LinkedList, LinkedListNode};
use super::time_source::TimeSource;
use core::cell::Cell;

pub use super::time_source::Ticks;

#[cfg(test)]
mod test;

pub struct TimerData<'a> {
    remaining_ticks: Cell<u32>,
    callback: Cell<Option<Callback<'a>>>,
}

impl TimerData<'_> {
    const fn new() -> Self {
        Self {
            remaining_ticks: Cell::new(0),
            callback: Cell::new(None),
        }
    }
}

type Timer<'a> = LinkedListNode<'a, TimerData<'a>>;

pub struct TimerGroup<'a> {
    timers: LinkedList<'a, TimerData<'a>>,
    last_ticks: Ticks,
    time_source: &'a dyn TimeSource,
}

impl<'a> TimerGroup<'a> {
    pub const fn new_timer() -> Timer<'a> {
        LinkedListNode::new(TimerData::new())
    }

    pub fn new(time_source: &'a dyn TimeSource) -> Self {
        Self {
            timers: LinkedList::new(),
            last_ticks: time_source.ticks(),
            time_source,
        }
    }

    pub fn start<Context>(
        &mut self,
        timer: &'a Timer<'a>,
        ticks: Ticks,
        context: &'a Context,
        callback: fn(context: &Context),
    ) {
        timer.remaining_ticks.replace(ticks);
        timer
            .callback
            .replace(Some(Callback::new(context, callback)));

        self.timers.push_back(timer);
    }

    pub fn remaining_ticks(&self, timer: &Timer) -> Ticks {
        timer.remaining_ticks.get()
    }

    pub fn run(&mut self) {
        let current_ticks = self.time_source.ticks();
        let delta_ticks = current_ticks.wrapping_sub(self.last_ticks);
        self.last_ticks = current_ticks;

        for (timer, remaining_ticks) in self
            .timers
            .iter()
            .map(|timer| (timer, timer.remaining_ticks.get()))
        {
            if remaining_ticks > delta_ticks {
                timer.remaining_ticks.replace(remaining_ticks - delta_ticks);
            } else {
                timer.remaining_ticks.replace(0);

                timer
                    .callback
                    .get()
                    .expect("Trying to call an empty Timer Callback")
                    .call();
            }
        }
    }
}
