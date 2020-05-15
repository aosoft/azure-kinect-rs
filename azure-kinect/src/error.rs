use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Succeeded,
    Failed,
    TooSmall,
    Timeout,
    Win32Error(u32),
}

impl Error {
    pub(crate) fn to_result<T>(self, ok: T) -> Result<T, Error> {
        match self {
            Error::Succeeded => Ok(ok),
            _ => Err(self),
        }
    }

    pub(crate) fn to_result_fn<T>(self, ok: &dyn Fn() -> T) -> Result<T, Error> {
        match self {
            Error::Succeeded => Ok(ok()),
            _ => Err(self),
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

impl From<k4a_result_t> for Error {
    fn from(s: k4a_result_t) -> Error {
        match s {
            k4a_result_t::K4A_RESULT_SUCCEEDED => Error::Succeeded,
            k4a_result_t::K4A_RESULT_FAILED => Error::Failed,
        }
    }
}

impl From<k4a_buffer_result_t> for Error {
    fn from(s: k4a_buffer_result_t) -> Error {
        match s {
            k4a_buffer_result_t::K4A_BUFFER_RESULT_SUCCEEDED => Error::Succeeded,
            k4a_buffer_result_t::K4A_BUFFER_RESULT_FAILED => Error::Failed,
            k4a_buffer_result_t::K4A_BUFFER_RESULT_TOO_SMALL => Error::TooSmall,
        }
    }
}

impl From<k4a_wait_result_t> for Error {
    fn from(s: k4a_wait_result_t) -> Error {
        match s {
            k4a_wait_result_t::K4A_WAIT_RESULT_SUCCEEDED => Error::Succeeded,
            k4a_wait_result_t::K4A_WAIT_RESULT_FAILED => Error::Failed,
            k4a_wait_result_t::K4A_WAIT_RESULT_TIMEOUT => Error::Timeout,
        }
    }
}
