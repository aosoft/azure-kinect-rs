use super::bindings::*;
use super::capture::Capture;
use super::error::Error;
use super::factory::Factory;
use std::ptr;

pub struct Device<'a> {
    factory: &'a Factory,
    handle: k4a_device_t,
}

impl Device<'_> {
    pub(crate) fn new(factory: &Factory, handle: k4a_device_t) -> Device {
        Device {
            factory: factory,
            handle: handle,
        }
    }

    pub fn get_capture(&self, timeout_in_ms: i32) -> Result<Capture, Error> {
        unsafe {
            let mut handle: k4a_capture_t = ptr::null_mut();
            let r: Error =
                (self.factory.k4a_device_get_capture)(self.handle, &mut handle, timeout_in_ms)
                    .into();
            match r {
                Succeded => Ok(Capture::new(self.factory, handle)),
                _ => Err(r),
            }
        }
    }

    pub fn get_capture_wait_infinite(&self) -> Result<Capture, Error> {
        self.get_capture(K4A_WAIT_INFINITE)
    }

    pub fn get_imu_sample(&self, timeout_in_ms: i32) -> Result<k4a_imu_sample_t, Error> {
        unsafe {
            let mut imu_sample = k4a_imu_sample_t {
                acc_sample: k4a_float3_t { xyz: k4a_float3_t__xyz{x:0.0, y:0.0, z:0.0}},
                acc_timestamp_usec: 0,
                gyro_sample: k4a_float3_t { xyz: k4a_float3_t__xyz{x:0.0, y:0.0, z:0.0}},
                gyro_timestamp_usec: 0,
                temperature: 0.0,
            };
            let r: Error = (self.factory.k4a_device_get_imu_sample)(
                self.handle,
                &mut imu_sample,
                timeout_in_ms,
            )
            .into();
            match r {
                Succeded => Ok(imu_sample),
                _ => Err(r),
            }
        }
    }
}

impl Drop for Device<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_device_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
