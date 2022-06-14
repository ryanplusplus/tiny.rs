use super::Event;
use core::cell::Cell;

#[test]
fn should_do_nothing_whne_published_with_no_subscribers() {
    let event: Event<u8> = Event::new();
    event.publish(&3);
}

#[test]
fn should_publish_to_all_subscribers() {
    let sub1_data: Cell<Option<u8>> = Cell::new(None);
    let sub2_data: Cell<Option<u8>> = Cell::new(None);

    let sub1 = Event::new_subscription(&sub1_data, |data, x| {
        data.set(Some(*x));
    });
    let sub2 = Event::new_subscription(&sub2_data, |data, x| {
        data.set(Some(*x));
    });

    let event: Event<u8> = Event::new();

    event.subscribe(&sub1);
    event.subscribe(&sub2);

    event.publish(&3);

    assert_eq!(Some(3), sub1_data.get());
    assert_eq!(Some(3), sub2_data.get());
}

#[test]
fn should_not_publish_subscribers_that_have_unsubscribed() {
    let sub1_data: Cell<Option<u8>> = Cell::new(None);
    let sub2_data: Cell<Option<u8>> = Cell::new(None);

    let sub1 = Event::new_subscription(&sub1_data, |data, x| {
        data.set(Some(*x));
    });
    let sub2 = Event::new_subscription(&sub2_data, |data, x| {
        data.set(Some(*x));
    });

    let event: Event<u8> = Event::new();

    event.subscribe(&sub1);
    event.subscribe(&sub2);
    event.unsubscribe(&sub1);

    event.publish(&8);

    assert_eq!(None, sub1_data.get());
    assert_eq!(Some(8), sub2_data.get());
}

#[test]
fn should_allow_subscribers_to_resubscribe() {
    let sub1_data: Cell<u8> = Cell::new(0);
    let sub2_data: Cell<u8> = Cell::new(0);

    let sub1 = Event::new_subscription(&sub1_data, |data, x| {
        data.set(data.get() + *x);
    });
    let sub2 = Event::new_subscription(&sub2_data, |data, x| {
        data.set(data.get() + *x);
    });

    let event: Event<u8> = Event::new();

    event.subscribe(&sub1);
    event.subscribe(&sub2);
    event.subscribe(&sub1);

    event.publish(&1);

    assert_eq!(1, sub1_data.get());
    assert_eq!(1, sub2_data.get());
}
