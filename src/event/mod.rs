use super::linked_list::{LinkedList, LinkedListNode};
use core::marker::PhantomData;

#[cfg(test)]
mod test;

type EventSubscription<'a, F> = LinkedListNode<'a, F>;

pub struct Event<'a, T, F: Fn(&T)> {
    subscribers: LinkedList<'a, F>,
    phantom: PhantomData<T>,
}

impl<'a, T, F: Fn(&T)> Event<'a, T, F> {
    pub fn new() -> Self {
        Self {
            subscribers: LinkedList::new(),
            phantom: PhantomData,
        }
    }

    pub fn subscribe(&mut self, subscription: &'a EventSubscription<'a, F>) {
        self.subscribers.push_front(subscription);
    }

    pub fn publish(&mut self, t: &T) {
        self.subscribers.for_each(|subscriber| {
            subscriber(t);
            true
        });
    }
}
