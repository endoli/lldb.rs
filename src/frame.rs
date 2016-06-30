// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use super::address::SBAddress;
use super::block::SBBlock;
use super::compileunit::SBCompileUnit;
use super::function::SBFunction;
use super::lineentry::SBLineEntry;
use super::module::SBModule;
use super::symbol::SBSymbol;
use super::thread::SBThread;
use sys;

/// One of the stack frames associated with a thread.
#[derive(Debug)]
pub struct SBFrame {
    /// The underlying raw `SBFrameRef`.
    pub raw: sys::SBFrameRef,
}

impl SBFrame {
    /// Construct a new `SBFrame`.
    pub fn wrap(raw: sys::SBFrameRef) -> SBFrame {
        SBFrame { raw: raw }
    }

    /// Construct a new `Some(SBFrame)` or `None`.
    pub fn maybe_wrap(raw: sys::SBFrameRef) -> Option<SBFrame> {
        if unsafe { sys::SBFrameIsValid(raw) != 0 } {
            Some(SBFrame { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBFrame` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBFrameIsValid(self.raw) != 0 }
    }

    /// The zero-based stack frame index for this frame.
    pub fn frame_id(&self) -> u32 {
        unsafe { sys::SBFrameGetFrameID(self.raw) }
    }

    /// The program counter (PC) as a section offset address (`SBAddress`).
    pub fn pc_address(&self) -> SBAddress {
        SBAddress::wrap(unsafe { sys::SBFrameGetPCAddress(self.raw) })
    }

    /// The `SBModule` for this stack frame.
    pub fn module(&self) -> SBModule {
        SBModule::wrap(unsafe { sys::SBFrameGetModule(self.raw) })
    }

    /// The `SBCompileUnit` for this stack frame.
    pub fn compile_unit(&self) -> SBCompileUnit {
        SBCompileUnit::wrap(unsafe { sys::SBFrameGetCompileUnit(self.raw) })
    }

    /// The `SBFunction` for this stack frame.
    pub fn function(&self) -> SBFunction {
        SBFunction::wrap(unsafe { sys::SBFrameGetFunction(self.raw) })
    }

    /// The `SBSymbol` for this stack frame.
    pub fn symbol(&self) -> SBSymbol {
        SBSymbol::wrap(unsafe { sys::SBFrameGetSymbol(self.raw) })
    }

    /// Get the deepest block that contains the frame PC.
    pub fn block(&self) -> SBBlock {
        SBBlock::wrap(unsafe { sys::SBFrameGetBlock(self.raw) })
    }

    /// Get the appropriate function name for this frame. Inlined functions in
    /// LLDB are represented by blocks that have inlined function information, so
    /// just looking at the `SBFunction` or `SBSymbol` for a frame isn't enough.
    /// This function will return the appropriate function, symbol or inlined
    /// function name for the frame.
    ///
    /// This function returns:
    ///
    /// * the name of the inlined function (if there is one)
    /// * the name of the concrete function (if there is one)
    /// * the name of the symbol (if there is one)
    /// * NULL
    ///
    /// See also `is_inlined`.
    pub fn function_name(&self) -> Option<&str> {
        unsafe {
            match CStr::from_ptr(sys::SBFrameGetFunctionName(self.raw)).to_str() {
                Ok(s) => Some(s),
                _ => None,
            }
        }
    }

    #[allow(missing_docs)]
    pub fn display_function_name(&self) -> Option<&str> {
        unsafe {
            match CStr::from_ptr(sys::SBFrameGetDisplayFunctionName(self.raw)).to_str() {
                Ok(s) => Some(s),
                _ => None,
            }
        }
    }

    /// Return `true` if this frame represents an inlined function.
    pub fn is_inlined(&self) -> bool {
        unsafe { sys::SBFrameIsInlined(self.raw) != 0 }
    }

    /// Gets the lexical block that defines the stack frame. Another way to think
    /// of this is it will return the block that contains all of the variables
    /// for a stack frame. Inlined functions are represented as `SBBlock` objects
    /// that have inlined function information: the name of the inlined function,
    /// where it was called from. The block that is returned will be the first
    /// block at or above the block for the PC (`SBFrame::block()`) that defines
    /// the scope of the frame. When a function contains no inlined functions,
    /// this will be the top most lexical block that defines the function.
    /// When a function has inlined functions and the PC is currently
    /// in one of those inlined functions, this method will return the inlined
    /// block that defines this frame. If the PC isn't currently in an inlined
    /// function, the lexical block that defines the function is returned.
    pub fn frame_block(&self) -> SBBlock {
        SBBlock::wrap(unsafe { sys::SBFrameGetFrameBlock(self.raw) })
    }

    /// The line table entry (`SBLineEntry`) for this stack frame.
    pub fn line_entry(&self) -> Option<SBLineEntry> {
        SBLineEntry::maybe_wrap(unsafe { sys::SBFrameGetLineEntry(self.raw) })
    }

    /// The thread that is executing this stack frame.
    pub fn thread(&self) -> SBThread {
        SBThread::wrap(unsafe { sys::SBFrameGetThread(self.raw) })
    }
}

impl Drop for SBFrame {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBFrame(self.raw) };
    }
}
