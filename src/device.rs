use super::bindings::*;
use super::capture::Capture;
use super::error::Error;
use super::factory::Factory;
use crate::error::Error::Succeded;
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

    /// Reads a sensor capture into cap.  Returns true if a capture was read, false if the read timed out.
    pub fn get_capture(&self, timeout_in_ms: i32) -> Result<Capture, Error> {
        unsafe {
            let mut handle: k4a_capture_t = ptr::null_mut();
            Error::from((self.factory.k4a_device_get_capture)(
                self.handle,
                &mut handle,
                timeout_in_ms,
            ))
            .to_result_fn(&|| Capture::new(self.factory, handle))
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

    pub fn start_cameras(&self, configuration: &k4a_device_configuration_t) -> Result<(), Error> {
        Error::from((self.factory.k4a_device_start_cameras)(
            self.handle,
            configuration,
        ))
        .to_result(())
    }

    pub fn stop_cameras(&self) {
        (self.factory.k4a_device_stop_cameras)(self.handle);
    }

    pub fn start_imu(&self) -> Result<(), Error> {
        Error::from((self.factory.k4a_device_start_imu)(self.handle)).to_result(())
    }

    pub fn stop_imu(&self) {
        (self.factory.k4a_device_stop_imu)(self.handle)
    }

    pub fn get_serialnum(&self) -> Result<String, Error> {
        unsafe {
            let mut buffer: usize = 0;
            let r: Error =
                (self.factory.k4a_device_get_serialnum)(self.handle, ptr::null_mut(), &mut buffer)
                    .into();
            match r {
                Succeded => Ok(String::new()),
                Error::TooSmall => {
                    if (buffer > 1) {
                        let mut serialnum = String::with_capacity(buffer);
                        serialnum.as_mut_vec().set_len(buffer - 1);
                        Error::from((self.factory.k4a_device_get_serialnum)(
                            self.handle,
                            serialnum.as_mut_ptr() as *mut ::std::os::raw::c_char,
                            &mut buffer,
                        ))
                        .to_result(serialnum)
                    } else {
                        Err(r)
                    }
                }
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
