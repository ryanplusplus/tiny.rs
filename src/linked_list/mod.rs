use core::cell::Cell;

mod iter;

#[cfg(test)]
mod test;

pub struct LinkedListNode<'a, T> {
    next: Cell<Option<&'a LinkedListNode<'a, T>>>,
    pub value: T,
}

pub struct LinkedList<'a, T> {
    head: Option<&'a LinkedListNode<'a, T>>,
}

impl<T> LinkedListNode<'_, T> {
    pub const fn new(value: T) -> Self {
        Self {
            next: Cell::new(None),
            value,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub const fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, node: &'a LinkedListNode<'a, T>) {
        if let Some(head) = self.head {
            node.next.replace(Some(head));
        }
        self.head = Some(node);
    }

    pub fn push_back(&mut self, node: &'a LinkedListNode<'a, T>) {
        match self.head {
            None => self.head = Some(node),
            Some(mut current) => {
                while let Some(next) = current.next.get() {
                    current = next;
                }

                current.next.replace(Some(node));
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<&'a LinkedListNode<'a, T>> {
        let popped = self.head;

        if let Some(head) = self.head {
            self.head = head.next.get();
        }

        popped
    }

    pub fn pop_back(&mut self) -> Option<&'a LinkedListNode<'a, T>> {
        match self.head {
            None => None,
            Some(head) => {
                if let Some(mut current) = head.next.get() {
                    let mut previous = head;

                    while let Some(next) = current.next.get() {
                        previous = current;
                        current = next;
                    }

                    previous.next.replace(None)
                } else {
                    self.head.take()
                }
            }
        }
    }

    pub fn remove(&mut self, node: &'a LinkedListNode<'a, T>) {
        if let Some(head) = self.head {
            if core::ptr::eq(head, node) {
                self.head = node.next.get();
                return;
            }

            let mut current = head;

            while let Some(next) = current.next.get() {
                if core::ptr::eq(next, node) {
                    current.next.replace(next.next.get());
                    return;
                }

                current = next;
            }
        }
    }

    pub fn count(&self) -> usize {
        match self.head {
            None => 0,
            Some(head) => {
                let mut current = head;
                let mut count = 1;

                while let Some(next) = current.next.get() {
                    count += 1;
                    current = next;
                }

                count
            }
        }
    }
}
