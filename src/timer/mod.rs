use super::callback::Callback;
use super::linked_list::{LinkedList, LinkedListNode};
use super::time_source::TimeSource;
use core::cell::Cell;

pub use super::time_source::Ticks;

#[cfg(test)]
mod test;

pub struct TimerData<'a> {
    periodic: Cell<bool>,
    start_ticks: Cell<Ticks>,
    remaining_ticks: Cell<Ticks>,
    callback: Cell<Option<Callback<'a>>>,
}

impl TimerData<'_> {
    const fn new() -> Self {
        Self {
            periodic: Cell::new(false),
            start_ticks: Cell::new(0),
            remaining_ticks: Cell::new(0),
            callback: Cell::new(None),
        }
    }
}

pub type Timer<'a> = LinkedListNode<'a, TimerData<'a>>;

pub struct TimerGroup<'a> {
    timers: LinkedList<'a, TimerData<'a>>,
    last_ticks: Cell<Ticks>,
    next_ready: Cell<Ticks>,
    time_source: &'a dyn TimeSource,
}

impl<'a> TimerGroup<'a> {
    pub const fn new_timer() -> Timer<'a> {
        LinkedListNode::new(TimerData::new())
    }

    pub fn new(time_source: &'a dyn TimeSource) -> Self {
        Self {
            timers: LinkedList::new(),
            last_ticks: Cell::new(time_source.ticks()),
            next_ready: Cell::new(0),
            time_source,
        }
    }

    fn add_timer(&self, timer: &'a Timer<'a>) {
        if timer.start_ticks.get() < self.next_ready.get() {
            self.next_ready.set(timer.start_ticks.get());
        }

        self.timers.remove(timer);
        self.timers.push_back(timer);
    }

    fn start_internal<Context>(
        &self,
        periodic: bool,
        timer: &'a Timer<'a>,
        ticks: Ticks,
        context: &'a Context,
        callback: fn(context: &'a Context),
    ) {
        timer.periodic.set(periodic);
        timer.start_ticks.set(ticks);
        timer.remaining_ticks.set(ticks);
        timer.callback.set(Some(Callback::new(context, callback)));

        self.add_timer(timer);
    }

    pub fn start<Context>(
        &self,
        timer: &'a Timer<'a>,
        ticks: Ticks,
        context: &'a Context,
        callback: fn(context: &'a Context),
    ) {
        self.start_internal(false, timer, ticks, context, callback);
    }

    pub fn start_periodic<Context>(
        &self,
        timer: &'a Timer<'a>,
        ticks: Ticks,
        context: &'a Context,
        callback: fn(context: &'a Context),
    ) {
        self.start_internal(true, timer, ticks, context, callback);
    }

    pub fn stop(&self, timer: &'a Timer<'a>) {
        self.timers.remove(timer);
    }

    pub fn running(&self, timer: &'a Timer<'a>) -> bool {
        self.timers.contains(timer)
    }

    pub fn remaining_ticks(&self, timer: &Timer) -> Ticks {
        timer.remaining_ticks.get()
    }

    pub fn run(&self) -> Ticks {
        self.next_ready.set(Ticks::max_value());
        let current_ticks = self.time_source.ticks();
        let delta_ticks = current_ticks.wrapping_sub(self.last_ticks.get());
        self.last_ticks.set(current_ticks);

        for timer in self.timers.iter() {
            if timer.remaining_ticks.get() > delta_ticks {
                timer
                    .remaining_ticks
                    .set(timer.remaining_ticks.get() - delta_ticks);

                if timer.remaining_ticks.get() < self.next_ready.get() {
                    self.next_ready.set(timer.remaining_ticks.get());
                }
            } else {
                timer.remaining_ticks.set(0);
            }
        }

        for timer in self.timers.iter() {
            if timer.remaining_ticks.get() == 0 {
                timer.callback.get().unwrap().call();

                if timer.periodic.get() && self.timers.contains(timer) {
                    timer.remaining_ticks.set(timer.start_ticks.get());
                    self.add_timer(timer);
                } else {
                    self.timers.remove(timer);
                }
            }
            return self.next_ready.get();
        }

        self.next_ready.get()
    }
}
