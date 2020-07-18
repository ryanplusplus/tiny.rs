use super::linked_list::{LinkedList, LinkedListNode};
use core::cell::Cell;

#[cfg(test)]
mod test;

pub struct TimerData {
    remaining_ticks: Cell<u32>,
    context: Cell<Option<*const ()>>,
    callback: Cell<Option<fn(*const ())>>,
}

impl TimerData {
    fn new() -> Self {
        Self {
            remaining_ticks: Cell::new(0),
            context: Cell::new(None),
            callback: Cell::new(None),
        }
    }
}

type Timer<'a> = LinkedListNode<'a, TimerData>;

pub struct TimerGroup<'a> {
    timers: LinkedList<'a, TimerData>,
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
        context: &Context,
        callback: fn(context: &Context),
    ) {
        timer.value.remaining_ticks.replace(0);
        timer
            .value
            .context
            .replace(Some(unsafe { core::intrinsics::transmute(context) }));
        timer
            .value
            .callback
            .replace(Some(unsafe { core::intrinsics::transmute(callback) }));

        self.timers.push_back(timer);
    }

    pub fn run(&mut self) {
        self.timers.for_each(|timer_data| {
            (timer_data.callback.get().unwrap())(timer_data.context.get().unwrap());
            true
        })
    }
}
