use crate::*;
use crate::Capture;
use azure_kinect_sys::k4a::*;
use azure_kinect_sys::k4arecord::{k4a_record_subtitle_settings_t, k4a_record_t, k4a_record_video_settings_t};
use std::ffi::CString;
use std::ptr;

pub struct Record<'a> {
    pub(crate) api_record: &'a azure_kinect_sys::api::ApiRecord,
    pub(crate) handle: k4a_record_t,
}

impl Record<'_> {
    pub(crate) fn from_handle<'a>(api_record: &'a azure_kinect_sys::api::ApiRecord, handle: k4a_record_t) -> Record<'a> {
        Record {
            api_record: api_record,
            handle: handle,
        }
    }

    /// Flushes all pending recording data to disk
    pub fn flush(&self) -> Result<(), Error> {
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_flush)(self.handle)).to_result(())
    }

    /// Adds a tag to the recording
    pub fn add_tag(&self, name: &str, value: &str) -> Result<(), Error> {
        let name = CString::new(name).unwrap_or_default();
        let value = CString::new(value).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_add_tag)(
            self.handle,
            name.as_ptr(),
            value.as_ptr(),
        ))
        .to_result(())
    }

    /// Adds the track header for recording IMU
    pub fn add_imu_track(&self) -> Result<(), Error> {
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_add_imu_track)(self.handle)).to_result(())
    }

    /// Adds an attachment to the recording
    pub fn add_attachment(&self, attachment_name: &str, buffer: &[u8]) -> Result<(), Error> {
        let attachment_name = CString::new(attachment_name).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_add_attachment)(
            self.handle,
            attachment_name.as_ptr(),
            buffer.as_ptr(),
            buffer.len(),
        ))
        .to_result(())
    }

    /// Adds custom video tracks to the recording
    pub fn add_custom_video_track(
        &self,
        track_name: &str,
        codec_id: &str,
        codec_context: &[u8],
        track_settings: &k4a_record_video_settings_t,
    ) -> Result<(), Error> {
        let track_name = CString::new(track_name).unwrap_or_default();
        let codec_id = CString::new(codec_id).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_add_custom_video_track)(
            self.handle,
            track_name.as_ptr(),
            codec_id.as_ptr(),
            codec_context.as_ptr(),
            codec_context.len(),
            track_settings,
        ))
        .to_result(())
    }

    /// Adds custom subtitle tracks to the recording
    pub fn add_custom_subtitle_track(
        &self,
        track_name: &str,
        codec_id: &str,
        codec_context: &[u8],
        track_settings: &k4a_record_subtitle_settings_t,
    ) -> Result<(), Error> {
        let track_name = CString::new(track_name).unwrap_or_default();
        let codec_id = CString::new(codec_id).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_add_custom_subtitle_track)(
            self.handle,
            track_name.as_ptr(),
            codec_id.as_ptr(),
            codec_context.as_ptr(),
            codec_context.len(),
            track_settings,
        ))
        .to_result(())
    }

    /// Writes the recording header and metadata to file
    pub fn write_header(&self) -> Result<(), Error> {
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_write_header)(self.handle)).to_result(())
    }

    /// Writes a camera capture to file
    pub fn write_capture(&self, capture: &Capture) -> Result<(), Error> {
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_write_capture)(
            self.handle,
            capture.handle as _,
        ))
        .to_result(())
    }

    /// Writes an imu sample to file
    pub fn write_imu_sample(&self, imu_sample: k4a_imu_sample_t) -> Result<(), Error> {
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_write_imu_sample)(
            self.handle,
            unsafe { std::mem::transmute(imu_sample) } ,
        ))
        .to_result(())
    }

    pub fn write_custom_track_data(
        &self,
        track_name: &str,
        device_timestamp_usec: u64,
        custom_data: &[u8],
    ) -> Result<(), Error> {
        let track_name = CString::new(track_name).unwrap_or_default();
        Error::from_k4a_result_t((self.api_record.funcs.k4a_record_write_custom_track_data)(
            self.handle,
            track_name.as_ptr(),
            device_timestamp_usec,
            custom_data.as_ptr() as *mut u8,
            custom_data.len(),
        ))
        .to_result(())
    }
}

impl Drop for Record<'_> {
    fn drop(&mut self) {
        (self.api_record.funcs.k4a_record_close)(self.handle);
        self.handle = ptr::null_mut();
    }
}
