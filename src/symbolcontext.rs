// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    sys, SBAddress, SBBlock, SBCompileUnit, SBFunction, SBLineEntry, SBModule, SBStream, SBSymbol,
};
use std::fmt;

/// A container that stores various debugger related info.
pub struct SBSymbolContext {
    /// The underlying raw `SBSymbolContextRef`.
    pub raw: sys::SBSymbolContextRef,
}

impl SBSymbolContext {
    /// Construct a new `SBSymbolContext`.
    pub(crate) fn wrap(raw: sys::SBSymbolContextRef) -> SBSymbolContext {
        SBSymbolContext { raw }
    }

    /// Construct a new `Some(SBSymbolContext)` or `None`.
    #[allow(dead_code)]
    pub(crate) fn maybe_wrap(raw: sys::SBSymbolContextRef) -> Option<SBSymbolContext> {
        if unsafe { sys::SBSymbolContextIsValid(raw) } {
            Some(SBSymbolContext { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBSymbolContext` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSymbolContextIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn module(&self) -> SBModule {
        SBModule::wrap(unsafe { sys::SBSymbolContextGetModule(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn compile_unit(&self) -> SBCompileUnit {
        SBCompileUnit::wrap(unsafe { sys::SBSymbolContextGetCompileUnit(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn function(&self) -> SBFunction {
        SBFunction::wrap(unsafe { sys::SBSymbolContextGetFunction(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn block(&self) -> SBBlock {
        SBBlock::wrap(unsafe { sys::SBSymbolContextGetBlock(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn line_entry(&self) -> Option<SBLineEntry> {
        SBLineEntry::maybe_wrap(unsafe { sys::SBSymbolContextGetLineEntry(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn symbol(&self) -> SBSymbol {
        SBSymbol::wrap(unsafe { sys::SBSymbolContextGetSymbol(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn parent_of_inlined_scope(
        &self,
        curr_frame_pc: &SBAddress,
        parent_frame_addr: &SBAddress,
    ) -> SBSymbolContext {
        SBSymbolContext::wrap(unsafe {
            sys::SBSymbolContextGetParentOfInlinedScope(
                self.raw,
                curr_frame_pc.raw,
                parent_frame_addr.raw,
            )
        })
    }
}

impl Clone for SBSymbolContext {
    fn clone(&self) -> SBSymbolContext {
        SBSymbolContext {
            raw: unsafe { sys::CloneSBSymbolContext(self.raw) },
        }
    }
}

impl fmt::Debug for SBSymbolContext {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBSymbolContextGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBSymbolContext {{ {} }}", stream.data())
    }
}

impl Drop for SBSymbolContext {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBSymbolContext(self.raw) };
    }
}

unsafe impl Send for SBSymbolContext {}
unsafe impl Sync for SBSymbolContext {}

#[cfg(feature = "graphql")]
#[graphql_object]
impl SBSymbolContext {
    fn is_valid() -> bool {
        self.is_valid()
    }

    fn module() -> SBModule {
        self.module()
    }

    fn compile_unit() -> SBCompileUnit {
        self.compile_unit()
    }

    fn function() -> SBFunction {
        self.function()
    }

    fn block() -> SBBlock {
        self.block()
    }

    fn line_entry() -> Option<SBLineEntry> {
        self.line_entry()
    }

    fn symbol() -> SBSymbol {
        self.symbol()
    }
}
