/*
 * 
 * This file is part of the rust-cp210x-runtime project.
 * 
 * Licensed under the MIT License. You may obtain a copy of the License at
 * 
 *     https://opensource.org/licenses/MIT
 * 
 * This code is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement.
 * 
 * Note: This project is not affiliated with, endorsed by, or in any way associated with Silicon Labs. For more information about Silicon Labs and their products, please visit their official website at https://www.silabs.com.
 */
use std::{fmt, result};

/// A result of a function that may return a `CP210xError`.
pub type Result<T> = result::Result<T, CP210xError>;

/// Error returned by the library 
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CP210xError {
    /// Invalid handle 
    InvalidHandle,

    /// Invalid parameter 
    InvalidParameter,

    /// Device I/O failed
    DeviceIoFailed,

    /// Function not supported or umimplemted in this platform
    FunctionNotSupported,

    /// Global data error
    GlobalDataError,

    /// File error
    FileError,

    /// Command failed
    CommandFailed,

    /// Invalid access (it may have been insufficient permissions)
    InvalidAccessType,

    /// Device not found
    DeviceNotFound,

    /// Other error
    Other,
}

impl fmt::Display for CP210xError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        fmt.write_str(match self {
            CP210xError::InvalidHandle => "Invalid Handle",
            CP210xError::InvalidParameter => "Invalid Parameter",
            CP210xError::DeviceIoFailed => "Device Input/Output Failed",
            CP210xError::FunctionNotSupported => {
                "Function not supported or unimplemented on this platform"
            }
            CP210xError::GlobalDataError => "Data error (Corrupted or Unretrievable)",
            CP210xError::FileError => "File not found or Inaccessible",
            CP210xError::CommandFailed => "Command Failed",
            CP210xError::InvalidAccessType => {
                "Invalid access (it may have been insufficient permissions)"
            }
            CP210xError::DeviceNotFound => "Device not found",
            CP210xError::Other => "Other error",
        })
    }
}

impl From<rusb::Error> for CP210xError {
    fn from(e: rusb::Error) -> Self {
        match e {
            rusb::Error::Io => Self::DeviceIoFailed,
            rusb::Error::InvalidParam => Self::InvalidParameter,
            rusb::Error::Access => Self::InvalidAccessType,
            rusb::Error::NoDevice => Self::DeviceNotFound,
            rusb::Error::NotFound => Self::DeviceNotFound,
            rusb::Error::Busy => Self::Other,
            rusb::Error::Timeout => Self::CommandFailed,
            rusb::Error::Overflow => Self::GlobalDataError,
            rusb::Error::Pipe => Self::GlobalDataError,
            rusb::Error::Interrupted => Self::GlobalDataError,
            rusb::Error::NoMem => Self::GlobalDataError,
            rusb::Error::NotSupported => Self::FunctionNotSupported,
            rusb::Error::BadDescriptor => Self::CommandFailed,
            rusb::Error::Other => Self::Other,
        }
    }
}

impl std::error::Error for CP210xError {}
