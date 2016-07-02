// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use super::stream::SBStream;
use sys;

/// A list of machine instructions.
pub struct SBInstructionList {
    /// The underlying raw `SBInstructionListRef`.
    pub raw: sys::SBInstructionListRef,
}

impl SBInstructionList {
    /// Construct a new `SBInstructionList`.
    pub fn wrap(raw: sys::SBInstructionListRef) -> SBInstructionList {
        SBInstructionList { raw: raw }
    }

    /// Construct a new `Some(SBInstructionList)` or `None`.
    pub fn maybe_wrap(raw: sys::SBInstructionListRef) -> Option<SBInstructionList> {
        if unsafe { sys::SBInstructionListIsValid(raw) != 0 } {
            Some(SBInstructionList { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBInstructionList` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBInstructionListIsValid(self.raw) != 0 }
    }
}

impl fmt::Debug for SBInstructionList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBInstructionListGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBInstructionList {{ {} }}", stream.data())
    }
}

impl Drop for SBInstructionList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBInstructionList(self.raw) };
    }
}
