// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, DisassemblyFlavor, SBAddress, SBInstructionList, SBStream, SBTarget, SymbolType};
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::ptr;

/// The symbol possibly associated with a stack frame.
pub struct SBSymbol {
    /// The underlying raw `SBSymbolRef`.
    pub raw: sys::SBSymbolRef,
}

impl SBSymbol {
    /// Construct a new `SBSymbol`.
    pub(crate) fn wrap(raw: sys::SBSymbolRef) -> SBSymbol {
        SBSymbol { raw }
    }

    /// Construct a new `Some(SBSymbol)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBSymbolRef) -> Option<SBSymbol> {
        if unsafe { sys::SBSymbolIsValid(raw) } {
            Some(SBSymbol { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBSymbol` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSymbolIsValid(self.raw) }
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
    pub fn mangled_name(&self) -> Option<&str> {
        unsafe { self.check_null_ptr(sys::SBSymbolGetMangledName(self.raw)) }
    }

    #[allow(missing_docs)]
    pub fn get_instructions(
        &self,
        target: &SBTarget,
        flavor: DisassemblyFlavor,
    ) -> SBInstructionList {
        let flavor = match flavor {
            DisassemblyFlavor::ATT => CString::new("att").ok(),
            DisassemblyFlavor::Default => None,
            DisassemblyFlavor::Intel => CString::new("intel").ok(),
        };
        SBInstructionList::wrap(unsafe {
            sys::SBSymbolGetInstructions2(
                self.raw,
                target.raw,
                flavor.map_or(ptr::null(), |s| s.as_ptr()),
            )
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
        unsafe { sys::SBSymbolIsExternal(self.raw) }
    }

    /// Is this symbol synthetically created from information in the
    /// module that contains it?
    pub fn is_synthetic(&self) -> bool {
        unsafe { sys::SBSymbolIsSynthetic(self.raw) }
    }

    unsafe fn check_null_ptr(&self, ptr: *const c_char) -> Option<&str> {
        if !ptr.is_null() {
            match CStr::from_ptr(ptr).to_str() {
                Ok(s) => Some(s),
                _ => panic!("Invalid string?"),
            }
        } else {
            None
        }
    }
}

impl Clone for SBSymbol {
    fn clone(&self) -> SBSymbol {
        SBSymbol {
            raw: unsafe { sys::CloneSBSymbol(self.raw) },
        }
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

unsafe impl Send for SBSymbol {}
unsafe impl Sync for SBSymbol {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBSymbol {
    fn name() -> &str {
        self.name()
    }

    fn display_name() -> &str {
        self.display_name()
    }

    fn mangled_name() -> Option<&str> {
        self.mangled_name()
    }

    fn start_address() -> Option<SBAddress> {
        self.start_address()
    }

    fn end_address() -> Option<SBAddress> {
        self.end_address()
    }

    // TODO(bm) This should be a u32
    fn prologue_byte_size() -> i32 {
        self.prologue_byte_size() as i32
    }

    fn is_external() -> bool {
        self.is_external()
    }

    fn is_synthetic() -> bool {
        self.is_synthetic()
    }
}
