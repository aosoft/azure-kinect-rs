use super::*;
use std::ptr;

pub struct PlaybackDataBlock<'a> {
    pub(crate) factory: &'a FactoryRecord,
    pub(crate) handle: k4a_playback_data_block_t,
}

impl PlaybackDataBlock<'_> {
    pub(crate) fn from_handle(factory: &FactoryRecord, handle: k4a_playback_data_block_t) -> PlaybackDataBlock {
        PlaybackDataBlock {
            factory: factory,
            handle: handle,
        }
    }

    /// Get the time stamp in micro seconds for the given data_block
    pub fn get_device_timestamp_usec(&self) -> u64 {
        (self.factory.k4a_playback_data_block_get_device_timestamp_usec)(self.handle)
    }

    /// Get the size of the data_block buffer.
    pub fn get_buffer_size(&self) -> usize {
        (self.factory.k4a_playback_data_block_get_buffer_size)(self.handle) as usize
    }

    /// Get the data_block buffer.
    pub fn get_buffer(&self) -> *const u8 {
        (self.factory.k4a_playback_data_block_get_buffer)(self.handle)
    }
}

impl Drop for PlaybackDataBlock<'_> {
    fn drop(&mut self) {
        (self.factory.k4a_playback_data_block_release)(self.handle);
        self.handle = ptr::null_mut();
    }
}
