#[derive(Clone, Copy, Debug)]
pub enum Error {
    Succeded,
    Failed,
    TooSmall,
    Timeout,
    Win32Error(u32),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub(crate) trait ToResult: Sized {
    fn to_result(&self) -> Result<Self, Error>;
}
