use crate::utility::*;
use crate::*;
use azure_kinect_sys::k4a::*;
use std::ptr;

pub struct Device<'a> {
    pub(crate) api: &'a azure_kinect_sys::api::Api,
    pub(crate) handle: k4a_device_t,
}

#[derive(Copy, Clone)]
pub struct ColorControlCapabilities {
    pub supports_auto: bool,
    pub min_value: i32,
    pub max_value: i32,
    pub step_value: i32,
    pub default_value: i32,
    pub default_mode: ColorControlMode,
}

impl Device<'_> {
    pub(crate) fn from_handle(
        api: &azure_kinect_sys::api::Api,
        handle: k4a_device_t,
    ) -> Device {
        Device {
            api,
            handle,
        }
    }

    /// Starts the K4A device's cameras
    pub fn start_cameras(
        &self,
        configuration: &DeviceConfiguration,
    ) -> Result<Camera, Error> {
        Camera::new(&self, configuration)
    }

    /// Get the K4A device serial number
    pub fn get_serialnum(&self) -> Result<String, Error> {
        get_k4a_string(&|serialnum, buffer| unsafe {
            (self.api.funcs.k4a_device_get_serialnum)(self.handle, serialnum, buffer)
        })
    }

    /// Get the K4A color sensor control value
    pub fn get_color_control(
        &self,
        command: ColorControlCommand,
    ) -> Result<(ColorControlMode, i32), Error> {
        let mut mode: k4a_color_control_mode_t =
            k4a_color_control_mode_t_K4A_COLOR_CONTROL_MODE_AUTO;
        let mut value: i32 = 0;
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_device_get_color_control)(
                self.handle,
                command.into(),
                &mut mode,
                &mut value,
            )
        })
        .to_result((ColorControlMode::from_primitive(mode), value))
    }

    /// Set the K4A color sensor control value
    pub fn set_color_control(
        &mut self,
        command: ColorControlCommand,
        mode: ColorControlMode,
        value: i32,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_device_set_color_control)(self.handle, command.into(), mode.into(), value)
        })
        .to_result(())
    }

    pub fn get_color_control_capabilities(
        &self,
        command: ColorControlCommand,
    ) -> Result<ColorControlCapabilities, Error> {
        let mut capabilties = unsafe { std::mem::zeroed::<ColorControlCapabilities>() };
        let mut mode: k4a_color_control_mode_t = k4a_color_control_mode_t::default();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_device_get_color_control_capabilities)(
                self.handle,
                command.into(),
                &mut capabilties.supports_auto,
                &mut capabilties.min_value,
                &mut capabilties.max_value,
                &mut capabilties.step_value,
                &mut capabilties.default_value,
                &mut mode,
            )
        })
        .to_result({
            capabilties.default_mode = ColorControlMode::from_primitive(mode);
            capabilties
        })
    }

    /// Get the raw calibration blob for the entire K4A device.
    pub fn get_raw_calibration(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|calibration, buffer| unsafe {
            (self.api.funcs.k4a_device_get_raw_calibration)(self.handle, calibration, buffer)
        })
    }

    /// Get the camera calibration for the entire K4A device, which is used for all transformation functions.
    pub fn get_calibration(
        &self,
        depth_mode: DepthMode,
        color_resolution: ColorResolution,
    ) -> Result<Calibration, Error> {
        let mut calibaraion = k4a_calibration_t::default();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_device_get_calibration)(
                self.handle,
                depth_mode.into(),
                color_resolution.into(),
                &mut calibaraion,
            )
        })
        .to_result_fn(|| Calibration::from_handle(self.api, calibaraion))
    }

    /// Get the device jack status for the synchronization connectors
    pub fn is_sync_connected(&self) -> Result<(bool, bool), Error> {
        let mut sync_in_jack_connected = false;
        let mut sync_out_jack_connected = false;
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_device_get_sync_jack)(
                self.handle,
                &mut sync_in_jack_connected,
                &mut sync_out_jack_connected,
            )
        })
        .to_result((sync_in_jack_connected, sync_out_jack_connected))
    }

    /// Get the device jack status for the synchronization in connector
    pub fn is_sync_in_connected(&self) -> Result<bool, Error> {
        Ok(self.is_sync_connected()?.0)
    }

    /// Get the device jack status for the synchronization out connector
    pub fn is_sync_out_connected(&self) -> Result<bool, Error> {
        Ok(self.is_sync_connected()?.1)
    }

    /// Get the version numbers of the K4A subsystems' firmware
    pub fn get_version(&self) -> Result<HardwareVersion, Error> {
        let mut version = k4a_hardware_version_t::default();
        Error::from_k4a_result_t(unsafe {
            (self.api.funcs.k4a_device_get_version)(self.handle, &mut version)
        })
        .to_result(HardwareVersion{ value: version })
    }
}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        unsafe {
            (self.api.funcs.k4a_device_close)(self.handle);
        }
        self.handle = ptr::null_mut();
    }
}


#[derive(Copy, Clone, Default)]
pub struct DeviceConfiguration {
    pub(crate) value: k4a_device_configuration_t,
}

impl DeviceConfiguration {
    pub(crate) fn for_k4arecord(&self) -> &azure_kinect_sys::k4arecord::k4a_device_configuration_t {
        unsafe { std::mem::transmute(&self.value) }
    }

    pub fn builder() -> DeviceConfigurationBuilder { DeviceConfigurationBuilder::default() }

    #[doc = " Image format to capture with the color camera."]
    pub fn color_format(&self) -> ImageFormat { ImageFormat::from_primitive(self.value.color_format) }

    #[doc = " Image resolution to capture with the color camera."]
    pub fn color_resolution(&self) -> ColorResolution { ColorResolution::from_primitive(self.value.color_resolution) }

    #[doc = " Capture mode for the depth camera."]
    pub fn depth_mode(&self) -> DepthMode { DepthMode::from_primitive(self.value.depth_mode) }

    #[doc = " Desired frame rate for the color and depth camera."]
    pub fn camera_fps(&self) -> Fps { Fps::from_primitive(self.value.camera_fps.into()) }

    #[doc = " Only produce k4a_capture_t objects if they contain synchronized color and depth images."]
    pub fn synchronized_images_only(&self) -> bool { self.value.synchronized_images_only }

    #[doc = " Desired delay between the capture of the color image and the capture of the depth image."]
    pub fn depth_delay_off_color_usec(&self) -> i32 { self.value.depth_delay_off_color_usec }

    #[doc = " The external synchronization mode."]
    pub fn wired_sync_mode(&self) -> WiredSyncMode { WiredSyncMode::from_primitive(self.value.wired_sync_mode.into()) }

    #[doc = " The external synchronization timing."]
    pub fn subordinate_delay_off_master_usec(&self) -> u32 { self.value.subordinate_delay_off_master_usec }

    #[doc = " Streaming indicator automatically turns on when the color or depth camera's are in use."]
    pub fn disable_streaming_indicator(&self) -> bool { self.value.disable_streaming_indicator }
}

#[derive(Default)]
pub struct DeviceConfigurationBuilder {
    value: k4a_device_configuration_t,
}

impl DeviceConfigurationBuilder {
    #[doc = " Image format to capture with the color camera."]
    pub fn color_format(mut self, value: ImageFormat) -> DeviceConfigurationBuilder {
        self.value.color_format = value.into();
        self
    }

    #[doc = " Image resolution to capture with the color camera."]
    pub fn color_resolution(mut self, value: ColorResolution) -> DeviceConfigurationBuilder {
        self.value.color_resolution = value.into();
        self
    }

    #[doc = " Capture mode for the depth camera."]
    pub fn depth_mode(mut self, value: DepthMode) -> DeviceConfigurationBuilder {
        self.value.depth_mode = value.into();
        self
    }

    #[doc = " Desired frame rate for the color and depth camera."]
    pub fn camera_fps(mut self, value: Fps) -> DeviceConfigurationBuilder {
        self.value.camera_fps = value.into();
        self
    }

    #[doc = " Only produce k4a_capture_t objects if they contain synchronized color and depth images."]
    pub fn synchronized_images_only(mut self, value: bool) -> DeviceConfigurationBuilder {
        self.value.synchronized_images_only = value;
        self
    }

    #[doc = " Desired delay between the capture of the color image and the capture of the depth image."]
    pub fn depth_delay_off_color_usec(mut self, value: i32) -> DeviceConfigurationBuilder {
        self.value.depth_delay_off_color_usec = value;
        self
    }

    #[doc = " The external synchronization mode."]
    pub fn wired_sync_mode(mut self, value: WiredSyncMode) -> DeviceConfigurationBuilder {
        self.value.wired_sync_mode = value.into();
        self
    }

    #[doc = " The external synchronization timing."]
    pub fn subordinate_delay_off_master_usec(mut self, value: u32) -> DeviceConfigurationBuilder {
        self.value.subordinate_delay_off_master_usec = value;
        self
    }

    #[doc = " Streaming indicator automatically turns on when the color or depth camera's are in use."]
    pub fn disable_streaming_indicator(mut self, value: bool) -> DeviceConfigurationBuilder {
        self.value.disable_streaming_indicator = value;
        self
    }

    pub fn build(&self) -> DeviceConfiguration {
        DeviceConfiguration { value: self.value }
    }
}