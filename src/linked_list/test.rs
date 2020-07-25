extern crate std;
use super::{LinkedList, LinkedListNode};
use core::ops::Deref;

#[test]
fn should_push_and_pop_front() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);

    list.push_front(&node1);
    list.push_front(&node2);

    assert_eq!(&42, list.pop_front().unwrap().deref());
    assert_eq!(&21, list.pop_front().unwrap().deref());
    assert!(list.pop_front().is_none());
}

#[test]
fn should_push_back() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    let node3: LinkedListNode<u8> = LinkedListNode::new(84);

    list.push_back(&node1);
    list.push_back(&node2);
    list.push_back(&node3);

    assert_eq!(&21, list.pop_front().unwrap().deref());
    assert_eq!(&42, list.pop_front().unwrap().deref());
    assert_eq!(&84, list.pop_front().unwrap().deref());
    assert!(list.pop_front().is_none());
}

#[test]
fn should_pop_back() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);

    list.push_back(&node1);
    list.push_back(&node2);

    assert_eq!(&42, list.pop_back().unwrap().deref());
    assert_eq!(&21, list.pop_back().unwrap().deref());
    assert!(list.pop_back().is_none());
}

#[test]
fn should_remove_from_front() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    let node3: LinkedListNode<u8> = LinkedListNode::new(63);

    list.push_back(&node1);
    list.push_back(&node2);
    list.push_back(&node3);

    list.remove(&node1);

    assert_eq!(&42, list.pop_front().unwrap().deref());
    assert_eq!(&63, list.pop_front().unwrap().deref());
    assert!(list.pop_front().is_none());
}

#[test]
fn should_remove_from_middle() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    let node3: LinkedListNode<u8> = LinkedListNode::new(63);

    list.push_back(&node1);
    list.push_back(&node2);
    list.push_back(&node3);

    list.remove(&node2);

    assert_eq!(&21, list.pop_front().unwrap().deref());
    assert_eq!(&63, list.pop_front().unwrap().deref());
    assert!(list.pop_front().is_none());
}

#[test]
fn should_remove_from_back() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    let node3: LinkedListNode<u8> = LinkedListNode::new(63);

    list.push_back(&node1);
    list.push_back(&node2);
    list.push_back(&node3);

    list.remove(&node3);

    assert_eq!(&21, list.pop_front().unwrap().deref());
    assert_eq!(&42, list.pop_front().unwrap().deref());
    assert!(list.pop_front().is_none());
}

#[test]
fn should_count() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);

    assert_eq!(0, list.count());

    list.push_back(&node1);
    assert_eq!(1, list.count());

    list.push_front(&node2);
    assert_eq!(2, list.count());

    list.pop_back();
    assert_eq!(1, list.count());

    list.pop_front();
    assert_eq!(0, list.count());
}

#[test]
fn should_be_iterable() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    list.push_back(&node1);
    list.push_back(&node2);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&21));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), None);
}

#[test]
fn should_be_iterable_twice() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    list.push_back(&node1);
    list.push_back(&node2);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&21));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), None);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&21));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), None);
}
