use core::cell::Cell;

pub trait KeyValueStore {
    fn read<T: SafelyDeserializable + Sized>(&self, key: Key) -> T;
    fn write<T: Sized>(&self, key: Key, value: &T);
    fn size_of(&self, key: Key) -> Size;
}

pub trait SafelyDeserializable {
    fn can_deserialize_from(bytes: &[Cell<u8>]) -> bool;
}

impl SafelyDeserializable for u8 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }
}

impl SafelyDeserializable for u16 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }
}

impl SafelyDeserializable for u32 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }
}

impl SafelyDeserializable for i8 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }
}

impl SafelyDeserializable for i16 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }
}

impl SafelyDeserializable for i32 {
    fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
        true
    }
}

pub type Key = u16;
pub type Size = u8;
