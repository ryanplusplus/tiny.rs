use super::TimerGroup;
use core::cell::Cell;

#[test]
fn should() {
    let sum1: Cell<u8> = Cell::new(0);
    let sum2: Cell<u16> = Cell::new(0);

    let mut timer_group = TimerGroup::new();
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();

    timer_group.start(&timer1, &sum1, |sum| {
        sum.replace(4);
    });

    timer_group.start(&timer2, &sum2, |sum| {
        sum.replace(5);
    });

    timer_group.run();

    assert_eq!(4, sum1.get());
    assert_eq!(5, sum2.get());
}
