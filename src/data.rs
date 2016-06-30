// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// A block of data.
#[derive(Debug)]
pub struct SBData {
    /// The underlying raw `SBDataRef`.
    pub raw: sys::SBDataRef,
}

impl SBData {
    /// Construct a new `SBData`.
    pub fn new(raw: sys::SBDataRef) -> SBData {
        SBData { raw: raw }
    }

    /// Construct a new `Some(SBData)` or `None`.
    pub fn maybe(raw: sys::SBDataRef) -> Option<SBData> {
        if unsafe { sys::SBDataIsValid(raw) != 0 } {
            Some(SBData { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBData` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBDataIsValid(self.raw) != 0 }
    }
}

impl Drop for SBData {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBData(self.raw) };
    }
}
