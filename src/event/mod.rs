#[cfg(test)]
mod test;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub struct Event<'a, T> {
    subscribers: Vec<Box<dyn Fn(&T) + 'a>>,
}

impl<'a, T> Event<'a, T> {
    pub fn new() -> Self {
        Self {
            subscribers: vec![],
        }
    }

    pub fn subscribe(&mut self, f: impl Fn(&T) + 'a) {
        self.subscribers.push(Box::new(f));
    }

    pub fn publish(&mut self, t: &T) {
        for subscriber in &self.subscribers {
            subscriber(t);
        }
    }
}
