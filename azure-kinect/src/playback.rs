use super::utility::*;
use super::*;
use std::ptr;

pub struct Playback<'a> {
    pub(crate) factory: &'a FactoryRecord,
    pub(crate) handle: k4a_playback_t,
}

impl Playback<'_> {
    pub(crate) fn from_handle(factory: &FactoryRecord, handle: k4a_playback_t) -> Playback {
        Playback {
            factory: factory,
            handle: handle,
        }
    }

    pub fn get_raw_calibration(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|calibration, buffer| {
            (self.factory.k4a_playback_get_raw_calibration)(self.handle, calibration, buffer as *mut std::os::raw::size_t)
        })
    }
}

impl Drop for Playback<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_playback_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
