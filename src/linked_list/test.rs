extern crate std;
use super::{LinkedList, LinkedListNode};
use std::vec;

#[test]
fn should_push_and_pop_front() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);

    list.push_front(&node1);
    list.push_front(&node2);

    assert_eq!(&42, list.pop_front().unwrap().value());
    assert_eq!(&21, list.pop_front().unwrap().value());
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

    assert_eq!(&21, list.pop_front().unwrap().value());
    assert_eq!(&42, list.pop_front().unwrap().value());
    assert_eq!(&84, list.pop_front().unwrap().value());
    assert!(list.pop_front().is_none());
}

#[test]
fn should_pop_back() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);

    list.push_back(&node1);
    list.push_back(&node2);

    assert_eq!(&42, list.pop_back().unwrap().value());
    assert_eq!(&21, list.pop_back().unwrap().value());
    assert!(list.pop_back().is_none());
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
fn should_for_each() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    list.push_back(&node1);
    list.push_back(&node2);

    let mut eached = vec![];

    list.for_each(|x| {
        eached.push(*x);
        true
    });

    assert_eq!(vec![21, 42], eached);
}

#[test]
fn should_abort_for_each_early() {
    let mut list = LinkedList::new();
    let node1: LinkedListNode<u8> = LinkedListNode::new(21);
    let node2: LinkedListNode<u8> = LinkedListNode::new(42);
    list.push_back(&node1);
    list.push_back(&node2);

    let mut eached = vec![];

    list.for_each(|x| {
        eached.push(*x);
        false
    });

    assert_eq!(vec![21], eached);
}
