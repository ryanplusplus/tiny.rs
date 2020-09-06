use core::{
    cell::Cell,
    ops::{Deref, DerefMut},
};

mod iter;

#[cfg(test)]
mod test;

pub struct LinkedListNode<'a, T> {
    next: Cell<Option<&'a LinkedListNode<'a, T>>>,
    pub value: T,
}

impl<T> LinkedListNode<'_, T> {
    pub const fn new(value: T) -> Self {
        Self {
            next: Cell::new(None),
            value,
        }
    }
}

impl<T> Deref for LinkedListNode<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for LinkedListNode<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub struct LinkedList<'a, T> {
    head: Cell<Option<&'a LinkedListNode<'a, T>>>,
}

impl<'a, T> LinkedList<'a, T> {
    pub const fn new() -> Self {
        Self {
            head: Cell::new(None),
        }
    }

    pub fn push_front(&self, node: &'a LinkedListNode<'a, T>) {
        if let Some(head) = self.head.take() {
            node.next.set(Some(head));
        }
        self.head.set(Some(node));
    }

    pub fn push_back(&self, node: &'a LinkedListNode<'a, T>) {
        match self.head.get() {
            None => self.head.set(Some(node)),
            Some(mut current) => {
                while let Some(next) = current.next.get() {
                    current = next;
                }

                current.next.set(Some(node));
            }
        }
    }

    pub fn pop_front(&self) -> Option<&'a LinkedListNode<'a, T>> {
        match self.head.take() {
            None => None,
            Some(head) => {
                self.head.set(head.next.get());
                Some(head)
            }
        }
    }

    pub fn pop_back(&self) -> Option<&'a LinkedListNode<'a, T>> {
        match self.head.get() {
            None => None,
            Some(head) => {
                if let Some(mut current) = head.next.get() {
                    let mut previous = head;

                    while let Some(next) = current.next.get() {
                        previous = current;
                        current = next;
                    }

                    previous.next.take()
                } else {
                    self.head.take()
                }
            }
        }
    }

    pub fn remove(&self, node: &'a LinkedListNode<'a, T>) {
        if let Some(head) = self.head.get() {
            if core::ptr::eq(head, node) {
                self.head.set(node.next.get());
                return;
            }

            let mut current = head;

            while let Some(next) = current.next.get() {
                if core::ptr::eq(next, node) {
                    current.next.set(next.next.get());
                    return;
                }

                current = next;
            }
        }
    }

    pub fn contains(&self, node: &'a LinkedListNode<'a, T>) -> bool {
        for current in self.iter() {
            if current as *const LinkedListNode<'a, T> == node as *const LinkedListNode<'a, T> {
                return true;
            }
        }
        false
    }

    pub fn count(&self) -> usize {
        self.iter().count()
    }
}
