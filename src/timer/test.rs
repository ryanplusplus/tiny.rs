use super::TimerGroup;
use core::cell::Cell;

#[test]
fn should() {
    let sum: Cell<u8> = Cell::new(0);

    let mut timer_group = TimerGroup::new();
    let timer = TimerGroup::new_timer();

    timer_group.start(&timer, || {
        sum.replace(4);
    });

    timer_group.run();

    assert_eq!(4, sum.get());
}
