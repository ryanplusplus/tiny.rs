use super::super::time_source::TimeSource;
use super::{Ticks, TimerGroup};
use core::cell::Cell;

struct FakeTimeSource {
    ticks: Cell<Ticks>,
}

impl FakeTimeSource {
    fn new(initial_ticks: Ticks) -> Self {
        Self {
            ticks: Cell::new(initial_ticks),
        }
    }

    fn tick(&self, ticks: Ticks) {
        self.ticks.set(self.ticks.get() + ticks);
    }
}

impl TimeSource for FakeTimeSource {
    fn ticks(&self) -> Ticks {
        self.ticks.get()
    }
}

#[test]
fn should_run_a_single_timer() {
    let time_source = FakeTimeSource::new(1234);
    let ran = Cell::new(false);

    let timer_group = TimerGroup::new(&time_source);
    let timer = TimerGroup::new_timer();

    timer_group.start(&timer, 11, &ran, |ran, _| {
        ran.set(true);
    });

    timer_group.run();
    assert!(!ran.get());
    assert_eq!(11, timer_group.remaining_ticks(&timer));

    time_source.tick(10);
    timer_group.run();
    assert!(!ran.get());
    assert_eq!(1, timer_group.remaining_ticks(&timer));

    time_source.tick(1);
    timer_group.run();
    assert!(ran.get());
    assert_eq!(0, timer_group.remaining_ticks(&timer));
}

#[test]
fn should_run_multiple_timers() {
    let time_source = FakeTimeSource::new(1234);
    let ran1 = Cell::new(false);
    let ran2 = Cell::new(false);

    let timer_group = TimerGroup::new(&time_source);
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();

    timer_group.start(&timer1, 5, &ran1, |ran, _| {
        ran.set(true);
    });

    timer_group.start(&timer2, 11, &ran2, |ran, _| {
        ran.set(true);
    });

    timer_group.run();
    assert!(!ran1.get());
    assert!(!ran2.get());
    assert_eq!(5, timer_group.remaining_ticks(&timer1));
    assert_eq!(11, timer_group.remaining_ticks(&timer2));

    time_source.tick(4);
    timer_group.run();
    assert!(!ran1.get());
    assert!(!ran2.get());
    assert_eq!(1, timer_group.remaining_ticks(&timer1));
    assert_eq!(7, timer_group.remaining_ticks(&timer2));

    time_source.tick(1);
    timer_group.run();
    assert!(ran1.get());
    assert!(!ran2.get());
    assert_eq!(0, timer_group.remaining_ticks(&timer1));
    assert_eq!(6, timer_group.remaining_ticks(&timer2));

    time_source.tick(5);
    timer_group.run();
    assert!(ran1.get());
    assert!(!ran2.get());
    assert_eq!(0, timer_group.remaining_ticks(&timer1));
    assert_eq!(1, timer_group.remaining_ticks(&timer2));

    time_source.tick(1);
    timer_group.run();
    assert!(ran1.get());
    assert!(ran2.get());
    assert_eq!(0, timer_group.remaining_ticks(&timer1));
    assert_eq!(0, timer_group.remaining_ticks(&timer2));
}

#[test]
fn should_remove_timers_that_have_expired() {
    let time_source = FakeTimeSource::new(1234);
    let run_count = Cell::new(0u8);

    let timer_group = TimerGroup::new(&time_source);
    let timer = TimerGroup::new_timer();

    timer_group.start(&timer, 5, &run_count, |run_count, _| {
        run_count.set(run_count.get() + 1);
    });

    time_source.tick(5);
    timer_group.run();
    assert_eq!(1, run_count.get());

    time_source.tick(5);
    timer_group.run();
    assert_eq!(1, run_count.get());
}

#[test]
fn should_run_periodic_timers() {
    let time_source = FakeTimeSource::new(1234);
    let run_count = Cell::new(0u8);

    let timer_group = TimerGroup::new(&time_source);
    let timer = TimerGroup::new_timer();

    timer_group.start_periodic(&timer, 5, &run_count, |run_count, _| {
        run_count.set(run_count.get() + 1);
    });

    time_source.tick(5);
    timer_group.run();
    assert_eq!(1, run_count.get());

    time_source.tick(5);
    timer_group.run();
    assert_eq!(2, run_count.get());
}

#[test]
fn should_allow_periodic_timers_to_be_stopped_from_callbacks() {
    let time_source = FakeTimeSource::new(1234);
    let run_count = Cell::new(0u8);

    let timer_group = TimerGroup::new(&time_source);
    let timer = TimerGroup::new_timer();

    let context = (&timer_group, &timer, &run_count);

    timer_group.start_periodic(&timer, 5, &context, |context, _| {
        context.2.set(context.2.get() + 1);
        context.0.stop(context.1);
    });

    time_source.tick(5);
    timer_group.run();
    assert_eq!(1, run_count.get());

    time_source.tick(5);
    timer_group.run();
    assert_eq!(1, run_count.get());
}

#[test]
fn should_allow_timers_to_be_restarted_without_being_stopped() {
    let time_source = FakeTimeSource::new(1234);
    let run_count = Cell::new(0u8);

    let timer_group = TimerGroup::new(&time_source);
    let timer = TimerGroup::new_timer();

    timer_group.start(&timer, 5, &run_count, |run_count, _| {
        run_count.set(run_count.get() + 1);
    });

    timer_group.start(&timer, 3, &run_count, |run_count, _| {
        run_count.set(run_count.get() + 1);
    });

    time_source.tick(3);
    timer_group.run();
    assert_eq!(1, run_count.get());
}

#[test]
fn should_call_back_one_timer_per_run() {
    let time_source = FakeTimeSource::new(1234);
    let ran1 = Cell::new(false);
    let ran2 = Cell::new(false);

    let timer_group = TimerGroup::new(&time_source);
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();

    timer_group.start(&timer1, 5, &ran1, |ran, _| {
        ran.set(true);
    });

    timer_group.start(&timer2, 5, &ran2, |ran, _| {
        ran.set(true);
    });

    time_source.tick(5);
    timer_group.run();
    assert!(ran1.get());
    assert!(!ran2.get());
}

#[test]
fn should_not_allow_starvation() {
    let time_source = FakeTimeSource::new(1234);
    let run_count1 = Cell::new(0u8);
    let run_count2 = Cell::new(0u8);

    let timer_group = TimerGroup::new(&time_source);
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();

    timer_group.start(&timer1, 0, &run_count1, |run_count, _| {
        run_count.set(run_count.get() + 1);
    });

    timer_group.start(&timer2, 0, &run_count2, |run_count, _| {
        run_count.set(run_count.get() + 1);
    });

    timer_group.run();
    assert_eq!(1, run_count1.get());
    assert_eq!(0, run_count2.get());

    timer_group.run();
    assert_eq!(1, run_count1.get());
    assert_eq!(1, run_count2.get());
}

#[test]
fn should_indicate_whether_a_timer_is_running() {
    let time_source = FakeTimeSource::new(1234);

    let timer_group = TimerGroup::new(&time_source);
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();

    timer_group.start(&timer1, 5, &0, |_, _| {});
    timer_group.start(&timer2, 7, &0, |_, _| {});
    assert!(timer_group.running(&timer1));
    assert!(timer_group.running(&timer2));

    time_source.tick(5);
    timer_group.run();
    assert!(!timer_group.running(&timer1));
    assert!(timer_group.running(&timer2));

    time_source.tick(2);
    timer_group.run();
    assert!(!timer_group.running(&timer1));
    assert!(!timer_group.running(&timer2));
}

#[test]
fn should_indicate_when_the_next_timer_will_be_ready() {
    let time_source = FakeTimeSource::new(1234);

    let timer_group = TimerGroup::new(&time_source);
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();
    let timer3 = TimerGroup::new_timer();

    timer_group.start(&timer1, 5, &0, |_, _| {});
    timer_group.start(&timer2, 7, &0, |_, _| {});
    timer_group.start(&timer3, 5, &0, |_, _| {});

    assert_eq!(5, timer_group.run());

    time_source.tick(4);
    assert_eq!(1, timer_group.run());

    time_source.tick(1);
    assert_eq!(0, timer_group.run());
    assert_eq!(2, timer_group.run());

    time_source.tick(2);
    assert_eq!(Ticks::max_value(), timer_group.run());
}

#[test]
fn should_consider_periodic_timers_when_giving_time_until_next_ready() {
    let time_source = FakeTimeSource::new(1234);

    let timer_group = TimerGroup::new(&time_source);
    let timer1 = TimerGroup::new_timer();
    let timer2 = TimerGroup::new_timer();

    timer_group.start_periodic(&timer1, 2, &0, |_, _| {});
    timer_group.start(&timer2, 7, &0, |_, _| {});

    time_source.tick(2);
    assert_eq!(2, timer_group.run());
}

#[test]
fn should_consider_restarted_timers_when_giving_time_until_next_ready() {
    let time_source = FakeTimeSource::new(1234);

    let timer_group = TimerGroup::new(&time_source);
    let timer = TimerGroup::new_timer();

    timer_group.start(&timer, 2, &timer, |timer, timer_group| {
        timer_group.start(&timer, 5, &0, |_, _| {});
    });

    time_source.tick(2);
    assert_eq!(5, timer_group.run());
}
