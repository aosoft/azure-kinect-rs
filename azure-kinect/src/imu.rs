use crate::*;
use azure_kinect_sys::k4a::*;
use std::fmt::{Display, Formatter};

pub struct ImuSample {
    pub(crate) value: k4a_imu_sample_t,
}

impl ImuSample {
    pub(crate) fn from_native(value: k4a_imu_sample_t) -> ImuSample {
        ImuSample { value }
    }

    #[doc = "< Temperature reading of this sample (Celsius)."]
    pub fn temperature(&self) -> f32 {
        self.value.temperature
    }
    #[doc = "< Accelerometer sample in meters per second squared."]
    pub fn acc_sample(&self) -> Float3 {
        Float3::from_native(self.value.acc_sample)
    }
    #[doc = "< Timestamp of the accelerometer in microseconds."]
    pub fn acc_timestamp_usec(&self) -> u64 {
        self.value.acc_timestamp_usec
    }
    #[doc = "< Gyro sample in radians per second."]
    pub fn gyro_sample(&self) -> Float3 {
        Float3::from_native(self.value.gyro_sample)
    }
    #[doc = "< Timestamp of the gyroscope in microseconds"]
    pub fn gyro_timestamp_usec(&self) -> u64 {
        self.value.gyro_timestamp_usec
    }
}

impl Display for ImuSample {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "temperature: {}, acc_sample: {}, acc_timestamp_usec: {}, gyro_sample: {} gyro_timestamp_usec: {}",
               self.temperature(),
               self.acc_sample(),
               self.acc_timestamp_usec(),
               self.gyro_sample(),
               self.gyro_timestamp_usec())
    }
}

pub struct Imu<'a> {
    device: &'a Device<'a>,
}

impl<'a> Imu<'a> {
    pub(crate) fn new(device: &'a Device<'a>) -> Result<Imu<'a>, Error> {
        Error::from_k4a_result_t(unsafe { (device.api.funcs.k4a_device_start_imu)(device.handle) })
            .to_result(())?;
        Ok(Imu { device })
    }

    /// Reads an IMU sample.  Returns true if a sample was read, false if the read timed out.
    pub fn get_imu_sample(&self, timeout_in_ms: i32) -> Result<ImuSample, Error> {
        let mut imu_sample = k4a_imu_sample_t::default();
        Error::from_k4a_wait_result_t(unsafe {
            (self.device.api.funcs.k4a_device_get_imu_sample)(
                self.device.handle,
                &mut imu_sample,
                timeout_in_ms,
            )
        })
        .to_result(ImuSample::from_native(imu_sample))
    }

    pub fn get_imu_sample_wait_infinite(&self) -> Result<ImuSample, Error> {
        self.get_imu_sample(azure_kinect_sys::k4a::K4A_WAIT_INFINITE)
    }
}

impl Drop for Imu<'_> {
    fn drop(&mut self) {
        unsafe { (self.device.api.funcs.k4a_device_stop_imu)(self.device.handle) }
    }
}
