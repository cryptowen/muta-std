use alloc::string::FromUtf8Error;

/// Syscall errors
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum SysError {
    /// Buffer length is not enough, error contains actual data length
    LengthNotEnough(u64),
    FromUtf8Error,
}

impl From<FromUtf8Error> for SysError {
    fn from(_err: FromUtf8Error) -> Self {
        Self::FromUtf8Error
    }
}
