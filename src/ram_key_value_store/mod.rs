use super::key_value_store::Key;
use super::key_value_store::KeyValueStore;
use super::key_value_store::Size;
use core::cell::Cell;
use core::mem;
use core::mem::MaybeUninit;

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
}

impl<'a> RamKeyValueStore<'a> {
    pub fn new(ram: &'a mut [Cell<u8>], elements: &'a [RamKeyValueStoreElement]) -> Self {
        let required_size: usize = elements.iter().map(|x| x.size as usize).sum();
        assert!(ram.len() == required_size, "RAM size doesn't match");

        Self { ram, elements }
    }
}

impl<'a> KeyValueStore for RamKeyValueStore<'a> {
    fn read<T: crate::key_value_store::SafelyDeserializable + Sized>(&self, key: Key) -> T {
        let mut offset = 0 as usize;

        for element in self.elements.iter() {
            if key == element.key {
                assert!(mem::size_of::<T>() == element.size as usize, "Invalid size");
                assert!(
                    <T>::can_deserialize_from(&self.ram[offset..]),
                    "Invalid contents"
                );

                unsafe {
                    let mut value = MaybeUninit::<T>::zeroed();

                    for i in 0..element.size {
                        let byte = self.ram[offset + i as usize].get();
                        let raw_pointer = value.as_mut_ptr() as *mut u8;
                        *raw_pointer.offset(i as isize) = byte;
                    }

                    return value.assume_init();
                };
            } else {
                offset += element.size as usize;
            }
        }

        panic!("Invalid key");
    }

    fn write<T: Sized>(&self, key: Key, value: &T) {
        let mut offset = 0 as usize;

        for element in self.elements.iter() {
            if key == element.key {
                assert!(mem::size_of::<T>() == element.size as usize, "Invalid size");

                for i in 0..element.size {
                    let byte = unsafe {
                        let raw_pointer = value as *const T as *const u8;
                        *raw_pointer.offset(i as isize)
                    };
                    self.ram[offset + i as usize].set(byte);
                }

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
}
