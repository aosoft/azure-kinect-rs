use crate::*;
use azure_kinect_sys::k4a::*;
use std::ptr;
use azure_kinect_sys::api::Api;

pub struct Image<'a> {
    funcs: &'a azure_kinect_sys::k4a::Funcs,
    pub(crate) handle: k4a_image_t,
}

impl Image<'_> {
    pub(crate) fn from_handle<'a>(funcs: &'a azure_kinect_sys::k4a::Funcs, handle: k4a_image_t) -> Image<'a> {
        Image {
            funcs: funcs,
            handle: handle,
        }
    }

    /// Create a blank image
    pub fn with_format<'a>(
        factory: &'a Factory,
        format: k4a_image_format_t,
        width_pixels: i32,
        height_pixels: i32,
        stride_bytes: i32,
    ) -> Result<Image<'a>, Error> {
        let mut handle: k4a_image_t = ptr::null_mut();
        Error::from_k4a_result_t((factory.api.k4a().k4a_image_create)(
            format,
            width_pixels,
            height_pixels,
            stride_bytes,
            &mut handle,
        ))
        .to_result_fn(|| Image::from_handle(factory.api.k4a(), handle))
    }

    /// Create an image from a pre-allocated buffer
    pub fn with_buffer<'a>(
        factory: &'a Factory,
        format: k4a_image_format_t,
        width_pixels: i32,
        height_pixels: i32,
        stride_bytes: i32,
        buffer: *mut u8,
        buffer_size: usize,
        buffer_release_cb: k4a_memory_destroy_cb_t,
        buffer_release_cb_context: *mut (),
    ) -> Result<Image<'a>, Error> {
        let mut handle: k4a_image_t = ptr::null_mut();
        Error::from_k4a_result_t((factory.api.k4a().k4a_image_create_from_buffer)(
            format,
            width_pixels,
            height_pixels,
            stride_bytes,
            buffer,
            buffer_size,
            buffer_release_cb,
            buffer_release_cb_context as _,
            &mut handle,
        ))
        .to_result_fn(|| Image::from_handle(factory.api.k4a(), handle))
    }

    /// Get the image buffer
    pub fn get_buffer(&self) -> *const u8 {
        (self.funcs.k4a_image_get_buffer)(self.handle)
    }

    /// Get the mutable image buffer
    pub fn get_mut_buffer(&mut self) -> *mut u8 {
        (self.funcs.k4a_image_get_buffer)(self.handle)
    }

    /// Get the image buffer size in bytes
    pub fn get_size(&self) -> usize {
        (self.funcs.k4a_image_get_size)(self.handle)
    }

    /// Get the image format of the image
    pub fn get_format(&self) -> k4a_image_format_t {
        (self.funcs.k4a_image_get_format)(self.handle)
    }

    /// Get the image width in pixels
    pub fn get_width_pixels(&self) -> i32 {
        (self.funcs.k4a_image_get_width_pixels)(self.handle)
    }

    /// Get the image height in pixels
    pub fn get_height_pixels(&self) -> i32 {
        (self.funcs.k4a_image_get_height_pixels)(self.handle)
    }

    /// Get the image stride in bytes
    pub fn get_stride_bytes(&self) -> i32 {
        (self.funcs.k4a_image_get_stride_bytes)(self.handle)
    }

    /// Get the image's device timestamp in microseconds
    pub fn get_device_timestamp_usec(&self) -> u64 {
        (self.funcs.k4a_image_get_device_timestamp_usec)(self.handle)
    }

    /// Get the image's system timestamp in nanoseconds
    pub fn get_system_timestamp_nsec(&self) -> u64 {
        (self.funcs.k4a_image_get_system_timestamp_nsec)(self.handle)
    }

    /// Get the image exposure time in microseconds
    pub fn get_exposure_usec(&self) -> u64 {
        (self.funcs.k4a_image_get_exposure_usec)(self.handle)
    }

    /// Get the image white balance in Kelvin (color images only)
    pub fn get_white_balance(&self) -> u32 {
        (self.funcs.k4a_image_get_white_balance)(self.handle)
    }

    /// Get the image's ISO speed (color images only)
    pub fn get_iso_speed(&self) -> u32 {
        (self.funcs.k4a_image_get_iso_speed)(self.handle)
    }

    /// Set the image's device timestamp in microseconds
    pub fn set_device_timestamp_usec(&mut self, timestamp: u64) {
        (self.funcs.k4a_image_set_device_timestamp_usec)(self.handle, timestamp)
    }

    /// Set the image's system timestamp in nanoseconds
    pub fn set_system_timestamp_nsec(&self, timestamp: u64) {
        (self.funcs.k4a_image_set_system_timestamp_nsec)(self.handle, timestamp)
    }

    /// Set the image exposure time in microseconds
    pub fn set_exposure_usec(&mut self, exposure: u64) {
        (self.funcs.k4a_image_set_exposure_usec)(self.handle, exposure)
    }

    /// Set the image white balance in Kelvin (color images only)
    pub fn set_white_balance(&mut self, white_balance: u32) {
        (self.funcs.k4a_image_set_white_balance)(self.handle, white_balance)
    }

    /// Set the image's ISO speed (color images only)
    pub fn set_iso_speed(&mut self, iso_speed: u32) {
        (self.funcs.k4a_image_set_iso_speed)(self.handle, iso_speed)
    }
}

impl Drop for Image<'_> {
    fn drop(&mut self) {
        (self.funcs.k4a_image_release)(self.handle);
        self.handle = ptr::null_mut();
    }
}

impl Clone for Image<'_> {
    fn clone(&self) -> Self {
        (self.funcs.k4a_image_reference)(self.handle);
        Image::from_handle(self.funcs, self.handle)
    }
}
