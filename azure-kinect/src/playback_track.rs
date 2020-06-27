use super::utility::*;
use super::*;
use crate::playback::Playback;
use std::ptr;

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
        (self.playback.factory.k4a_playback_check_track_exists)(self.playback.handle, self.name.as_ptr())
    }

    /// Checks whether a track is one of the built-in tracks: "COLOR", "DEPTH", etc...
    pub fn is_builtin(&self) -> bool {
        (self.playback.factory.k4a_playback_track_is_builtin)(self.playback.handle, self.name.as_ptr())
    }


}
