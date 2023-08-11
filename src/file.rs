// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBError};
use libc::FILE;
use std::ffi::CString;

/// Represents a file.
pub struct SBFile {
    /// The underlying raw `SBFileRef`.
    pub raw: sys::SBFileRef,
}

impl SBFile {
    /// Construct a new `SBFile`.
    pub(crate) fn wrap(raw: sys::SBFileRef) -> SBFile {
        SBFile { raw }
    }

    /// Construct a new `Some(SBFile)` or `None`.
    #[allow(dead_code)]
    pub(crate) fn maybe_wrap(raw: sys::SBFileRef) -> Option<SBFile> {
        if unsafe { sys::SBFileIsValid(raw) } {
            Some(SBFile { raw })
        } else {
            None
        }
    }

    /// Create an `SBFile` from a [`libc::FILE`].
    ///
    /// # Safety
    ///
    /// The `file` pointer must be valid.
    pub unsafe fn from_file(&self, file: *mut FILE, transfer_ownership: bool) -> SBFile {
        SBFile::wrap(sys::CreateSBFile2(file, transfer_ownership))
    }

    /// Create an `SBFile` from a file descriptor.
    pub fn from_fd(&self, fd: i32, mode: &str, transfer_ownership: bool) -> SBFile {
        let cmode = CString::new(mode).unwrap();
        SBFile::wrap(unsafe { sys::CreateSBFile3(fd, cmode.as_ptr(), transfer_ownership) })
    }

    /// Check whether or not this is a valid `SBFile` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBFileIsValid(self.raw) }
    }

    /// Read data from the file.
    ///
    /// If successful, the result will contain the number of bytes read.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, SBError> {
        let mut bytes_read: usize = 0;
        let e = SBError::wrap(unsafe {
            sys::SBFileRead(self.raw, buf.as_mut_ptr(), buf.len(), &mut bytes_read)
        });
        if e.is_success() {
            Ok(bytes_read)
        } else {
            Err(e)
        }
    }

    /// Write data to the file.
    ///
    /// If successful, the result will contain the number of bytes written.
    pub fn write(&self, buf: &[u8]) -> Result<usize, SBError> {
        let mut bytes_written: usize = 0;
        let e = SBError::wrap(unsafe {
            sys::SBFileWrite(self.raw, buf.as_ptr(), buf.len(), &mut bytes_written)
        });
        if e.is_success() {
            Ok(bytes_written)
        } else {
            Err(e)
        }
    }

    /// Flush the file.
    pub fn flush(&self) -> Result<(), SBError> {
        SBError::wrap(unsafe { sys::SBFileFlush(self.raw) }).into_result()
    }

    /// Close the file.
    pub fn close(&self) -> Result<(), SBError> {
        SBError::wrap(unsafe { sys::SBFileClose(self.raw) }).into_result()
    }
}
