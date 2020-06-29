use super::linked_list::LinkedList;
use super::linked_list::LinkedListNode;
use core::marker::PhantomData;

pub struct EventSubscription<T, F: Fn(&T)> {
    callback: F,
    phantom: PhantomData<T>,
}

impl<T, F: Fn(&T)> EventSubscription<T, F> {
    pub fn new(callback: F) -> Self {
        Self {
            callback,
            phantom: PhantomData,
        }
    }
}

pub struct Event<'a, T, F: Fn(&T)> {
    subscribers: LinkedList<'a, EventSubscription<T, F>>,
    phantom: PhantomData<T>,
}

impl<'a, T, F: Fn(&T)> Event<'a, T, F> {
    pub fn new() -> Self {
        Self {
            subscribers: LinkedList::new(),
            phantom: PhantomData,
        }
    }

    pub fn subscribe(&mut self, subscription: &'a LinkedListNode<'a, EventSubscription<T, F>>) {
        self.subscribers.push_front(subscription);
    }

    pub fn publish(&mut self, t: &T) {
        self.subscribers.for_each(|subscriber| {
            (subscriber.callback)(t);
            true
        });
    }
}

#[cfg(test)]
mod test;
