// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// A list of machine instructions.
#[derive(Debug)]
pub struct SBInstructionList {
    /// The underlying raw `SBInstructionListRef`.
    pub raw: sys::SBInstructionListRef,
}

impl SBInstructionList {
    /// Construct a new `SBInstructionList`.
    pub fn new(raw: sys::SBInstructionListRef) -> SBInstructionList {
        SBInstructionList { raw: raw }
    }

    /// Construct a new `Some(SBInstructionList)` or `None`.
    pub fn maybe(raw: sys::SBInstructionListRef) -> Option<SBInstructionList> {
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
