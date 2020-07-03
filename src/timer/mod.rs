#[cfg(test)]
mod test;

extern crate alloc;

use super::time_source;
use super::time_source::TimeSource;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::Cell;
use core::cell::RefCell;

#[derive(Clone, Copy, PartialEq)]
pub struct Timer {
    id: u16,
}

impl Timer {
    fn new(id: u16) -> Self {
        Self { id }
    }
}

pub type Ticks = u32;

struct TimerState<'a> {
    timer: Timer,
    remaining_ticks: Ticks,
    callback: Box<dyn Fn() + 'a>,
}

impl<'a> TimerState<'a> {
    pub fn new(timer: Timer, remaining_ticks: Ticks, callback: impl Fn() + 'a) -> Self {
        Self {
            timer,
            remaining_ticks,
            callback: Box::new(callback),
        }
    }
}

pub struct TimerGroup<'a> {
    timers: RefCell<Vec<TimerState<'a>>>,
    current_id: Cell<u16>,
    time_source: &'a dyn TimeSource,
    last_ticks: Cell<time_source::Ticks>,
}

impl<'a> TimerGroup<'a> {
    pub fn new(time_source: &'a dyn TimeSource) -> Self {
        Self {
            timers: RefCell::new(vec![]),
            current_id: Cell::new(0),
            time_source,
            last_ticks: Cell::new(time_source.ticks()),
        }
    }

    pub fn timer(&self) -> Timer {
        let id = self.current_id.get();
        self.current_id.replace(self.current_id.get() + 1);
        Timer::new(id)
    }

    pub fn start(&self, timer: Timer, ticks: u32, callback: impl Fn() + 'a) {
        self.stop(timer);
        let timer_state = TimerState::new(timer, ticks, callback);
        self.timers.borrow_mut().push(timer_state);
    }

    pub fn stop(&self, timer: Timer) {
        self.timers
            .borrow_mut()
            .retain(|timer_data| timer_data.timer != timer);
    }

    pub fn is_running(&self, timer: Timer) -> bool {
        self.timers.borrow().iter().any(|x| x.timer == timer)
    }

    pub fn remaining_ticks(&self, timer: Timer) -> Option<Ticks> {
        self.timers
            .borrow()
            .iter()
            .find(|x| x.timer == timer)
            .map(|x| x.remaining_ticks)
    }

    pub fn run(&self) {
        let current_ticks = self.time_source.ticks();
        let elapsed_ticks = current_ticks.wrapping_sub(self.last_ticks.get());
        self.last_ticks.replace(current_ticks);

        let mut timer_to_run = None;

        for (i, timer) in self.timers.borrow_mut().iter_mut().enumerate() {
            if timer.remaining_ticks > elapsed_ticks {
                timer.remaining_ticks -= elapsed_ticks;
            } else {
                timer.remaining_ticks = 0;
                timer_to_run.get_or_insert(i);
            }
        }

        if let Some(timer_index) = timer_to_run {
            let timer = { self.timers.borrow_mut().remove(timer_index) };
            self.stop(timer.timer);
            (timer.callback)();
        }
    }
}
