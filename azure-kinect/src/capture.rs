use super::*;
use std::ptr;

pub struct Capture<'a> {
    factory: &'a Factory,
    pub(crate) handle: k4a_capture_t,
}

impl Capture<'_> {
    pub fn new(factory: &Factory) -> Result<Capture, Error> {
        let mut handle: k4a_capture_t = ptr::null_mut();
        Error::from((factory.k4a_capture_create)(&mut handle))
            .to_result_fn(|| Capture::from_handle(factory, handle))
    }

    pub(crate) fn from_handle(factory: &Factory, handle: k4a_capture_t) -> Capture {
        Capture {
            factory: factory,
            handle: handle,
        }
    }

    /// Get the color image associated with the capture
    pub fn get_color_image(&self) -> Image {
        Image::from_handle(
            self.factory,
            (self.factory.k4a_capture_get_color_image)(self.handle),
        )
    }

    /// Get the depth image associated with the capture
    pub fn get_depth_image(&self) -> Image {
        Image::from_handle(
            self.factory,
            (self.factory.k4a_capture_get_depth_image)(self.handle),
        )
    }

    /// Get the IR image associated with the capture
    pub fn get_ir_image(&self) -> Image {
        Image::from_handle(
            self.factory,
            (self.factory.k4a_capture_get_ir_image)(self.handle),
        )
    }

    /// Set / add a color image to the capture
    pub fn set_color_image(&mut self, color_image: Image) {
        (self.factory.k4a_capture_set_color_image)(self.handle, color_image.handle)
    }

    /// Set / add a depth image to the capture
    pub fn set_depth_image(&mut self, depth_image: Image) {
        (self.factory.k4a_capture_set_depth_image)(self.handle, depth_image.handle)
    }

    /// Set / add an IR image to the capture
    pub fn set_ir_image(&mut self, ir_image: Image) {
        (self.factory.k4a_capture_set_ir_image)(self.handle, ir_image.handle)
    }

    /// Set the temperature associated with the capture in Celsius.
    pub fn set_temperature_c(&mut self, temperature_c: f32) {
        (self.factory.k4a_capture_set_temperature_c)(self.handle, temperature_c)
    }

    /// Get temperature (in Celsius) associated with the capture.
    pub fn get_temperature_c(&self) -> f32 {
        (self.factory.k4a_capture_get_temperature_c)(self.handle)
    }
}

impl Drop for Capture<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_capture_release)(self.handle);
        self.handle = ptr::null_mut();
    }
}

impl Clone for Capture<'_> {
    fn clone(&self) -> Self {
        (self.factory.k4a_capture_reference)(self.handle);
        Capture::from_handle(self.factory, self.handle)
    }
}
