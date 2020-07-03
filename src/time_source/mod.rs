pub type Ticks = u32;

pub trait TimeSource {
    fn ticks(&self) -> Ticks;
}
