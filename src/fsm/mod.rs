use core::cell::Cell;

#[cfg(test)]
mod test;

pub enum FsmSignal<UserSignal> {
    Entry,
    Exit,
    User(UserSignal),
}

pub type FsmState<Type, UserSignal> = fn(obj: &Type, signal: FsmSignal<UserSignal>) -> ();

pub struct Fsm<'a, Type, UserSignal> {
    obj: &'a Type,
    current: Cell<FsmState<Type, UserSignal>>,
}

impl<'a, Type, UserSignal> Fsm<'a, Type, UserSignal> {
    pub fn new(obj: &'a Type, initial: FsmState<Type, UserSignal>) -> Self {
        initial(obj, FsmSignal::Entry);

        Self {
            obj,
            current: Cell::new(initial),
        }
    }

    pub fn send_signal(&self, signal: UserSignal) {
        self.current.get()(self.obj, FsmSignal::User(signal));
    }

    pub fn transition(&self, next: FsmState<Type, UserSignal>) {
        self.current.get()(self.obj, FsmSignal::Exit);
        self.current.set(next);
        self.current.get()(self.obj, FsmSignal::Entry);
    }
}
