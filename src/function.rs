// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    sys, DisassemblyFlavor, LanguageType, SBAddress, SBBlock, SBInstructionList, SBStream,
    SBTarget, SBType,
};
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::ptr;

/// A generic function, which can be inlined or not.
pub struct SBFunction {
    /// The underlying raw `SBFunctionRef`.
    pub raw: sys::SBFunctionRef,
}

impl SBFunction {
    /// Construct a new `SBFunction`.
    pub(crate) fn wrap(raw: sys::SBFunctionRef) -> SBFunction {
        SBFunction { raw }
    }

    /// Construct a new `Some(SBFunction)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBFunctionRef) -> Option<SBFunction> {
        if unsafe { sys::SBFunctionIsValid(raw) } {
            Some(SBFunction { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBFunction` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBFunctionIsValid(self.raw) }
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
    pub fn mangled_name(&self) -> Option<&str> {
        unsafe { self.check_null_ptr(sys::SBFunctionGetMangledName(self.raw)) }
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
            sys::SBFunctionGetInstructions2(
                self.raw,
                target.raw,
                flavor.map_or(ptr::null(), |s| s.as_ptr()),
            )
        })
    }

    /// Get the address of the start of this function.
    pub fn start_address(&self) -> SBAddress {
        SBAddress::wrap(unsafe { sys::SBFunctionGetStartAddress(self.raw) })
    }

    /// Get the address of the end of this function.
    pub fn end_address(&self) -> SBAddress {
        SBAddress::wrap(unsafe { sys::SBFunctionGetEndAddress(self.raw) })
    }

    /// Get the size of the function prologue, in bytes.
    pub fn prologue_byte_size(&self) -> u32 {
        unsafe { sys::SBFunctionGetPrologueByteSize(self.raw) }
    }

    /// The return type for this function.
    pub fn return_type(&self) -> SBType {
        SBType::wrap(unsafe { sys::SBFunctionGetType(self.raw) })
    }

    /// Get the top level lexical block for this function.
    pub fn block(&self) -> SBBlock {
        SBBlock::wrap(unsafe { sys::SBFunctionGetBlock(self.raw) })
    }

    /// The language that this function was written in.
    pub fn language(&self) -> LanguageType {
        unsafe { sys::SBFunctionGetLanguage(self.raw) }
    }

    /// Returns true if the function was compiled with optimization.
    ///
    /// Optimization, in this case, is meant to indicate that the debugger
    /// experience may be confusing for the user -- variables optimized away,
    /// stepping jumping between source lines -- and the driver may want to
    /// provide some guidance to the user about this.
    /// Returns false if unoptimized, or unknown.
    pub fn is_optimized(&self) -> bool {
        unsafe { sys::SBFunctionGetIsOptimized(self.raw) }
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

impl Clone for SBFunction {
    fn clone(&self) -> SBFunction {
        SBFunction {
            raw: unsafe { sys::CloneSBFunction(self.raw) },
        }
    }
}

impl fmt::Debug for SBFunction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBFunctionGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBFunction {{ {} }}", stream.data())
    }
}

impl Drop for SBFunction {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBFunction(self.raw) };
    }
}

unsafe impl Send for SBFunction {}
unsafe impl Sync for SBFunction {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBFunction {
    fn name() -> &str {
        self.name()
    }

    fn display_name() -> &str {
        self.display_name()
    }

    fn mangled_name() -> Option<&str> {
        self.mangled_name()
    }

    fn start_address() -> SBAddress {
        self.start_address()
    }

    fn end_address() -> SBAddress {
        self.end_address()
    }

    // TODO(bm) This should be a u32
    fn prologue_byte_size() -> i32 {
        self.prologue_byte_size() as i32
    }

    fn return_type() -> SBType {
        self.return_type()
    }

    fn block() -> SBBlock {
        self.block()
    }

    fn is_optimized() -> bool {
        self.is_optimized()
    }
}
