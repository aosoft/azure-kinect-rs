use super::bindings::*;
use super::calibration::Calibration;
use super::capture::Capture;
use super::error::Error;
use super::factory::Factory;
use super::utility::*;
use std::ptr;

pub struct Device<'a> {
    factory: &'a Factory,
    handle: k4a_device_t,
}

impl Device<'_> {
    pub(crate) fn from_handle(factory: &Factory, handle: k4a_device_t) -> Device {
        Device {
            factory: factory,
            handle: handle,
        }
    }

    /// Reads a sensor capture into cap.  Returns true if a capture was read, false if the read timed out.
    pub fn get_capture(&self, timeout_in_ms: i32) -> Result<Capture, Error> {
        unsafe {
            let mut handle: k4a_capture_t = ptr::null_mut();
            Error::from((self.factory.k4a_device_get_capture)(
                self.handle,
                &mut handle,
                timeout_in_ms,
            ))
            .to_result_fn(&|| Capture::from_handle(self.factory, handle))
        }
    }

    /// Reads a sensor capture into cap.  Returns true if a capture was read, false if the read timed out.
    pub fn get_capture_wait_infinite(&self) -> Result<Capture, Error> {
        self.get_capture(K4A_WAIT_INFINITE)
    }

    /// Reads an IMU sample.  Returns true if a sample was read, false if the read timed out.
    pub fn get_imu_sample(&self, timeout_in_ms: i32) -> Result<k4a_imu_sample_t, Error> {
        unsafe {
            let mut imu_sample = k4a_imu_sample_t::default();
            Error::from((self.factory.k4a_device_get_imu_sample)(
                self.handle,
                &mut imu_sample,
                timeout_in_ms,
            ))
            .to_result(imu_sample)
        }
    }

    pub fn get_imu_sample_wait_infinite(&self) -> Result<k4a_imu_sample_t, Error> {
        self.get_imu_sample(K4A_WAIT_INFINITE)
    }

    /// Starts the K4A device's cameras
    pub fn start_cameras(&self, configuration: &k4a_device_configuration_t) -> Result<(), Error> {
        Error::from((self.factory.k4a_device_start_cameras)(
            self.handle,
            configuration,
        ))
        .to_result(())
    }

    /// Stops the K4A device's cameras
    pub fn stop_cameras(&self) {
        (self.factory.k4a_device_stop_cameras)(self.handle);
    }

    /// Starts the K4A IMU
    pub fn start_imu(&self) -> Result<(), Error> {
        Error::from((self.factory.k4a_device_start_imu)(self.handle)).to_result(())
    }

    /// Stops the K4A IMU
    pub fn stop_imu(&self) {
        (self.factory.k4a_device_stop_imu)(self.handle)
    }

    /// Get the K4A device serial number
    pub fn get_serialnum(&self) -> Result<String, Error> {
        get_k4a_string(&|serialnum, buffer| {
            (self.factory.k4a_device_get_serialnum)(self.handle, serialnum, buffer)
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
        unsafe {
            Error::from((self.factory.k4a_device_get_color_control)(
                self.handle,
                command,
                &mut mode,
                &mut value,
            ))
            .to_result((mode, value))
        }
    }

    /// Set the K4A color sensor control value
    pub fn set_color_control(
        &self,
        command: k4a_color_control_command_t,
        mode: k4a_color_control_mode_t,
        value: i32,
    ) -> Result<(), Error> {
        Error::from((self.factory.k4a_device_set_color_control)(
            self.handle,
            command,
            mode,
            value,
        ))
        .to_result(())
    }

    /// Get the raw calibration blob for the entire K4A device.
    pub fn get_raw_calibration(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|calibration, buffer| {
            (self.factory.k4a_device_get_raw_calibration)(self.handle, calibration, buffer)
        })
    }

    /// Get the camera calibration for the entire K4A device, which is used for all transformation functions.
    pub fn get_calibration(
        &self,
        depth_mode: k4a_depth_mode_t,
        color_resolution: k4a_color_resolution_t,
    ) -> Result<Calibration, Error> {
        unsafe {
            let mut calibaraion = k4a_calibration_t::default();
            Error::from((self.factory.k4a_device_get_calibration)(
                self.handle,
                depth_mode,
                color_resolution,
                &mut calibaraion,
            ))
            .to_result_fn(&|| Calibration::from_handle(self.factory, calibaraion))
        }
    }

    /// Get the device jack status for the synchronization connectors
    pub fn is_sync_connected(&self) -> Result<(bool, bool), Error> {
        unsafe {
            let mut sync_in_jack_connected = false;
            let mut sync_out_jack_connected = false;
            Error::from((self.factory.k4a_device_get_sync_jack)(
                self.handle,
                &mut sync_in_jack_connected,
                &mut sync_out_jack_connected,
            ))
            .to_result((sync_in_jack_connected, sync_out_jack_connected))
        }
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
        unsafe {
            let mut version = k4a_hardware_version_t::default();
            Error::from((self.factory.k4a_device_get_version)(
                self.handle,
                &mut version,
            ))
            .to_result(version)
        }
    }
}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_device_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
