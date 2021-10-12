use crate::playback::Playback;
use crate::record::Record;
use crate::*;
use std::ffi::CString;
use std::os::raw;
use std::ptr;
use azure_kinect_sys::k4a::{k4a_calibration_t, k4a_capture_t, k4a_image_t, k4a_memory_destroy_cb_t};

pub type DebugMessageHandler = Box<dyn Fn(LogLevel, &str, raw::c_int, &str)>;


pub struct Factory {
    pub(crate) api: azure_kinect_sys::api::Api,
    debug_message_handler: Option<DebugMessageHandler>,
}

impl Factory {
    pub fn new() -> Result<Factory, Error> {
        Ok(Factory {
            api: azure_kinect_sys::api::Api::new()?,
            debug_message_handler: None
        })
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<Factory, Error> {
        Ok(Factory {
            api: azure_kinect_sys::api::Api::with_library_directory(lib_dir)?,
            debug_message_handler: None
        })
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
    pub fn calibration_get_from_raw(&self, raw_calibration: &Vec<u8>,
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
        }).to_result_fn(|| Calibration::from_handle(&self.api, calibration))
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
    pub fn image_create_from_buffer(
        &self,
        format: ImageFormat,
        width_pixels: i32,
        height_pixels: i32,
        stride_bytes: i32,
        buffer: *mut u8,
        buffer_size: usize,
        buffer_release_cb: k4a_memory_destroy_cb_t,
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
                buffer_release_cb,
                buffer_release_cb_context as _,
                &mut handle,
            )
        })
            .to_result_fn(|| Image::from_handle(&self.api, handle))
    }

    /// Get handle to transformation handle.
    pub fn transformation_create<'a>(
        &'a self,
        calibration: &'a Calibration,
    ) -> Transformation<'a> {
        let handle = unsafe { (self.api.funcs.k4a_transformation_create)(&calibration.calibration) };
        Transformation::from_handle(&self, handle, calibration)
    }


    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub(crate) fn set_debug_message_handler(
        &mut self,
        api: &azure_kinect_sys::api::Api,
        debug_message_handler: DebugMessageHandler,
        min_level: LogLevel,
    ) {
        self.debug_message_handler = debug_message_handler.into();
        unsafe {
            (api.funcs.k4a_set_debug_message_handler)(
                Some(Self::debug_message_handler_func),
                &self.debug_message_handler as *const Option<DebugMessageHandler> as _,
                min_level.into(),
            );
        }
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub(crate) fn reset_debug_message_handler(&mut self, api: &azure_kinect_sys::api::Api) {
        self.debug_message_handler = None;
        unsafe {
            (api.funcs.k4a_set_debug_message_handler)(
                None,
                ptr::null_mut(),
                azure_kinect_sys::k4a::k4a_log_level_t_K4A_LOG_LEVEL_OFF,
            );
        }
    }

    extern "C" fn debug_message_handler_func(
        context: *mut ::std::os::raw::c_void,
        level: azure_kinect_sys::k4a::k4a_log_level_t,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
        message: *const ::std::os::raw::c_char,
    ) {
        unsafe {
            let h = context as *const Option<DebugMessageHandler>;
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
}

pub struct FactoryRecord {
    pub core: Factory,
    pub(crate) api_record: azure_kinect_sys::api::ApiRecord,
}

impl FactoryRecord {
    pub fn new() -> Result<FactoryRecord, Error> {
        Ok(FactoryRecord {
            core: Factory::new()?,
            api_record: azure_kinect_sys::api::ApiRecord::new()?,
        })
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<FactoryRecord, Error> {
        Ok(FactoryRecord {
            core: Factory::with_library_directory(lib_dir)?,
            api_record: azure_kinect_sys::api::ApiRecord::with_library_directory(lib_dir)?,
        })
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
}
