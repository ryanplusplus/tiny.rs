use super::callback::CallbackWith1Argument as EventSubscriptionState;
use super::linked_list::{LinkedList, LinkedListNode};
#[cfg(test)]
mod test;

pub type EventSubscription<'a, Arg> = LinkedListNode<'a, EventSubscriptionState<'a, Arg>>;

pub struct Event<'a, Arg> {
    subscribers: LinkedList<'a, EventSubscriptionState<'a, Arg>>,
}

impl<'a, Arg> Event<'a, Arg> {
    pub fn new() -> Self {
        Self {
            subscribers: LinkedList::new(),
        }
    }

    pub fn new_subscription<Context>(
        context: &'a Context,
        f: fn(&Context, &Arg),
    ) -> EventSubscription<'a, Arg> {
        EventSubscription::new(EventSubscriptionState::make(context, f))
    }

    pub fn subscribe(&mut self, subscription: &'a EventSubscription<'a, Arg>) {
        self.subscribers.push_front(subscription);
    }

    pub fn publish(&mut self, args: &Arg) {
        for subscriber in self.subscribers.iter() {
            subscriber.call(args);
        }
    }
}
