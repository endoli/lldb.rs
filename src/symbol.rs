// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use super::address::SBAddress;
use sys;

/// The symbol possibly associated with a stack frame.
#[derive(Debug)]
pub struct SBSymbol {
    /// The underlying raw `SBSymbolRef`.
    pub raw: sys::SBSymbolRef,
}

impl SBSymbol {
    /// Check whether or not this is a valid `SBSymbol` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSymbolIsValid(self.raw) != 0 }
    }

    /// The name of this function.
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBSymbolGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The display name for the function, as it should be seen in a UI.
    pub fn display_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBSymbolGetDisplayName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The mangled (linkage) name for this function.
    pub fn mangled_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBSymbolGetMangledName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Get the address of the start of this function.
    pub fn start_address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBSymbolGetStartAddress(self.raw) } }
    }

    /// Get the address of the end of this function.
    pub fn end_address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBSymbolGetEndAddress(self.raw) } }
    }

    /// Get the size of the function prologue, in bytes.
    pub fn prologue_byte_size(&self) -> u32 {
        unsafe { sys::SBSymbolGetPrologueByteSize(self.raw) }
    }

    /// What type of symbol is this?
    pub fn symbol_type(&self) -> sys::LLDBSymbolType {
        unsafe { sys::SBSymbolGetType(self.raw) as sys::LLDBSymbolType }
    }

    /// Is this symbol externally visible (exported) from the module that
    /// contains it?
    pub fn is_external(&self) -> bool {
        unsafe { sys::SBSymbolIsExternal(self.raw) != 0 }
    }

    /// Is this symbol synthetically created from information in the
    /// module that contains it?
    pub fn is_synthetic(&self) -> bool {
        unsafe { sys::SBSymbolIsSynthetic(self.raw) != 0 }
    }
}
