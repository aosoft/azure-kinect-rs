use super::bindings::*;
use super::factory::Factory;
use std::ptr;

pub struct Capture<'a> {
    factory: &'a Factory,
    handle: k4a_capture_t,
}

impl Capture<'_> {
    pub(crate) fn new(factory: &Factory, handle: k4a_capture_t) -> Capture {
        Capture {
            factory: factory,
            handle: handle,
        }
    }
}

impl Drop for Capture<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_capture_release)(self.handle);
        self.handle = ptr::null_mut();
    }
}
