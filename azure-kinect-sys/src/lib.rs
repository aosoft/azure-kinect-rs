pub mod k4a;
pub mod k4arecord;

pub mod loader;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Failed,
    Win32Error(u32),
}
