use crate::playback::Playback;
use crate::record::Record;
use crate::*;
use azure_kinect_sys::k4a::{k4a_calibration_t, k4a_capture_t, k4a_image_t};
use std::ffi::CString;
use std::os::raw;
use std::ptr;

pub type DebugMessageHandler = dyn Fn(LogLevel, &str, raw::c_int, &str);
pub type MemoryDestroyCallback = extern "C" fn(buffer: *mut (), context: *mut ());

pub trait PreAllocatedBufferInfo {
    fn format(&self) -> ImageFormat;
    fn width_pixels(&self) -> i32;
    fn height_pixels(&self) -> i32;
    fn stride_bytes(&self) -> i32;
    fn buffer(&self) -> *mut u8;
    fn buffer_size(&self) -> usize;
}

pub struct Factory<'a> {
    pub(crate) api: azure_kinect_sys::api::Api,
    debug_message_handler: Option<&'a DebugMessageHandler>,
}

impl<'a> Factory<'a> {
    pub fn new() -> Result<Factory<'a>, Error> {
        Ok(Factory {
            api: azure_kinect_sys::api::Api::new()?,
            debug_message_handler: None,
        })
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<Factory<'a>, Error> {
        Ok(Factory {
            api: azure_kinect_sys::api::Api::with_library_directory(lib_dir)?,
            debug_message_handler: None,
        })
    }

    pub(crate) fn with_get_module() -> Result<Factory<'a>, Error> {
        Ok(Factory {
            api: azure_kinect_sys::api::Api::with_get_module()?,
            debug_message_handler: None,
        })
    }

    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn set_debug_message_handler(
        mut self,
        debug_message_handler: &'a DebugMessageHandler,
        min_level: LogLevel,
    ) -> Self {
        self.set_debug_message_handler_internal(debug_message_handler, min_level);
        self
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn reset_debug_message_handler(mut self) -> Self {
        self.reset_debug_message_handler_internal();
        self
    }

    pub(crate) fn set_debug_message_handler_internal(
        &mut self,
        debug_message_handler: &'a DebugMessageHandler,
        min_level: LogLevel,
    ) {
        self.debug_message_handler = Some(debug_message_handler);
        unsafe {
            (self.api.funcs.k4a_set_debug_message_handler)(
                Some(Self::debug_message_handler_func),
                &self.debug_message_handler as *const Option<&DebugMessageHandler> as _,
                min_level.into(),
            );
        }
    }

    pub(crate) fn reset_debug_message_handler_internal(&mut self) {
        self.debug_message_handler = None;
        unsafe {
            (self.api.funcs.k4a_set_debug_message_handler)(
                None,
                ptr::null_mut(),
                azure_kinect_sys::k4a::k4a_log_level_t_K4A_LOG_LEVEL_OFF,
            );
        }
    }

    /// Gets the number of connected devices
    pub fn device_get_installed_count(&self) -> u32 {
        unsafe { (self.api.funcs.k4a_device_get_installed_count)() }
    }

    /// Open a k4a device.
    pub fn device_open(&self, index: u32) -> Result<Device, Error> {
        let mut handle: azure_kinect_sys::k4a::k4a_device_t = ptr::null_mut();
        Error::from_k4a_result_t(unsafe { (self.api.funcs.k4a_device_open)(index, &mut handle) })
            .to_result_fn(|| Device::from_handle(&self.api, handle))
    }

    /// Get the camera calibration for a device from a raw calibration blob.
    pub fn calibration_get_from_raw(
        &self,
        raw_calibration: &Vec<u8>,
        target_depth_mode: DepthMode,
        target_color_resolution: ColorResolution,
    ) -> Result<Calibration, Error> {
        let mut calibration = k4a_calibration_t::default();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_calibration_get_from_raw)(
                raw_calibration.as_ptr() as *mut i8,
                raw_calibration.len(),
                target_depth_mode.into(),
                target_color_resolution.into(),
                &mut calibration,
            )
        })
        .to_result_fn(|| Calibration::from_handle(&self.api, calibration))
    }

    pub fn capture_create(&self) -> Result<Capture, Error> {
        let mut handle: k4a_capture_t = ptr::null_mut();
        Error::from_k4a_result_t(unsafe { (self.api.funcs.k4a_capture_create)(&mut handle) })
            .to_result_fn(|| Capture::from_handle(&self.api, handle))
    }

    /// Create a blank image
    pub fn image_create(
        &self,
        format: ImageFormat,
        width_pixels: i32,
        height_pixels: i32,
        stride_bytes: i32,
    ) -> Result<Image, Error> {
        let mut handle: k4a_image_t = ptr::null_mut();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_image_create)(
                format.into(),
                width_pixels,
                height_pixels,
                stride_bytes,
                &mut handle,
            )
        })
        .to_result_fn(|| Image::from_handle(&self.api, handle))
    }

    /// Create an image from a pre-allocated buffer
    pub fn image_create_from_buffer_native(
        &self,
        format: ImageFormat,
        width_pixels: i32,
        height_pixels: i32,
        stride_bytes: i32,
        buffer: *mut u8,
        buffer_size: usize,
        buffer_release_cb: Option<MemoryDestroyCallback>,
        buffer_release_cb_context: *mut (),
    ) -> Result<Image, Error> {
        let mut handle: k4a_image_t = ptr::null_mut();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_image_create_from_buffer)(
                format.into(),
                width_pixels,
                height_pixels,
                stride_bytes,
                buffer,
                buffer_size,
                std::mem::transmute(buffer_release_cb),
                buffer_release_cb_context as _,
                &mut handle,
            )
        })
        .to_result_fn(|| Image::from_handle(&self.api, handle))
    }

    /// Create an image from a pre-allocated buffer
    pub fn image_create_from_buffer<T: FnOnce(*mut ())>(
        &self,
        format: ImageFormat,
        width_pixels: i32,
        height_pixels: i32,
        stride_bytes: i32,
        buffer: *mut u8,
        buffer_size: usize,
        buffer_release_cb: Box<T>,
    ) -> Result<Image, Error> {
        self.image_create_from_buffer_native(
            format.into(),
            width_pixels,
            height_pixels,
            stride_bytes,
            buffer,
            buffer_size,
            Some(Self::buffer_release_callback::<T>),
            Box::<T>::into_raw(buffer_release_cb) as _,
        )
    }

    /// Create an image from a pre-allocated buffer
    pub fn image_create_from_buffer_with_info<T: PreAllocatedBufferInfo + Drop>(
        &self,
        buffer_info: Box<T>,
    ) -> Result<Image, Error> {
        self.image_create_from_buffer(
            buffer_info.format().into(),
            buffer_info.width_pixels(),
            buffer_info.height_pixels(),
            buffer_info.stride_bytes(),
            buffer_info.buffer(),
            buffer_info.buffer_size(),
            Box::new(|x| {
                let buffer_info = buffer_info;
            })
        )
    }

    /// Get handle to transformation handle.
    pub fn transformation_create(&'a self, calibration: &'a Calibration) -> Transformation<'a> {
        let handle =
            unsafe { (self.api.funcs.k4a_transformation_create)(&calibration.calibration) };
        Transformation::from_handle(&self, handle, calibration)
    }

    extern "C" fn debug_message_handler_func(
        context: *mut ::std::os::raw::c_void,
        level: azure_kinect_sys::k4a::k4a_log_level_t,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
    ) {
        unsafe {
            let h = context as *const Option<&'a DebugMessageHandler>;
            if h != ptr::null() && (*h).is_some() {
                (*h).as_ref().unwrap()(
                    LogLevel::from_primitive(level),
                    std::ffi::CStr::from_ptr(file).to_str().unwrap_or_default(),
                    line,
                    std::ffi::CStr::from_ptr(message)
                        .to_str()
                        .unwrap_or_default(),
                );
            }
        }
    }

    extern "C" fn buffer_release_callback<T: FnOnce(*mut ())>(buffer: *mut (), context: *mut ()) {
        unsafe {
            let f = Box::<T>::from_raw(context as _);
            f(buffer);
        }
    }
}

pub struct FactoryRecord<'a> {
    pub core: Factory<'a>,
    pub(crate) api_record: azure_kinect_sys::api::ApiRecord,
}

impl<'a> FactoryRecord<'a> {
    pub fn new() -> Result<FactoryRecord<'a>, Error> {
        FactoryRecord::with_api_record(azure_kinect_sys::api::ApiRecord::new()?)
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<FactoryRecord<'a>, Error> {
        FactoryRecord::with_api_record(azure_kinect_sys::api::ApiRecord::with_library_directory(
            lib_dir,
        )?)
    }

    fn with_api_record(
        api_record: azure_kinect_sys::api::ApiRecord,
    ) -> Result<FactoryRecord<'a>, Error> {
        Ok(FactoryRecord {
            core: Factory::with_get_module()?,
            api_record,
        })
    }

    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn set_debug_message_handler(
        mut self,
        debug_message_handler: &'a DebugMessageHandler,
        min_level: LogLevel,
    ) -> Self {
        self.core
            .set_debug_message_handler_internal(debug_message_handler, min_level);
        self
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn reset_debug_message_handler(mut self) -> Self {
        self.core.reset_debug_message_handler_internal();
        self
    }

    /// Opens a K4A recording for playback.
    pub fn playback_open(&self, path: &str) -> Result<Playback, Error> {
        let mut handle: azure_kinect_sys::k4arecord::k4a_playback_t = ptr::null_mut();
        let path = CString::new(path).unwrap_or_default();
        Error::from_k4a_result_t(unsafe {
            (self.api_record.funcs.k4a_playback_open)(path.as_ptr(), &mut handle)
        })
        .to_result_fn(|| Playback::from_handle(&self, handle))
    }

    /// Opens a new recording file for writing
    pub fn record_create(
        &self,
        path: &str,
        device: &Device,
        device_configuration: &DeviceConfiguration,
    ) -> Result<Record, Error> {
        let mut handle: azure_kinect_sys::k4arecord::k4a_record_t = ptr::null_mut();
        let path = CString::new(path).unwrap_or_default();
        Error::from_k4a_result_t(unsafe {
            (self.api_record.funcs.k4a_record_create)(
                path.as_ptr(),
                device.handle as _,
                *device_configuration.for_k4arecord(),
                &mut handle,
            )
        })
        .to_result_fn(|| Record::from_handle(&self.api_record, handle))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let manager = Factory::with_library_directory(
            std::env::current_dir()?.to_str().ok_or(Error::Failed)?,
        );
        assert!(manager.is_ok());
        let manager2 = manager.unwrap();
        let c = unsafe { (manager2.api.funcs.k4a_device_get_installed_count)() };
        println!("device count = {}", c);
        Ok(())
    }

    #[test]
    fn test_image_create_from_buffer() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let factory = Factory::with_library_directory(
            std::env::current_dir()?.to_str().ok_or(Error::Failed)?,
        );
        assert!(factory.is_ok());

        let mut mem = Vec::<u8>::with_capacity(256 * 4 * 256);
        unsafe {
            mem.set_len(mem.capacity());
        }

        let factory = factory.unwrap();
        let image = factory.image_create_from_buffer(
            ImageFormat::BGRA32,
            255,
            256,
            256 * 4,
            &mut mem[0] as _,
            mem.len(),
            Box::new(|x| {
                assert_eq!(x as *const u8, &mem[0] as *const u8);
            }),
        )?;

        Ok(())
    }
}
