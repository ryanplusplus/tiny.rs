use super::Event;
use core::cell::Cell;

#[test]
fn should() {
    let sum: Cell<u8> = Cell::new(0);

    let mut event: Event<u8, _> = Event::new();

    event.subscribe(|x| {
        sum.replace(sum.get() + *x);
    });

    event.publish(&3);
    event.publish(&4);

    assert_eq!(7, sum.get());
}
