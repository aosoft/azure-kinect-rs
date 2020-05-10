use super::bindings::*;
use super::error::Error;
use super::factory::Factory;
use std::ptr;

pub struct Image<'a> {
    factory: &'a Factory,
    pub(crate) handle: k4a_image_t,
}

impl Image<'_> {
    pub(crate) fn from_handle(factory: &Factory, handle: k4a_image_t) -> Image {
        Image {
            factory: factory,
            handle: handle,
        }
    }
}

impl Drop for Image<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_image_release)(self.handle);
        self.handle = ptr::null_mut();
    }
}
