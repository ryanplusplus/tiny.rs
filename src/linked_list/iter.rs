use super::{LinkedList, LinkedListNode};
use core::ops::Deref;

pub struct LinkedListIter<'node, T> {
    current: Option<&'node LinkedListNode<'node, T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.current;
        self.current = self.current.and_then(|node| node.next.get());
        old.map(Deref::deref)
    }
}

impl<'a, T> IntoIterator for LinkedList<'a, T> {
    type Item = &'a T;
    type IntoIter = LinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter { current: self.head }
    }
}

impl<'a, T> LinkedList<'a, T> {
    pub fn iter(&self) -> LinkedListIter<'a, T> {
        LinkedListIter { current: self.head }
    }
}
