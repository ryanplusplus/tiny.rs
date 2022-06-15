extern crate std;
use super::{RamKeyValueStore, RamKeyValueStoreElement};
use crate::{
    event::Event,
    key_value_store::{Key, KeyValueStore, SafelyDeserializable},
};
use core::cell::Cell;

#[test]
#[should_panic = "Incorrect RAM size"]
fn should_require_the_ram_to_match_the_total_value_size() {
    let mut ram: [Cell<u8>; 7] = Default::default();
    let elements = [
        RamKeyValueStoreElement::new::<i16>(4),
        RamKeyValueStoreElement::new::<u32>(7),
    ];
    RamKeyValueStore::new(&mut ram, &elements);
}

#[test]
#[should_panic = "Invalid size"]
fn should_require_the_correct_size_when_reading() {
    let mut ram: [Cell<u8>; 2] = Default::default();
    let elements = [RamKeyValueStoreElement::new::<i16>(4)];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    kvs.read::<u32>(4);
}

#[test]
#[should_panic = "Invalid size"]
fn should_require_the_correct_size_when_writing() {
    let mut ram: [Cell<u8>; 2] = Default::default();
    let elements = [RamKeyValueStoreElement::new::<i16>(4)];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    kvs.write::<u32>(4, &0x1234);
}

#[test]
fn should_allow_element_sizes_to_be_queried() {
    let mut ram: [Cell<u8>; 6] = Default::default();
    let elements = [
        RamKeyValueStoreElement::new::<i16>(4),
        RamKeyValueStoreElement::new::<u32>(7),
    ];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    assert_eq!(2, kvs.size_of(4));
    assert_eq!(4, kvs.size_of(7));
}

#[test]
fn should_allow_values_to_be_read_and_written() {
    let mut ram: [Cell<u8>; 6] = Default::default();
    let elements = [
        RamKeyValueStoreElement::new::<i16>(4),
        RamKeyValueStoreElement::new::<u32>(7),
    ];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    kvs.write::<i16>(4, &-1234);
    kvs.write::<u32>(7, &0x87654321);

    assert_eq!(-1234, kvs.read::<i16>(4));
    assert_eq!(0x87654321, kvs.read::<u32>(7));
}

#[test]
#[should_panic = "Unable to safely deserialize"]
fn should_require_the_destination_type_to_be_safely_deserializable_when_reading() {
    struct SomeType {
        _value: u16,
    }

    impl SafelyDeserializable for SomeType {
        fn can_deserialize_from(_bytes: &[Cell<u8>]) -> bool {
            false
        }

        fn size() -> u8 {
            2
        }
    }

    let mut ram: [Cell<u8>; 2] = Default::default();
    let elements = [RamKeyValueStoreElement::new::<SomeType>(4)];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    kvs.read::<SomeType>(4);
}

#[test]
fn should_publish_on_change_event_when_new_data_is_written() {
    let mut ram: [Cell<u8>; 4] = Default::default();
    let elements = [
        RamKeyValueStoreElement::new::<u16>(4),
        RamKeyValueStoreElement::new::<u16>(7),
    ];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    let publication_data: Cell<Option<Key>> = Cell::new(None);
    let subscription = Event::new_subscription(&publication_data, |data, key| {
        data.set(Some(*key));
    });

    kvs.on_change().subscribe(&subscription);
    kvs.write::<u16>(4, &0x1234);

    assert_eq!(Some(4), publication_data.get());
}

#[test]
fn should_not_publish_on_change_event_when_the_same_data_is_written() {
    let mut ram: [Cell<u8>; 4] = Default::default();
    let elements = [
        RamKeyValueStoreElement::new::<u16>(4),
        RamKeyValueStoreElement::new::<u16>(7),
    ];
    let kvs = RamKeyValueStore::new(&mut ram, &elements);

    let publication_data: Cell<Option<Key>> = Cell::new(None);
    let subscription = Event::new_subscription(&publication_data, |data, key| {
        data.set(Some(*key));
    });

    kvs.write::<u16>(4, &0x1234);
    kvs.on_change().subscribe(&subscription);
    kvs.write::<u16>(4, &0x1234);

    assert_eq!(None, publication_data.get());
}
