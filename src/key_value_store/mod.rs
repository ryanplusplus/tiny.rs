use super::event::Event;
use core::cell::Cell;

pub trait KeyValueStore<'a> {
    fn read<T: SafelyDeserializable + Sized>(&self, key: Key) -> T;
    fn write<T: Sized>(&self, key: Key, value: &T);
    fn size_of(&self, key: Key) -> Size;
    fn on_change(&self) -> &Event<'a, Key>;
}

pub trait SafelyDeserializable {
    fn can_deserialize_from(bytes: &[Cell<u8>]) -> bool;
    fn size() -> u8;
}

impl SafelyDeserializable for u8 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size() -> u8 {
        1
    }
}

impl SafelyDeserializable for u16 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size() -> u8 {
        2
    }
}

impl SafelyDeserializable for u32 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size() -> u8 {
        4
    }
}

impl SafelyDeserializable for i8 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size() -> u8 {
        1
    }
}

impl SafelyDeserializable for i16 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size() -> u8 {
        2
    }
}

impl SafelyDeserializable for i32 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }

    fn size() -> u8 {
        4
    }
}

pub type Key = u16;
pub type Size = u8;
