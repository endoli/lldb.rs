// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBError};

/// A block of data.
#[derive(Debug)]
pub struct SBData {
    /// The underlying raw `SBDataRef`.
    pub raw: sys::SBDataRef,
}

impl SBData {
    /// Construct a new `SBData`.
    pub(crate) fn wrap(raw: sys::SBDataRef) -> SBData {
        SBData { raw }
    }

    /// Construct a new `Some(SBData)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBDataRef) -> Option<SBData> {
        if unsafe { sys::SBDataIsValid(raw) } {
            Some(SBData { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBData` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBDataIsValid(self.raw) }
    }

    /// Get address of the specified offset in this data region
    pub fn get_address(&self, offset: sys::lldb_offset_t) -> Result<sys::lldb_addr_t, SBError> {
        let error = SBError::default();
        let result = unsafe { sys::SBDataGetAddress(self.raw, error.raw, offset) };
        if error.is_success() {
            Ok(result)
        } else {
            Err(error)
        }
    }

    /// Reads the data at specified offset to the buffer.
    fn read_raw_data(&self, offset: sys::lldb_offset_t, buffer: &mut [u8]) -> Result<(), SBError> {
        let error = SBError::default();
        unsafe {
            sys::SBDataReadRawData(
                self.raw,
                error.raw,
                offset,
                buffer.as_mut_ptr() as *mut _,
                buffer.len(),
            );
        }
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

impl Clone for SBData {
    fn clone(&self) -> SBData {
        SBData {
            raw: unsafe { sys::CloneSBData(self.raw) },
        }
    }
}

impl Drop for SBData {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBData(self.raw) };
    }
}

unsafe impl Send for SBData {}
unsafe impl Sync for SBData {}
