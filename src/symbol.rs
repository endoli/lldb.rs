// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr;
use super::address::SBAddress;
use super::instructionlist::SBInstructionList;
use super::stream::SBStream;
use super::target::SBTarget;
use super::{DisassemblyFlavor, SymbolType};
use sys;

/// The symbol possibly associated with a stack frame.
pub struct SBSymbol {
    /// The underlying raw `SBSymbolRef`.
    pub raw: sys::SBSymbolRef,
}

impl SBSymbol {
    /// Construct a new `SBSymbol`.
    pub fn wrap(raw: sys::SBSymbolRef) -> SBSymbol {
        SBSymbol { raw: raw }
    }

    /// Construct a new `Some(SBSymbol)` or `None`.
    pub fn maybe_wrap(raw: sys::SBSymbolRef) -> Option<SBSymbol> {
        if unsafe { sys::SBSymbolIsValid(raw) != 0 } {
            Some(SBSymbol { raw: raw })
        } else {
            None
        }
    }

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

    ///
    pub fn get_instructions(&self,
                            target: &SBTarget,
                            flavor: DisassemblyFlavor)
                            -> SBInstructionList {
        let flavor = match flavor {
            DisassemblyFlavor::ATT => CString::new("att").unwrap().as_ptr(),
            DisassemblyFlavor::Default => ptr::null(),
            DisassemblyFlavor::Intel => CString::new("intel").unwrap().as_ptr(),
        };
        SBInstructionList::wrap(unsafe {
            sys::SBSymbolGetInstructions2(self.raw, target.raw, flavor)
        })
    }

    /// Get the address that this symbol refers to, if present.
    pub fn start_address(&self) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBSymbolGetStartAddress(self.raw) })
    }

    /// If the symbol has an address and the underlying value has a
    /// non-zero size, this will have the address of the end of
    /// the value.
    ///
    /// Note: It seems unfortunate that if the underlying value is 0-sized,
    /// this will result in `None` rather than the same address as the
    /// `start_address`.
    pub fn end_address(&self) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBSymbolGetEndAddress(self.raw) })
    }

    /// Get the size of the function prologue, in bytes.
    pub fn prologue_byte_size(&self) -> u32 {
        unsafe { sys::SBSymbolGetPrologueByteSize(self.raw) }
    }

    /// What type of symbol is this?
    pub fn symbol_type(&self) -> SymbolType {
        unsafe { sys::SBSymbolGetType(self.raw) }
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

impl fmt::Debug for SBSymbol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBSymbolGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBSymbol {{ {} }}", stream.data())
    }
}

impl Drop for SBSymbol {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBSymbol(self.raw) };
    }
}
