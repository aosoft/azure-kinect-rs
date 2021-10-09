use crate::*;
use azure_kinect_sys::k4a::*;

pub struct Imu<'a> {
    device: &'a Device<'a>,
}

impl Imu<'_> {
    pub(crate) fn new<'a>(device: &'a Device<'a>) -> Result<Imu<'a>, Error> {
        Error::from_k4a_result_t(unsafe { (device.api.funcs.k4a_device_start_imu)(device.handle) } ).to_result(())?;
        Ok(Imu { device })
    }

    /// Reads an IMU sample.  Returns true if a sample was read, false if the read timed out.
    pub fn get_imu_sample(&self, timeout_in_ms: i32) -> Result<k4a_imu_sample_t, Error> {
        let mut imu_sample = k4a_imu_sample_t::default();
        Error::from_k4a_wait_result_t(unsafe { (self.device.api.funcs.k4a_device_get_imu_sample)(
            self.device.handle,
            &mut imu_sample,
            timeout_in_ms,
        ) } )
        .to_result(imu_sample)
    }

    pub fn get_imu_sample_wait_infinite(&self) -> Result<k4a_imu_sample_t, Error> {
        self.get_imu_sample(K4A_WAIT_INFINITE)
    }
}

impl Drop for Imu<'_> {
    fn drop(&mut self) {
        unsafe { (self.device.api.funcs.k4a_device_stop_imu)(self.device.handle) }
    }
}
