use super::Event;
use core::cell::Cell;

#[test]
fn should() {
    let sum1: Cell<u8> = Cell::new(0);
    let sum2: Cell<u16> = Cell::new(3);

    let sub1 = Event::new_subscription(&sum1, |sum: &Cell<u8>, x: &u8| {
        sum.replace(sum.get() + *x);
    });

    let sub2 = Event::new_subscription(&sum2, |sum: &Cell<u16>, x: &u8| {
        sum.replace(sum.get() + *x as u16);
    });

    let mut event: Event<u8> = Event::new();

    event.subscribe(&sub1);
    event.subscribe(&sub2);

    event.publish(&3);
    event.publish(&4);

    assert_eq!(7, sum1.get());
    assert_eq!(10, sum2.get());
}
