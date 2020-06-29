extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;

pub struct Event<T, F: Fn(&T)> {
    subscribers: Vec<F>,
    phantom: PhantomData<T>,
}

impl<T, F: Fn(&T)> Event<T, F> {
    pub fn new() -> Self {
        Self {
            subscribers: vec![],
            phantom: PhantomData,
        }
    }

    pub fn subscribe(&mut self, f: F) {
        self.subscribers.push(f);
    }

    pub fn publish(&mut self, t: &T) {
        for subscriber in &mut self.subscribers {
            subscriber(t);
        }
    }
}

#[cfg(test)]
mod test;
