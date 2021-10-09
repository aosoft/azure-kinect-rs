use crate::error::ToResult;
use crate::*;
use crate::playback::Playback;
use crate::record::Record;
use std::ffi::{c_void, CString};
use std::os::raw;
use std::ptr;
use azure_kinect_sys::k4a::k4a_log_level_t;


pub type DebugMessageHandler = Box<dyn Fn(k4a_log_level_t, &str, raw::c_int, &str)>;

struct DebugMessageHandlerRegister {
    debug_message_handler: Option<DebugMessageHandler>,
}

impl DebugMessageHandlerRegister {
    pub fn new() -> DebugMessageHandlerRegister {
        DebugMessageHandlerRegister { debug_message_handler: None }
    }

    /// Sets and clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn set_debug_message_handler(
        &mut self,
        api: &azure_kinect_sys::api::Api,
        debug_message_handler: DebugMessageHandler,
        min_level: k4a_log_level_t,
    ) {
        self.debug_message_handler = debug_message_handler.into();
        (api.funcs.k4a_set_debug_message_handler)(
            Some(Self::debug_message_handler_func),
            &self.debug_message_handler as *const Option<DebugMessageHandler> as _,
            min_level as azure_kinect_sys::k4a::k4a_log_level_t,
        );
    }

    /// Clears the callback function to receive debug messages from the Azure Kinect device.
    pub fn reset_debug_message_handler(mut self, api: &azure_kinect_sys::api::Api) {
        self.debug_message_handler = None;
        (api.funcs.k4a_set_debug_message_handler)(
            None,
            ptr::null_mut(),
            azure_kinect_sys::k4a::k4a_log_level_t_K4A_LOG_LEVEL_OFF,
        );
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
                    level,
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


pub struct Factory {
    pub(crate) api: azure_kinect_sys::api::Api,
    debug_message_handler: DebugMessageHandlerRegister,
}

impl Factory {
    pub fn new() -> Result<Factory, Error> {
        let api = azure_kinect_sys::api::Api::new()?;
        Ok(Factory {
            debug_message_handler: DebugMessageHandlerRegister::new(),
            api,
        })
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<Factory, Error> {
        let api = azure_kinect_sys::api::Api::with_library_directory(lib_dir)?;
        Ok(Factory {
            debug_message_handler: DebugMessageHandlerRegister::new(),
            api,
        })
    }

    pub fn set_debug_message_handler(
        &mut self,
        debug_message_handler: DebugMessageHandler,
        min_level: k4a_log_level_t,
    ) {
        self.debug_message_handler.set_debug_message_handler(&self.api, debug_message_handler, min_level)
    }

    pub fn reset_debug_message_handler(mut self) {
        self.debug_message_handler.reset_debug_message_handler(&self.api);
    }


    /// Gets the number of connected devices
    pub fn device_get_installed_count(&self) -> u32 {
        (self.api.funcs.k4a_device_get_installed_count)()
    }

    /// Open a k4a device.
    pub fn device_open(&self, index: u32) -> Result<Device, Error> {
        let mut handle: azure_kinect_sys::k4a::k4a_device_t = ptr::null_mut();
        Error::from_k4a_result_t((self.api.funcs.k4a_device_open)(index, &mut handle))
            .to_result_fn(|| Device::from_handle(&self.api, handle))
    }
}


pub struct FactoryRecord {
    pub(crate) api_record: azure_kinect_sys::api::ApiRecord,
    debug_message_handler: DebugMessageHandlerRegister,
}

impl FactoryRecord {
    pub fn new() -> Result<FactoryRecord, Error> {
        let api_record = azure_kinect_sys::api::ApiRecord::new()?;
        Ok(FactoryRecord {
            debug_message_handler: DebugMessageHandlerRegister::new(),
            api_record,
        })
    }

    pub fn with_library_directory(lib_dir: &str) -> Result<FactoryRecord, Error> {
        let api_record = azure_kinect_sys::api::ApiRecord::with_library_directory(lib_dir)?;
        Ok(FactoryRecord {
            debug_message_handler: DebugMessageHandlerRegister::new(),
            api_record,
        })
    }

    pub fn set_debug_message_handler(
        &mut self,
        debug_message_handler: DebugMessageHandler,
        min_level: k4a_log_level_t,
    ) {
        self.debug_message_handler.set_debug_message_handler(&self.api_record.k4a, debug_message_handler, min_level)
    }

    pub fn reset_debug_message_handler(mut self) {
        self.debug_message_handler.reset_debug_message_handler(&self.api_record.k4a);
    }

    /// Gets the number of connected devices
    pub fn device_get_installed_count(&self) -> u32 {
        (self.api_record.k4a.funcs.k4a_device_get_installed_count)()
    }

    /// Open a k4a device.
    pub fn device_open(&self, index: u32) -> Result<Device, Error> {
        let mut handle: azure_kinect_sys::k4a::k4a_device_t = ptr::null_mut();
        Error::from_k4a_result_t((self.api_record.k4a.funcs.k4a_device_open)(index, &mut handle))
            .to_result_fn(|| Device::from_handle(&self.api_record.k4a, handle))
    }

    /// Opens a K4A recording for playback.
    pub fn playback_open(&self, path: &str) -> Result<Playback, Error> {
        let mut handle: azure_kinect_sys::k4arecord::k4a_playback_t = ptr::null_mut();
        let path = CString::new(path).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_playback_open)(path.as_ptr(), &mut handle))
            .to_result_fn(|| Playback::from_handle(&self.api_record, handle))
    }

    /// Opens a new recording file for writing
    pub fn record_create(
        &self,
        path: &str,
        device: &Device,
        device_configuration: &azure_kinect_sys::k4arecord::k4a_device_configuration_t,
    ) -> Result<Record, Error> {
        let mut handle: azure_kinect_sys::k4arecord::k4a_record_t = ptr::null_mut();
        let path = CString::new(path).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_create)(
            path.as_ptr(),
            device.handle as _,
            *device_configuration,
            &mut handle,
        ))
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
        let c = (manager2.k4a_device_get_installed_count)();
        println!("device count = {}", c);
        Ok(())
    }
}
