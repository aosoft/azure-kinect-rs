use super::utility::*;
use super::*;
use std::ffi::{CStr, CString};
use std::ptr;

pub struct Record<'a> {
    pub(crate) factory: &'a FactoryRecord,
    pub(crate) handle: k4a_record_t,
}

impl Record<'_> {
    pub(crate) fn from_handle(factory: &FactoryRecord, handle: k4a_record_t) -> Record {
        Record {
            factory: factory,
            handle: handle,
        }
    }

    /// Flushes all pending recording data to disk
    pub fn flush(&self) -> Result<(), Error> {
        Error::from((self.factory.k4a_record_flush)(self.handle)).to_result(())
    }

    /// Adds a tag to the recording
    pub fn add_tag(&self, name: &str, value: &str) -> Result<(), Error> {
        let name = CString::new(name).unwrap_or_default();
        let value = CString::new(value).unwrap_or_default();
        Error::from((self.factory.k4a_record_add_tag)(
            self.handle,
            name.as_ptr(),
            value.as_ptr(),
        ))
        .to_result(())
    }

    /// Adds the track header for recording IMU
    pub fn add_imu_track(&self) -> Result<(), Error> {
        Error::from((self.factory.k4a_record_add_imu_track)(self.handle)).to_result(())
    }
}

impl Drop for Record<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_record_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
