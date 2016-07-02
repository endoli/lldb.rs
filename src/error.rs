// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use super::stream::SBStream;
use super::ErrorType;
use sys;

/// A container for holding any error code.
pub struct SBError {
    /// The underlying raw `SBErrorRef`.
    pub raw: sys::SBErrorRef,
}

impl SBError {
    /// Construct a new `SBError`.
    pub fn new() -> SBError {
        SBError::wrap(unsafe { sys::CreateSBError() })
    }

    /// Construct a new `SBError`.
    pub fn wrap(raw: sys::SBErrorRef) -> SBError {
        SBError { raw: raw }
    }

    /// Construct a new `Some(SBError)` or `None`.
    pub fn maybe_wrap(raw: sys::SBErrorRef) -> Option<SBError> {
        if unsafe { sys::SBErrorIsValid(raw) != 0 } {
            Some(SBError { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBError` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBErrorIsValid(self.raw) != 0 }
    }

    /// Any textual error message associated with the error.
    pub fn error_string(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBErrorGetCString(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Does this error represent a failure?
    pub fn is_failure(&self) -> bool {
        unsafe { sys::SBErrorFail(self.raw) != 0 }
    }

    /// Does this error represent a success?
    pub fn is_success(&self) -> bool {
        unsafe { sys::SBErrorSuccess(self.raw) != 0 }
    }

    /// The underlying error code. Must be interpreted in conjunction
    /// with the error type.
    pub fn error(&self) -> u32 {
        unsafe { sys::SBErrorGetError(self.raw) }
    }

    /// What type of error is this?
    pub fn error_type(&self) -> ErrorType {
        unsafe { sys::SBErrorGetType(self.raw) }
    }
}

impl Default for SBError {
    fn default() -> SBError {
        SBError::new()
    }
}

impl fmt::Debug for SBError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBErrorGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBError {{ {} }}", stream.data())
    }
}

impl Drop for SBError {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBError(self.raw) };
    }
}
