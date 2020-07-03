use super::Event;
use core::cell::Cell;

#[test]
fn should() {
    let foo: Cell<u8> = Cell::new(0);
    let bar: Cell<u8> = Cell::new(0);

    let mut event: Event<u8> = Event::new();

    event.subscribe(|x: &u8| {
        foo.replace(foo.get() + *x);
    });

    event.subscribe(|x: &u8| {
        bar.replace(*x);
    });

    event.publish(&3);
    event.publish(&4);

    assert_eq!(7, foo.get());
    assert_eq!(4, bar.get());
}
