use super::event::Event;
use core::cell::Cell;

pub trait KeyValueStore<'a> {
    fn read(&self, key: Key, value: &mut dyn Storable);
    fn write(&self, key: Key, value: &dyn Storable);
    fn size_of(&self, key: Key) -> Size;
    fn on_change(&self) -> &Event<'a, Key>;
}

pub type Key = u16;
pub type Size = u8;

pub trait Storable {
    fn can_deserialize_from(&self, bytes: &[Cell<u8>]) -> bool;
    fn size(&self) -> u8 {
        core::mem::size_of_val(self) as u8
    }
}

#[doc(hidden)]
macro_rules! __impl_storable {
    ($typ:ty) => {
        impl Storable for $typ {
            fn can_deserialize_from(&self, _: &[Cell<u8>]) -> bool {
                true
            }
        }
    };
}

__impl_storable!(u8);
__impl_storable!(u16);
__impl_storable!(u32);
__impl_storable!(u64);
__impl_storable!(u128);

__impl_storable!(i8);
__impl_storable!(i16);
__impl_storable!(i32);
__impl_storable!(i64);
__impl_storable!(i128);

__impl_storable!(usize);
__impl_storable!(isize);

impl Storable for bool {
    fn can_deserialize_from(&self, bytes: &[Cell<u8>]) -> bool {
        bytes[0].get() == (false as u8) || bytes[0].get() == (true as u8)
    }
}
