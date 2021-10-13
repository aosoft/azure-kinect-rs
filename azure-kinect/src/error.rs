#![allow(non_upper_case_globals)]

use azure_kinect_sys::k4a::*;
use azure_kinect_sys::k4arecord::k4a_stream_result_t;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Succeeded,
    Failed,
    TooSmall,
    Timeout,
    Win32Error(u32),
    Eof,
}

impl Error {
    pub(crate) fn from_k4a_result_t(s: k4a_result_t) -> Error {
        match s {
            k4a_result_t_K4A_RESULT_SUCCEEDED => Error::Succeeded,
            k4a_result_t_K4A_RESULT_FAILED => Error::Failed,
            _ => Error::Failed,
        }
    }

    pub(crate) fn from_k4a_buffer_result_t(s: k4a_buffer_result_t) -> Error {
        match s {
            k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED => Error::Succeeded,
            k4a_buffer_result_t_K4A_BUFFER_RESULT_FAILED => Error::Failed,
            k4a_buffer_result_t_K4A_BUFFER_RESULT_TOO_SMALL => Error::TooSmall,
            _ => Error::Failed,
        }
    }

    pub(crate) fn from_k4a_wait_result_t(s: k4a_wait_result_t) -> Error {
        match s {
            k4a_wait_result_t_K4A_WAIT_RESULT_SUCCEEDED => Error::Succeeded,
            k4a_wait_result_t_K4A_WAIT_RESULT_FAILED => Error::Failed,
            k4a_wait_result_t_K4A_WAIT_RESULT_TIMEOUT => Error::Timeout,
            _ => Error::Failed,
        }
    }

    pub(crate) fn from_k4a_stream_result_t(s: k4a_stream_result_t) -> Error {
        match s {
            azure_kinect_sys::k4arecord::k4a_stream_result_t_K4A_STREAM_RESULT_SUCCEEDED => {
                Error::Succeeded
            }
            azure_kinect_sys::k4arecord::k4a_stream_result_t_K4A_STREAM_RESULT_FAILED => {
                Error::Failed
            }
            azure_kinect_sys::k4arecord::k4a_stream_result_t_K4A_STREAM_RESULT_EOF => Error::Eof,
            _ => Error::Failed,
        }
    }

    pub(crate) fn to_result<T>(self, ok: T) -> Result<T, Error> {
        match self {
            Error::Succeeded => Ok(ok),
            _ => Err(self),
        }
    }

    pub(crate) fn to_result_fn<T, F: FnOnce() -> T>(self, ok: F) -> Result<T, Error> {
        match self {
            Error::Succeeded => Ok(ok()),
            _ => Err(self),
        }
    }
}

impl From<azure_kinect_sys::Error> for Error {
    fn from(e: azure_kinect_sys::Error) -> Self {
        match e {
            azure_kinect_sys::Error::Failed => Self::Failed,
            azure_kinect_sys::Error::Win32Error(code) => Self::Win32Error(code),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(crate) trait ToResult: Sized {
    fn to_result(&self) -> Result<Self, Error>;
}
