use super::event::Event;
use core::cell::Cell;

pub trait KeyValueStore<'a> {
    fn read(&self, key: Key, value: &mut dyn Storable);
    fn write(&self, key: Key, value: &dyn Storable);
    fn size_of(&self, key: Key) -> Size;
    fn on_change(&self) -> &Event<'a, Key>;
}

pub trait Storable {
    fn can_deserialize_from(&self, bytes: &[Cell<u8>]) -> bool;
    fn size(&self) -> u8;
}

impl Storable for u8 {
    fn can_deserialize_from(&self, _bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size(&self) -> u8 {
        1
    }
}

impl Storable for u16 {
    fn can_deserialize_from(&self, _bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size(&self) -> u8 {
        2
    }
}

impl Storable for u32 {
    fn can_deserialize_from(&self, _bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size(&self) -> u8 {
        4
    }
}

impl Storable for i8 {
    fn can_deserialize_from(&self, _bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size(&self) -> u8 {
        1
    }
}

impl Storable for i16 {
    fn can_deserialize_from(&self, _bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size(&self) -> u8 {
        2
    }
}

impl Storable for i32 {
    fn can_deserialize_from(&self, _bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size(&self) -> u8 {
        4
    }
}

pub type Key = u16;
pub type Size = u8;
