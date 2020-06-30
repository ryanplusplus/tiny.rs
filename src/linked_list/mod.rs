use core::cell::Cell;

pub struct LinkedListNode<'a, T> {
    next: Cell<Option<&'a LinkedListNode<'a, T>>>,
    value: T,
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
        match self.head {
            None => self.head = Some(node),
            Some(head) => {
                node.next.replace(Some(head));
                self.head = Some(node);
            }
        };
    }

    pub fn push_back(&mut self, node: &'a LinkedListNode<'a, T>) {
        match self.head {
            None => self.head = Some(node),
            Some(head) => {
                let mut current = head;

                while let Some(next) = head.next.get() {
                    current = next;
                }

                current.next.replace(Some(node));
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<&'a LinkedListNode<'a, T>> {
        let popped = self.head;

        match self.head {
            None => (),
            Some(head) => {
                self.head = head.next.get();
            }
        };

        popped
    }

    pub fn pop_back(&mut self) -> Option<&'a LinkedListNode<'a, T>> {
        match self.head {
            None => None,
            Some(head) => {
                if let Some(head_next) = head.next.get() {
                    let mut previous = head;
                    let mut current = head_next;

                    while let Some(next) = current.next.get() {
                        previous = current;
                        current = next;
                    }

                    previous.next.replace(None);
                    Some(current)
                } else {
                    let popped = self.head;
                    self.head = None;
                    popped
                }
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

    pub fn for_each<F: FnMut(&T) -> bool>(&self, mut f: F) {
        match self.head {
            None => return,
            Some(head) => {
                let mut current = head;

                loop {
                    if !f(current.value()) {
                        break;
                    }

                    if let Some(next) = current.next.get() {
                        current = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test;
