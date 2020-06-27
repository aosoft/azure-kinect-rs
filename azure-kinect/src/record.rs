use super::utility::*;
use super::*;
use crate::playback_data_block::PlaybackDataBlock;
use crate::playback_track::PlaybackTrack;
use std::ptr;

pub struct Record<'a> {
    pub(crate) factory: &'a FactoryRecord,
    pub(crate) handle: k4a_record_t,
}




impl Drop for Record<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_record_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
