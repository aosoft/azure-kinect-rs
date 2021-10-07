use crate::*;
use azure_kinect_sys::k4a::*;
use std::ptr;

pub struct Camera<'a> {
    pub(crate) device: &'a Device<'a>,
}

impl Camera<'_> {
    pub(crate) fn new<'a>(
        device: &'a Device<'a>,
        configuration: &k4a_device_configuration_t,
    ) -> Result<Camera<'a>, Error> {
        Error::from_k4a_result_t((device.factory.api.k4a().k4a_device_start_cameras)(
            device.handle,
            configuration,
        ))
        .to_result(())?;
        Ok(Camera::<'a> { device })
    }

    /// Reads a sensor capture into cap.  Returns true if a capture was read, false if the read timed out.
    pub fn get_capture(&self, timeout_in_ms: i32) -> Result<Capture, Error> {
        let mut handle: k4a_capture_t = ptr::null_mut();
        Error::from_k4a_wait_result_t((self.device.factory.api.k4a().k4a_device_get_capture)(
            self.device.handle,
            &mut handle,
            timeout_in_ms,
        ))
        .to_result_fn(|| Capture::from_handle(self.device.factory, handle))
    }

    /// Reads a sensor capture into cap.  Returns true if a capture was read, false if the read timed out.
    pub fn get_capture_wait_infinite(&self) -> Result<Capture, Error> {
        self.get_capture(K4A_WAIT_INFINITE)
    }

    /// Starts the K4A IMU
    pub fn start_imu(&self) -> Result<Imu, Error> {
        Imu::new(self.device)
    }
}

impl Drop for Camera<'_> {
    fn drop(&mut self) {
        (self.device.factory.api.k4a().k4a_device_stop_cameras)(self.device.handle);
    }
}
