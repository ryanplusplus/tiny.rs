use super::Event;
use super::EventSubscription;
use core::cell::Cell;

#[test]
fn should() {
    let sum: Cell<u8> = Cell::new(0);

    let sub = EventSubscription::new(|x: &u8| {
        sum.replace(sum.get() + *x);
    });

    let mut event: Event<u8, _> = Event::new();

    event.subscribe(&sub);

    event.publish(&3);
    event.publish(&4);

    assert_eq!(7, sum.get());
}
