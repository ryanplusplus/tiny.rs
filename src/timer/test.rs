use super::time_source;
use super::time_source::TimeSource;
use super::TimerGroup;
use core::cell::Cell;

struct FakeTimeSource {
    ticks: Cell<time_source::Ticks>,
}

impl FakeTimeSource {
    fn new() -> Self {
        Self {
            ticks: Cell::new(100),
        }
    }

    fn tick(&self, ticks: time_source::Ticks) {
        self.ticks.replace(self.ticks.get() + ticks);
    }
}

impl TimeSource for FakeTimeSource {
    fn ticks(&self) -> time_source::Ticks {
        return self.ticks.get();
    }
}

#[test]
fn should() {
    let time_source = FakeTimeSource::new();

    let foo: Cell<u8> = Cell::new(0);
    let bar: Cell<u8> = Cell::new(0);

    let timer_group = TimerGroup::new(&time_source);

    let foo_timer = timer_group.timer();
    let bar_timer = timer_group.timer();

    timer_group.start(foo_timer, 10, || {
        foo.replace(foo.get() + 1);
    });

    timer_group.start(bar_timer, 20, || {
        bar.replace(bar.get() + 1);
    });

    time_source.tick(9);
    timer_group.run();
    assert_eq!(0, foo.get());
    assert_eq!(0, bar.get());
    assert_eq!(true, timer_group.is_running(foo_timer));
    assert_eq!(true, timer_group.is_running(bar_timer));
    assert_eq!(Some(1), timer_group.remaining_ticks(foo_timer));
    assert_eq!(Some(11), timer_group.remaining_ticks(bar_timer));

    time_source.tick(1);
    timer_group.run();
    assert_eq!(1, foo.get());
    assert_eq!(0, bar.get());
    assert_eq!(false, timer_group.is_running(foo_timer));
    assert_eq!(true, timer_group.is_running(bar_timer));
    assert_eq!(None, timer_group.remaining_ticks(foo_timer));
    assert_eq!(Some(10), timer_group.remaining_ticks(bar_timer));

    time_source.tick(10);
    timer_group.run();
    assert_eq!(1, foo.get());
    assert_eq!(1, bar.get());
    assert_eq!(false, timer_group.is_running(foo_timer));
    assert_eq!(false, timer_group.is_running(bar_timer));
    assert_eq!(None, timer_group.remaining_ticks(foo_timer));
    assert_eq!(None, timer_group.remaining_ticks(bar_timer));
}
