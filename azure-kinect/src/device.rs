use crate::utility::*;
use crate::*;
use azure_kinect_sys::k4a::*;
use std::ptr;

pub struct Device<'a> {
    pub(crate) factory: &'a Factory<'a>,
    pub(crate) handle: k4a_device_t,
}

#[derive(Copy, Clone, Default)]
pub struct ColorControlCapabilities {
    supports_auto: bool,
    min_value: i32,
    max_value: i32,
    step_value: i32,
    default_value: i32,
    default_mode: k4a_color_control_mode_t,
}

impl Device<'_> {
    pub(crate) fn from_handle<'a>(factory: &'a Factory, handle: k4a_device_t) -> Device<'a> {
        Device {
            factory: factory,
            handle: handle,
        }
    }

    /// Starts the K4A device's cameras
    pub fn start_cameras(
        &self,
        configuration: &k4a_device_configuration_t,
    ) -> Result<Camera, Error> {
        Camera::new(&self, configuration)
    }

    /// Get the K4A device serial number
    pub fn get_serialnum(&self) -> Result<String, Error> {
        get_k4a_string(&|serialnum, buffer| {
            (self.factory.api.k4a().k4a_device_get_serialnum)(self.handle, serialnum, buffer)
        })
    }

    /// Get the K4A color sensor control value
    pub fn get_color_control(
        &self,
        command: k4a_color_control_command_t,
    ) -> Result<(k4a_color_control_mode_t, i32), Error> {
        let mut mode: k4a_color_control_mode_t =
            k4a_color_control_mode_t::K4A_COLOR_CONTROL_MODE_AUTO;
        let mut value: i32 = 0;
        Error::from_k4a_result_t((self.factory.api.k4a().k4a_device_get_color_control)(
            self.handle,
            command,
            &mut mode,
            &mut value,
        ))
        .to_result((mode, value))
    }

    /// Set the K4A color sensor control value
    pub fn set_color_control(
        &self,
        command: k4a_color_control_command_t,
        mode: k4a_color_control_mode_t,
        value: i32,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t((self.factory.api.k4a().k4a_device_set_color_control)(
            self.handle,
            command,
            mode,
            value,
        ))
        .to_result(())
    }

    pub fn get_color_control_capabilities(
        &self,
        command: k4a_color_control_command_t,
    ) -> Result<ColorControlCapabilities, Error> {
        let mut capabilties = ColorControlCapabilities::default();
        Error::from_k4a_result_t((self.factory.api.k4a().k4a_device_get_color_control_capabilities)(
            self.handle,
            command,
            &mut capabilties.supports_auto,
            &mut capabilties.min_value,
            &mut capabilties.max_value,
            &mut capabilties.step_value,
            &mut capabilties.default_value,
            &mut capabilties.default_mode,
        ))
        .to_result(capabilties)
    }

    /// Get the raw calibration blob for the entire K4A device.
    pub fn get_raw_calibration(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|calibration, buffer| {
            (self.factory.api.k4a().k4a_device_get_raw_calibration)(self.handle, calibration, buffer)
        })
    }

    /// Get the camera calibration for the entire K4A device, which is used for all transformation functions.
    pub fn get_calibration(
        &self,
        depth_mode: k4a_depth_mode_t,
        color_resolution: k4a_color_resolution_t,
    ) -> Result<Calibration, Error> {
        let mut calibaraion = k4a_calibration_t::default();
        Error::from((self.factory.api.k4a().k4a_device_get_calibration)(
            self.handle,
            depth_mode,
            color_resolution,
            &mut calibaraion,
        ))
        .to_result_fn(|| Calibration::from_handle(self.factory, calibaraion))
    }

    /// Get the device jack status for the synchronization connectors
    pub fn is_sync_connected(&self) -> Result<(bool, bool), Error> {
        let mut sync_in_jack_connected = false;
        let mut sync_out_jack_connected = false;
        Error::from((self.factory.api.k4a().k4a_device_get_sync_jack)(
            self.handle,
            &mut sync_in_jack_connected,
            &mut sync_out_jack_connected,
        ))
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
    pub fn get_version(&self) -> Result<k4a_hardware_version_t, Error> {
        let mut version = k4a_hardware_version_t::default();
        Error::from((self.factory.api.k4a().k4a_device_get_version)(
            self.handle,
            &mut version,
        ))
        .to_result(version)
    }
}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        (self.factory.api.k4a().k4a_device_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
