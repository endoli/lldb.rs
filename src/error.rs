// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, ErrorType, SBStream};
use std::fmt;
use std::{error::Error, ffi::CStr};

/// A container for holding any error code and an error message.
///
/// An `SBError` is used to indicate whether or not an operation
/// has succeeded or failed, along with an indication of why it
/// has failed.
///
/// To check if the operation has succeeded, use [`SBError::is_success()`].
/// If it has failed, then [`SBError::is_failure()`] will return true,
/// and more information about the error can be obtained from
/// [`SBError::error()`], [`SBError::error_string()`], and
/// [`SBError::error_type()`].
pub struct SBError {
    /// The underlying raw `SBErrorRef`.
    pub raw: sys::SBErrorRef,
}

impl SBError {
    /// Construct a new `Some(SBError)` or `None`.
    pub fn maybe_wrap(raw: sys::SBErrorRef) -> Option<SBError> {
        if unsafe { sys::SBErrorIsValid(raw) } {
            Some(SBError { raw })
        } else {
            None
        }
    }

    /// Does this error represent a success?
    ///
    /// An error starts out in the success state by default:
    ///
    /// ```
    /// # use lldb::SBError;
    /// let e = SBError::default();
    /// assert!(e.is_success());
    /// ```
    ///
    /// See also:
    ///
    /// * [`SBError::into_result()`]
    /// * [`SBError::is_failure()`]
    pub fn is_success(&self) -> bool {
        unsafe { sys::SBErrorSuccess(self.raw) }
    }

    /// Does this error represent a failure?
    ///
    /// See also:
    ///
    /// * [`SBError::error()`]
    /// * [`SBError::error_string()`]
    /// * [`SBError::error_type()`]
    /// * [`SBError::into_result()`]
    /// * [`SBError::is_success()`]
    pub fn is_failure(&self) -> bool {
        unsafe { sys::SBErrorFail(self.raw) }
    }

    /// Convert to a `Result<(), SBError>`.
    ///
    /// An `SBError` represents either a success or a failure. This method
    /// converts the success variant to `Ok(())` and the error variant
    /// to `Err(self)`.
    ///
    /// ```
    /// # use lldb::SBError;
    /// let e = SBError::default();
    /// // Do something with `e`.
    /// let r = e.into_result();
    /// ```
    ///
    /// See also:
    ///
    /// * [`SBError::error()`]
    /// * [`SBError::error_string()`]
    /// * [`SBError::error_type()`]
    pub fn into_result(self) -> Result<(), SBError> {
        if self.is_success() {
            Ok(())
        } else {
            Err(self)
        }
    }

    /// The underlying error code. Must be interpreted in conjunction
    /// with the error type.
    ///
    /// See also:
    ///
    /// * [`SBError::error_string()`]
    /// * [`SBError::error_type()`]
    pub fn error(&self) -> u32 {
        unsafe { sys::SBErrorGetError(self.raw) }
    }

    /// Any textual error message associated with the error.
    ///
    /// See also:
    ///
    /// * [`SBError::error()`]
    /// * [`SBError::error_type()`]
    pub fn error_string(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBErrorGetCString(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// What type of error is this?
    ///
    /// See also:
    ///
    /// * [`SBError::error()`]
    /// * [`SBError::error_string()`]
    pub fn error_type(&self) -> ErrorType {
        unsafe { sys::SBErrorGetType(self.raw) }
    }
}

impl Clone for SBError {
    fn clone(&self) -> SBError {
        SBError {
            raw: unsafe { sys::CloneSBError(self.raw) },
        }
    }
}

impl Default for SBError {
    fn default() -> SBError {
        SBError::from(unsafe { sys::CreateSBError() })
    }
}

impl fmt::Debug for SBError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBErrorGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBError {{ {} }}", stream.data())
    }
}

impl fmt::Display for SBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_success() {
            write!(f, "SBError representing success")
        } else {
            write!(f, "SBError: {}", self.error_string())
        }
    }
}

impl Error for SBError {}

impl Drop for SBError {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBError(self.raw) };
    }
}

impl From<sys::SBErrorRef> for SBError {
    fn from(raw: sys::SBErrorRef) -> SBError {
        SBError { raw }
    }
}

unsafe impl Send for SBError {}
unsafe impl Sync for SBError {}
