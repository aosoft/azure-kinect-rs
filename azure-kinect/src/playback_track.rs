use crate::playback::Playback;
use crate::utility::*;
use crate::*;
use azure_kinect_sys::k4arecord::k4a_record_video_settings_t;

pub struct PlaybackTrack<'a> {
    pub(crate) playback: &'a Playback<'a>,
    name: std::ffi::CString,
}

impl PlaybackTrack<'_> {
    pub(crate) fn new<'a>(playback: &'a Playback, name: std::ffi::CString) -> PlaybackTrack<'a> {
        PlaybackTrack {
            playback: playback,
            name: name,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.to_str().unwrap_or_default()
    }

    /// Checks whether a track with the given track name exists in the playback file.
    pub fn check_exists(&self) -> bool {
        unsafe {
            (self.playback.factory.api_record.funcs.k4a_playback_check_track_exists)(
                self.playback.handle, self.name.as_ptr()
            )
        }
    }

    /// Checks whether a track is one of the built-in tracks: "COLOR", "DEPTH", etc...
    pub fn is_builtin(&self) -> bool {
        unsafe {
            (self.playback.factory.api_record.funcs.k4a_playback_track_is_builtin)(
                self.playback.handle,
                self.name.as_ptr(),
            )
        }
    }

    /// Gets the video-specific track information for a particular video track.
    pub fn get_video_settings(&self) -> Result<k4a_record_video_settings_t, Error> {
        let mut settings = k4a_record_video_settings_t::default();
        Error::from_k4a_result_t(unsafe {
            (self.playback.factory.api_record.funcs.k4a_playback_track_get_video_settings)(
                self.playback.handle,
                self.name.as_ptr(),
                &mut settings,
            )
        })
        .to_result(settings)
    }

    /// Gets the codec id string for a particular track.
    pub fn get_codec_id(&self) -> Result<String, Error> {
        get_k4a_string(&|codec_id, codec_id_size| unsafe {
            (self.playback.factory.api_record.funcs.k4a_playback_track_get_codec_id)(
                self.playback.handle,
                self.name.as_ptr(),
                codec_id,
                codec_id_size,
            )
        })
    }

    /// Gets the codec context for a particular track.
    pub fn get_codec_context(&self) -> Result<Vec<u8>, Error> {
        get_k4a_binary_data(&|codec_context, codec_context_size| unsafe {
            (self.playback.factory.api_record.funcs.k4a_playback_track_get_codec_context)(
                self.playback.handle,
                self.name.as_ptr(),
                codec_context,
                codec_context_size,
            )
        })
    }
}
