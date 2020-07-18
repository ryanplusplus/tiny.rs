use super::linked_list::{LinkedList, LinkedListNode};

#[cfg(test)]
mod test;

pub struct EventSubscriptionState<Args> {
    context: *const (),
    f: fn(*const (), &Args),
}

pub type EventSubscription<'a, Args> = LinkedListNode<'a, EventSubscriptionState<Args>>;

pub struct Event<'a, Args> {
    subscribers: LinkedList<'a, EventSubscriptionState<Args>>,
}

impl<'a, Args> Event<'a, Args> {
    pub fn new() -> Self {
        Self {
            subscribers: LinkedList::new(),
        }
    }

    pub fn new_subscription<Context>(
        context: &'a Context,
        f: fn(&Context, &Args),
    ) -> EventSubscription<'a, Args> {
        EventSubscription::new(EventSubscriptionState {
            context: unsafe { core::intrinsics::transmute(context) },
            f: unsafe { core::intrinsics::transmute(f) },
        })
    }

    pub fn subscribe(&mut self, subscription: &'a EventSubscription<'a, Args>) {
        self.subscribers.push_front(subscription);
    }

    pub fn publish(&mut self, args: &Args) {
        self.subscribers.for_each(|subscriber| {
            (subscriber.f)(subscriber.context, args);
            true
        });
    }
}
