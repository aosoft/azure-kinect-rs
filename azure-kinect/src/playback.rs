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

    /// Get the raw calibration blob for the K4A device that made the recording.
    pub fn get_raw_calibration(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|calibration, buffer| {
            (self.factory.k4a_playback_get_raw_calibration)(
                self.handle,
                calibration,
                buffer as *mut size_t,
            )
        })
    }

    /// Get the camera calibration for the K4A device that made the recording, which is used for all transformation
    pub fn get_calibration(&self) -> Result<Calibration, Error> {
        let mut calibaraion = k4a_calibration_t::default();
        Error::from((self.factory.k4a_playback_get_calibration)(
            self.handle,
            &mut calibaraion,
        ))
        .to_result_fn(&|| Calibration::from_handle(&self.factory.k4a, calibaraion))
    }

    pub fn get_record_configuration(&self) -> Result<k4a_record_configuration_t, Error> {
        let mut configuration = k4a_record_configuration_t::default();
        Error::from((self.factory.k4a_playback_get_record_configuration)(
            self.handle,
            &mut configuration,
        ))
        .to_result(configuration)
    }
}

impl Drop for Playback<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_playback_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
