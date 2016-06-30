// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// The value of a variable, register or expression.
#[derive(Debug)]
pub struct SBValue {
    /// The underlying raw `SBValueRef`.
    pub raw: sys::SBValueRef,
}

impl SBValue {
    /// Construct a new `SBValue`.
    pub fn wrap(raw: sys::SBValueRef) -> SBValue {
        SBValue { raw: raw }
    }

    /// Construct a new `Some(SBValue)` or `None`.
    pub fn maybe_wrap(raw: sys::SBValueRef) -> Option<SBValue> {
        if unsafe { sys::SBValueIsValid(raw) != 0 } {
            Some(SBValue { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBValue` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBValueIsValid(self.raw) != 0 }
    }
}

impl Drop for SBValue {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBValue(self.raw) };
    }
}
