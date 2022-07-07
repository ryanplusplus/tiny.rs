use super::Fsm;
use super::FsmSignal;
use core::cell::Cell;

enum UserSignal {
    Signal1,
    Signal2,
}

#[test]
fn should_send_entry_to_the_initial_state() {
    fn state(obj: &Cell<bool>, signal: FsmSignal<UserSignal>) {
        match signal {
            FsmSignal::Entry => {
                obj.set(true);
            }
            _ => (),
        };
    }

    let succeeded = Cell::new(false);
    let _ = Fsm::new(&succeeded, state);

    assert!(succeeded.get());
}

#[test]
fn should_send_signals_to_the_initial_state_after_init() {
    fn state(obj: &Cell<bool>, signal: FsmSignal<UserSignal>) {
        match signal {
            FsmSignal::User(UserSignal::Signal1) => {
                obj.set(true);
            }
            _ => (),
        };
    }

    let succeeded = Cell::new(false);
    let fsm = Fsm::new(&succeeded, state);
    fsm.send_signal(UserSignal::Signal1);

    assert!(succeeded.get());
}

#[test]
fn should_send_exit_to_the_current_state_on_transition() {
    fn state_a(obj: &Cell<bool>, signal: FsmSignal<UserSignal>) {
        match signal {
            FsmSignal::Exit => {
                obj.set(true);
            }
            _ => (),
        };
    }

    fn state_b(_obj: &Cell<bool>, _signal: FsmSignal<UserSignal>) {}

    let succeeded = Cell::new(false);
    let fsm = Fsm::new(&succeeded, state_a);
    fsm.transition(state_b);

    assert!(succeeded.get());
}

#[test]
fn should_send_emtry_to_the_next_state_on_transition() {
    fn state_a(_obj: &Cell<bool>, _signal: FsmSignal<UserSignal>) {}

    fn state_b(obj: &Cell<bool>, signal: FsmSignal<UserSignal>) {
        match signal {
            FsmSignal::Entry => {
                obj.set(true);
            }
            _ => (),
        };
    }

    let succeeded = Cell::new(false);
    let fsm = Fsm::new(&succeeded, state_a);
    fsm.transition(state_b);

    assert!(succeeded.get());
}

#[test]
fn should_send_signals_to_the_new_state_after_a_transition() {
    fn state_a(_obj: &Cell<bool>, _signal: FsmSignal<UserSignal>) {}

    fn state_b(obj: &Cell<bool>, signal: FsmSignal<UserSignal>) {
        match signal {
            FsmSignal::User(UserSignal::Signal2) => {
                obj.set(true);
            }
            _ => (),
        };
    }

    let succeeded = Cell::new(false);
    let fsm = Fsm::new(&succeeded, state_a);
    fsm.transition(state_b);
    fsm.send_signal(UserSignal::Signal2);

    assert!(succeeded.get());
}
