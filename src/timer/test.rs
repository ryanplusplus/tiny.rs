use super::{Timer, TimerGroup};
use core::cell::Cell;

#[test]
fn should() {
    let foo: Cell<u8> = Cell::new(0);
    let bar: Cell<u8> = Cell::new(0);

    let mut timer_group = TimerGroup::new();

    let timer1 = timer_group.new_timer();
    let timer2 = timer_group.new_timer();

    timer_group.start(timer1, 10, || {
        foo.replace(4);
    });

    timer_group.start(timer2, 20, || {
        bar.replace(5);
    });

    timer_group.run();

    assert_eq!(4, foo.get());
    assert_eq!(5, bar.get());
}
