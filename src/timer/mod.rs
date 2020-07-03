#[cfg(test)]
mod test;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::Cell;
use core::cell::RefCell;

#[derive(Clone, Copy)]
pub struct Timer {
    id: u16,
}

impl Timer {
    fn new(id: u16) -> Self {
        Self { id }
    }
}

struct TimerState<'a> {
    timer: Timer,
    remaining_ticks: u32,
    callback: Box<dyn Fn() + 'a>,
}

impl<'a> TimerState<'a> {
    pub fn new(timer: Timer, remaining_ticks: u32, callback: impl Fn() + 'a) -> Self {
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
}

impl<'a> TimerGroup<'a> {
    pub fn new() -> Self {
        Self {
            timers: RefCell::new(vec![]),
            current_id: Cell::new(0),
        }
    }

    pub fn new_timer(&self) -> Timer {
        let id = self.current_id.get();
        self.current_id.replace(self.current_id.get() + 1);
        Timer::new(id)
    }

    pub fn start(&self, timer: Timer, ticks: u32, callback: impl Fn() + 'a) {
        // remove any matching

        let timer_state = TimerState::new(timer, ticks, callback);
        self.timers.borrow_mut().push(timer_state);
    }

    pub fn run(&mut self) {
        for timer in self.timers.borrow().iter() {
            (timer.callback)();
        }
    }
}
