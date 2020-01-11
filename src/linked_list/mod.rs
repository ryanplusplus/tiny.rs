use core::cell::UnsafeCell;

pub struct LinkedListNode<'a, T> {
    next: UnsafeCell<Option<&'a LinkedListNode<'a, T>>>,
    value: T,
}

pub struct LinkedList<'a, T> {
    head: Option<&'a LinkedListNode<'a, T>>,
}

impl<T> LinkedListNode<'_, T> {
    pub const fn new(value: T) -> Self {
        Self {
            next: UnsafeCell::new(None),
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
        match self.head {
            None => self.head = Some(node),
            Some(head) => unsafe {
                *node.next.get() = Some(head);
                self.head = Some(node);
            },
        };
    }

    pub fn push_back(&mut self, node: &'a LinkedListNode<'a, T>) {
        match self.head {
            None => self.head = Some(node),
            Some(head) => unsafe {
                let mut current = Some(head);

                while (*(&current.unwrap().next).get()).is_some() {
                    current = *(&current.unwrap().next).get();
                }

                *current.unwrap().next.get() = Some(node);
            },
        };
    }

    pub fn pop_front(&mut self) -> Option<&'a LinkedListNode<'a, T>> {
        let popped = self.head;

        match self.head {
            None => (),
            Some(head) => unsafe {
                self.head = *head.next.get();
            },
        };

        popped
    }

    pub fn pop_back(&mut self) -> Option<&'a LinkedListNode<'a, T>> {
        match self.head {
            None => None,
            Some(head) => unsafe {
                if (*head.next.get()).is_some() {
                    let mut previous = None;
                    let mut current = Some(head);

                    while (*(&current.unwrap().next).get()).is_some() {
                        previous = current;
                        current = *(&current.unwrap().next).get();
                    }

                    *previous.unwrap().next.get() = None;
                    current
                } else {
                    let popped = self.head;
                    self.head = None;
                    popped
                }
            },
        }
    }

    pub fn count(&self) -> usize {
        match self.head {
            None => 0,
            Some(head) => unsafe {
                let mut current = Some(head);
                let mut count = 1;

                while (*(&current.unwrap().next).get()).is_some() {
                    count += 1;
                    current = *(&current.unwrap().next).get();
                }

                count
            },
        }
    }
}

#[cfg(test)]
mod test;
