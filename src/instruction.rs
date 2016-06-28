// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::address::SBAddress;
use super::data::SBData;
use super::target::SBTarget;
use sys;

/// A machine instruction.
#[derive(Debug)]
pub struct SBInstruction {
    /// The underlying raw `SBInstructionRef`.
    pub raw: sys::SBInstructionRef,
}

impl SBInstruction {
    /// Check whether or not this is a valid `SBInstruction` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBInstructionIsValid(self.raw) != 0 }
    }

    /// Get the address of the instruction.
    pub fn address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBInstructionGetAddress(self.raw) } }
    }

    /// Get the address class for the address of the instruction.
    pub fn address_class(&self) -> sys::LLDBAddressClass {
        unsafe { sys::SBInstructionGetAddressClass(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn data(&self, target: &SBTarget) -> SBData {
        SBData { raw: unsafe { sys::SBInstructionGetData(self.raw, target.raw) } }
    }

    #[allow(missing_docs)]
    pub fn byte_size(&self) -> u32 {
        unsafe { sys::SBInstructionGetByteSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_branch(&self) -> bool {
        unsafe { sys::SBInstructionDoesBranch(self.raw) != 0 }
    }
}
