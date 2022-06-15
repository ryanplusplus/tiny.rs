use super::event::Event;
use super::key_value_store::Key;
use super::key_value_store::KeyValueStore;
use super::key_value_store::Size;
use super::key_value_store::Storable;
use core::cell::Cell;
use core::mem;

#[cfg(test)]
mod test;

pub struct RamKeyValueStoreElement {
    pub key: Key,
    pub size: Size,
}

impl RamKeyValueStoreElement {
    pub fn new<T: Sized>(key: Key) -> Self {
        Self {
            key: key,
            size: mem::size_of::<T>() as u8,
        }
    }
}

pub struct RamKeyValueStore<'a> {
    ram: &'a mut [Cell<u8>],
    elements: &'a [RamKeyValueStoreElement],
    on_change_event: Event<'a, Key>,
}

impl<'a> RamKeyValueStore<'a> {
    pub fn new(ram: &'a mut [Cell<u8>], elements: &'a [RamKeyValueStoreElement]) -> Self {
        let required_size: usize = elements.iter().map(|x| x.size as usize).sum();
        assert!(ram.len() == required_size, "Incorrect RAM size");

        Self {
            ram,
            elements,
            on_change_event: Event::<'a, Key>::new(),
        }
    }
}

impl<'a> KeyValueStore<'a> for RamKeyValueStore<'a> {
    fn read(&self, key: Key, value: &mut dyn Storable) {
        let mut offset = 0 as usize;

        for element in self.elements.iter() {
            if key == element.key {
                assert!(value.size() == element.size, "Invalid size");
                assert!(
                    value.can_deserialize_from(&self.ram[offset..offset + element.size as usize]),
                    "Unable to safely deserialize"
                );

                let dst_raw_pointer = value as *mut dyn Storable as *mut u8;
                let src_raw_pointer = self.ram[offset..].as_ptr() as *const u8;

                unsafe {
                    core::ptr::copy_nonoverlapping(
                        src_raw_pointer,
                        dst_raw_pointer,
                        element.size as usize,
                    );
                };

                return;
            } else {
                offset += element.size as usize;
            }
        }

        panic!("Invalid key");
    }

    fn write(&self, key: Key, value: &dyn Storable) {
        let mut offset = 0 as usize;

        for element in self.elements.iter() {
            if key == element.key {
                assert!(value.size() == element.size, "Invalid size");

                let value_slice = unsafe {
                    core::slice::from_raw_parts(
                        value as *const dyn Storable as *const Cell<u8>,
                        element.size as usize,
                    )
                };

                if value_slice != &self.ram[offset..offset + element.size as usize] {
                    self.on_change_event.publish(&element.key);
                }

                let src_raw_pointer = value as *const dyn Storable as *const u8;
                let dst_raw_pointer = self.ram[offset..].as_ptr() as *mut u8;

                unsafe {
                    core::ptr::copy_nonoverlapping(
                        src_raw_pointer,
                        dst_raw_pointer,
                        element.size as usize,
                    );
                };

                return;
            } else {
                offset += element.size as usize;
            }
        }

        panic!("Invalid key");
    }

    fn size_of(&self, key: Key) -> Size {
        for element in self.elements.iter() {
            if key == element.key {
                return element.size;
            }
        }

        panic!("Invalid key");
    }

    fn on_change(&self) -> &Event<'a, Key> {
        &self.on_change_event
    }
}
