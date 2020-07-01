use super::linked_list::{LinkedList, LinkedListNode};
use core::cell::Cell;
use core::cell::RefCell;

#[cfg(test)]
mod test;

pub struct TimerData<F: Fn()> {
    remaining_ticks: Cell<u32>,
    callback: RefCell<Option<F>>,
}

impl<F: Fn()> TimerData<F> {
    fn new() -> Self {
        Self {
            remaining_ticks: Cell::new(0),
            callback: RefCell::new(None),
        }
    }
}

type Timer<'a, F> = LinkedListNode<'a, TimerData<F>>;

pub struct TimerGroup<'a, F: Fn()> {
    timers: LinkedList<'a, TimerData<F>>,
}

impl<'a, F: Fn()> TimerGroup<'a, F> {
    pub fn new_timer() -> Timer<'a, F> {
        LinkedListNode::new(TimerData::new())
    }

    pub fn new() -> Self {
        Self {
            timers: LinkedList::new(),
        }
    }

    pub fn start(&mut self, timer: &'a Timer<'a, F>, callback: F) {
        timer.value.callback.replace(Some(callback));
        timer.value.remaining_ticks.replace(0);
        self.timers.push_back(timer);
    }

    pub fn run(&mut self) {
        self.timers.for_each(|timer_data| {
            (timer_data.callback.borrow().as_ref().unwrap())();
            true
        })
    }
}
