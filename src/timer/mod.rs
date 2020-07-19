use super::callback::Callback;
use super::linked_list::{LinkedList, LinkedListNode};
use core::cell::Cell;

#[cfg(test)]
mod test;

pub struct TimerData<'a> {
    remaining_ticks: Cell<u32>,
    callback: Cell<Option<Callback<'a>>>,
}

impl TimerData<'_> {
    fn new() -> Self {
        Self {
            remaining_ticks: Cell::new(0),
            callback: Cell::new(None),
        }
    }
}

type Timer<'a> = LinkedListNode<'a, TimerData<'a>>;

pub struct TimerGroup<'a> {
    timers: LinkedList<'a, TimerData<'a>>,
}

impl<'a> TimerGroup<'a> {
    pub fn new_timer() -> Timer<'a> {
        LinkedListNode::new(TimerData::new())
    }

    pub fn new() -> Self {
        Self {
            timers: LinkedList::new(),
        }
    }

    pub fn start<Context>(
        &mut self,
        timer: &'a Timer<'a>,
        context: &'a Context,
        callback: fn(context: &Context),
    ) {
        timer.value.remaining_ticks.replace(0);
        timer
            .value
            .callback
            .replace(Some(Callback::new(context, callback)));

        self.timers.push_back(timer);
    }

    pub fn run(&mut self) {
        for timer_data in self.timers.iter() {
            timer_data
                .callback
                .get()
                .expect("Trying to call an empty Timer Callback")
                .call();
        }
    }
}
