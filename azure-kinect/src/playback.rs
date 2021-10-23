use crate::imu::ImuSample;
use crate::playback_data_block::PlaybackDataBlock;
use crate::playback_track::PlaybackTrack;
use crate::utility::*;
use crate::*;
use azure_kinect_sys::k4a::*;
use azure_kinect_sys::k4arecord::{
    k4a_playback_data_block_t, k4a_playback_seek_origin_t, k4a_playback_t,
    k4a_record_configuration_t,
};
use std::ptr;

pub struct RecordConfiguration {
    pub(crate) value: k4a_record_configuration_t,
}

impl RecordConfiguration {
    #[doc = " Image format used to record the color camera."]
    pub fn color_format(&self) -> ImageFormat {
        ImageFormat::from_primitive(self.value.color_format)
    }
    #[doc = " Image resolution used to record the color camera."]
    pub fn color_resolution(&self) -> ColorResolution {
        ColorResolution::from_primitive(self.value.color_resolution)
    }
    #[doc = " Mode used to record the depth camera."]
    pub fn depth_mode(&self) -> DepthMode {
        DepthMode::from_primitive(self.value.depth_mode)
    }
    #[doc = " Frame rate used to record the color and depth camera."]
    pub fn camera_fps(&self) -> Fps {
        Fps::from_primitive(self.value.camera_fps)
    }
    #[doc = " True if the recording contains Color camera frames."]
    pub fn color_track_enabled(&self) -> bool {
        self.value.color_track_enabled
    }
    #[doc = " True if the recording contains Depth camera frames."]
    pub fn depth_track_enabled(&self) -> bool {
        self.value.depth_track_enabled
    }
    #[doc = " True if the recording contains IR camera frames."]
    pub fn ir_track_enabled(&self) -> bool {
        self.value.ir_track_enabled
    }
    #[doc = " True if the recording contains IMU sample data."]
    pub fn imu_track_enabled(&self) -> bool {
        self.value.imu_track_enabled
    }
    #[doc = " The delay between color and depth images in the recording."]
    #[doc = " A negative delay means depth images are first { self.value. } and a positive delay means color images are first."]
    pub fn depth_delay_off_color_usec(&self) -> i32 {
        self.value.depth_delay_off_color_usec
    }
    #[doc = " External synchronization mode"]
    pub fn wired_sync_mode(&self) -> WiredSyncMode {
        WiredSyncMode::from_primitive(self.value.wired_sync_mode)
    }
    #[doc = " The delay between this recording and the externally synced master camera."]
    #[doc = " This value is 0 unless \\p wired_sync_mode is set to ::K4A_WIRED_SYNC_MODE_SUBORDINATE"]
    pub fn subordinate_delay_off_master_usec(&self) -> u32 {
        self.value.subordinate_delay_off_master_usec
    }
    #[doc = " The timestamp offset of the start of the recording. All recorded timestamps are offset by this value such that"]
    #[doc = " the recording starts at timestamp 0. This value can be used to synchronize timestamps between 2 recording files."]
    pub fn start_timestamp_offset_usec(&self) -> u32 {
        self.value.start_timestamp_offset_usec
    }
}

pub struct Playback<'a> {
    pub(crate) factory: &'a FactoryRecord,
    pub(crate) handle: k4a_playback_t,
}

impl<'a> Playback<'a> {
    pub(crate) fn from_handle(
        factory: &'a FactoryRecord,
        handle: azure_kinect_sys::k4arecord::k4a_playback_t,
    ) -> Playback<'a> {
        Playback { factory, handle }
    }

    /// Get the raw calibration blob for the K4A device that made the recording.
    pub fn get_raw_calibration(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|calibration, buffer| unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_raw_calibration)(self.handle, calibration, buffer)
        })
    }

    /// Get the camera calibration for the K4A device that made the recording, which is used for all transformation
    pub fn get_calibration(&self) -> Result<Calibration, Error> {
        let mut calibaraion = k4a_calibration_t::default();
        Error::from_k4a_result_t(unsafe {
            (self.factory.api_record.funcs.k4a_playback_get_calibration)(
                self.handle,
                std::mem::transmute(&mut calibaraion),
            )
        })
        .to_result_fn(|| Calibration::from_handle(&self.factory.core.api, calibaraion))
    }

    /// Gets the configuration of the recording
    pub fn get_record_configuration(&self) -> Result<RecordConfiguration, Error> {
        let mut configuration = k4a_record_configuration_t::default();
        Error::from_k4a_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_record_configuration)(self.handle, &mut configuration)
        })
        .to_result(RecordConfiguration {
            value: configuration,
        })
    }

    /// Get the next capture in the recording.
    pub fn get_next_capture(&self) -> Result<Capture, Error> {
        let mut handle: k4a_capture_t = ptr::null_mut();
        Error::from_k4a_stream_result_t(unsafe {
            (self.factory.api_record.funcs.k4a_playback_get_next_capture)(
                self.handle,
                std::mem::transmute(&mut handle),
            )
        })
        .to_result_fn(|| Capture::from_handle(&self.factory.core.api, handle))
    }

    /// Get the previous capture in the recording.
    pub fn get_previous_capture(&self) -> Result<Capture, Error> {
        let mut handle: k4a_capture_t = ptr::null_mut();
        Error::from_k4a_stream_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_previous_capture)(
                self.handle, std::mem::transmute(&mut handle)
            )
        })
        .to_result_fn(|| Capture::from_handle(&self.factory.core.api, handle))
    }

    /// Reads the value of a tag from the recording
    pub fn get_tag(&self, name: &str) -> Result<String, Error> {
        let name = std::ffi::CString::new(name).unwrap_or_default();
        get_k4a_string(&|tag, buffer| unsafe {
            (self.factory.api_record.funcs.k4a_playback_get_tag)(
                self.handle,
                name.as_ptr(),
                tag,
                buffer,
            )
        })
    }

    /// Get the next IMU sample in the recording.
    pub fn get_next_imu_sample(&self) -> Result<ImuSample, Error> {
        let mut imu_sample = k4a_imu_sample_t::default();
        Error::from_k4a_stream_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_next_imu_sample)(
                self.handle,
                std::mem::transmute(&mut imu_sample),
            )
        })
        .to_result(ImuSample::from_native(imu_sample))
    }

    /// Get the previous IMU sample in the recording.
    pub fn get_previous_imu_sample(&self) -> Result<ImuSample, Error> {
        let mut imu_sample = k4a_imu_sample_t::default();
        Error::from_k4a_stream_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_previous_imu_sample)(
                self.handle,
                std::mem::transmute(&mut imu_sample),
            )
        })
        .to_result(ImuSample::from_native(imu_sample))
    }

    /// Seeks to a specific time point in the recording
    pub fn seek_timestamp(
        &self,
        offset_usec: i64,
        origin: k4a_playback_seek_origin_t,
    ) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self.factory.api_record.funcs.k4a_playback_seek_timestamp)(
                self.handle,
                offset_usec,
                origin,
            )
        })
        .to_result(())
    }

    /// Get the last valid timestamp in the recording
    pub fn get_recording_length_usec(&self) -> u64 {
        unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_recording_length_usec)(self.handle)
        }
    }

    /// Set the image format that color captures will be converted to. By default the conversion format will be the
    /// same as the image format stored in the recording file, and no conversion will occur.
    pub fn set_color_conversion(&mut self, format: ImageFormat) -> Result<(), Error> {
        Error::from_k4a_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_set_color_conversion)(self.handle, format.into())
        })
        .to_result(())
    }

    /// Get the next data block in the recording.
    pub fn get_next_data_block(&self, track: &str) -> Result<PlaybackDataBlock, Error> {
        let mut block_handle: k4a_playback_data_block_t = ptr::null_mut();
        let track = std::ffi::CString::new(track).unwrap_or_default();

        Error::from_k4a_stream_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_next_data_block)(
                self.handle, track.as_ptr(), &mut block_handle
            )
        })
        .to_result_fn(|| PlaybackDataBlock::from_handle(&self.factory.api_record, block_handle))
    }

    /// Get the previous data block from the recording.
    pub fn get_previous_data_block(&self, track: &str) -> Result<PlaybackDataBlock, Error> {
        let mut block_handle: k4a_playback_data_block_t = ptr::null_mut();
        let track = std::ffi::CString::new(track).unwrap_or_default();

        Error::from_k4a_stream_result_t(unsafe {
            (self
                .factory
                .api_record
                .funcs
                .k4a_playback_get_previous_data_block)(
                self.handle,
                track.as_ptr(),
                &mut block_handle,
            )
        })
        .to_result_fn(|| PlaybackDataBlock::from_handle(&self.factory.api_record, block_handle))
    }

    /// Get the attachment block from the recording.
    pub fn get_attachment(&self, attachment: &str) -> Result<Vec<u8>, Error> {
        let attachment = std::ffi::CString::new(attachment).unwrap_or_default();
        get_k4a_binary_data(&|data, data_size| unsafe {
            (self.factory.api_record.funcs.k4a_playback_get_attachment)(
                self.handle,
                attachment.as_ptr(),
                data,
                data_size,
            )
        })
    }

    /// Get the number of tracks in a playback file.
    pub fn get_track_count(&self) -> usize {
        unsafe {
            (self.factory.api_record.funcs.k4a_playback_get_track_count)(self.handle) as usize
        }
    }

    /// Gets the track at a specific index.
    pub fn get_track(&self, track_index: usize) -> Result<PlaybackTrack, Error> {
        Ok(PlaybackTrack::new(
            &self,
            get_k4a_cstring(&|track_name, track_name_size| unsafe {
                (self.factory.api_record.funcs.k4a_playback_get_track_name)(
                    self.handle,
                    track_index,
                    track_name,
                    track_name_size,
                )
            })?,
        ))
    }
}

impl Drop for Playback<'_> {
    fn drop(&mut self) {
        unsafe {
            (self.factory.api_record.funcs.k4a_playback_close)(self.handle);
        }
        self.handle = ptr::null_mut();
    }
}
