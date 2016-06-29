// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use super::address::SBAddress;
use sys;

/// A generic function, which can be inlined or not.
#[derive(Debug)]
pub struct SBFunction {
    /// The underlying raw `SBFunctionRef`.
    pub raw: sys::SBFunctionRef,
}

impl SBFunction {
    /// Construct a new `SBFunction`.
    pub fn new(raw: sys::SBFunctionRef) -> SBFunction {
        SBFunction { raw: raw }
    }

    /// Construct a new `Some(SBFunction)` or `None`.
    pub fn maybe(raw: sys::SBFunctionRef) -> Option<SBFunction> {
        if unsafe { sys::SBFunctionIsValid(raw) != 0 } {
            Some(SBFunction { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBFunction` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBFunctionIsValid(self.raw) != 0 }
    }

    /// The name of this function.
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBFunctionGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The display name for the function, as it should be seen in a UI.
    pub fn display_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBFunctionGetDisplayName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The mangled (linkage) name for this function.
    pub fn mangled_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBFunctionGetMangledName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Get the address of the start of this function.
    pub fn start_address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBFunctionGetStartAddress(self.raw) } }
    }

    /// Get the address of the end of this function.
    pub fn end_address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBFunctionGetEndAddress(self.raw) } }
    }

    /// Get the size of the function prologue, in bytes.
    pub fn prologue_byte_size(&self) -> u32 {
        unsafe { sys::SBFunctionGetPrologueByteSize(self.raw) }
    }

    /// Returns true if the function was compiled with optimization.
    ///
    /// Optimization, in this case, is meant to indicate that the debugger
    /// experience may be confusing for the user -- variables optimized away,
    /// stepping jumping between source lines -- and the driver may want to
    /// provide some guidance to the user about this.
    /// Returns false if unoptimized, or unknown.
    pub fn is_optimized(&self) -> bool {
        unsafe { sys::SBFunctionGetIsOptimized(self.raw) != 0 }
    }
}
