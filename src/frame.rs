// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{CStr, CString};
use std::fmt;
use super::address::SBAddress;
use super::block::SBBlock;
use super::compileunit::SBCompileUnit;
use super::expressionoptions::SBExpressionOptions;
use super::function::SBFunction;
use super::lineentry::SBLineEntry;
use super::module::SBModule;
use super::stream::SBStream;
use super::symbol::SBSymbol;
use super::thread::SBThread;
use super::value::SBValue;
use super::valuelist::SBValueList;
use super::variablesoptions::SBVariablesOptions;
use super::lldb_addr_t;
use sys;

/// One of the stack frames associated with a thread.
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

    /// Get the Canonical Frame Address for this stack frame.
    ///
    /// This is the DWARF standard's definition of a CFA, a
    /// stack address that remains constant throughout the
    /// lifetime of the function.
    pub fn cfa(&self) -> Option<lldb_addr_t> {
        let cfa = unsafe { sys::SBFrameGetCFA(self.raw) };
        if cfa != u64::max_value() {
            Some(cfa)
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn pc(&self) -> lldb_addr_t {
        unsafe { sys::SBFrameGetPC(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_pc(&mut self, new_pc: lldb_addr_t) -> bool {
        unsafe { sys::SBFrameSetPC(self.raw, new_pc) != 0 }
    }

    #[allow(missing_docs)]
    pub fn sp(&self) -> lldb_addr_t {
        unsafe { sys::SBFrameGetSP(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn fp(&self) -> lldb_addr_t {
        unsafe { sys::SBFrameGetFP(self.raw) }
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

    /// Evaluate an expression within the context of this frame.
    pub fn evaluate_expression(&self, expression: &str, options: &SBExpressionOptions) -> SBValue {
        let expression = CString::new(expression).unwrap();
        SBValue::wrap(unsafe {
                          sys::SBFrameEvaluateExpression(self.raw, expression.as_ptr(), options.raw)
                      })
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

    #[allow(missing_docs)]
    pub fn disassemble(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBFrameDisassemble(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn variables(&self, options: &SBVariablesOptions) -> SBValueList {
        SBValueList::wrap(unsafe { sys::SBFrameGetVariables(self.raw, options.raw) })
    }

    #[allow(missing_docs)]
    pub fn all_variables(&self) -> SBValueList {
        let mut options = SBVariablesOptions::new();
        options.set_include_arguments(true);
        options.set_include_locals(true);
        options.set_include_statics(true);
        options.set_in_scope_only(true);
        self.variables(&options)
    }

    #[allow(missing_docs)]
    pub fn arguments(&self) -> SBValueList {
        let mut options = SBVariablesOptions::new();
        options.set_include_arguments(true);
        options.set_include_locals(false);
        options.set_include_statics(false);
        options.set_in_scope_only(false);
        self.variables(&options)
    }

    #[allow(missing_docs)]
    pub fn locals(&self) -> SBValueList {
        let mut options = SBVariablesOptions::new();
        options.set_include_arguments(false);
        options.set_include_locals(true);
        options.set_include_statics(false);
        options.set_in_scope_only(false);
        self.variables(&options)
    }

    #[allow(missing_docs)]
    pub fn statics(&self) -> SBValueList {
        let mut options = SBVariablesOptions::new();
        options.set_include_arguments(false);
        options.set_include_locals(false);
        options.set_include_statics(true);
        options.set_in_scope_only(false);
        self.variables(&options)
    }

    #[allow(missing_docs)]
    pub fn registers(&self) -> SBValueList {
        SBValueList::wrap(unsafe { sys::SBFrameGetRegisters(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn find_register(&self, name: &str) -> Option<SBValue> {
        let name = CString::new(name).unwrap();
        SBValue::maybe_wrap(unsafe { sys::SBFrameFindRegister(self.raw, name.as_ptr()) })
    }

    #[allow(missing_docs)]
    pub fn parent_frame(&self) -> Option<SBFrame> {
        let thread = self.thread();
        let parent_idx = self.frame_id() + 1;
        if parent_idx < unsafe { sys::SBThreadGetNumFrames(thread.raw) } {
            SBFrame::maybe_wrap(unsafe { sys::SBThreadGetFrameAtIndex(thread.raw, parent_idx) })
        } else {
            None
        }
    }
}

impl fmt::Debug for SBFrame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBFrameGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBFrame {{ {} }}", stream.data())
    }
}

impl Drop for SBFrame {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBFrame(self.raw) };
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBFrame: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    // TODO(bm): This should be u32
    field frame_id() -> i64 {
        self.frame_id() as i64
    }

    // TODO(bm) This should be u64
    field cfa() -> Option<i64> {
        self.cfa().map(|i| i as i64)
    }

    // TODO(bm) This should be u64
    field pc() -> i64 {
        self.pc() as i64
    }

    // TODO(bm) This should be u64
    field sp() -> i64 {
        self.sp() as i64
    }

    // TODO(bm) This should be u64
    field fp() -> i64 {
        self.fp() as i64
    }

    field pc_address() -> SBAddress {
        self.pc_address()
    }

    field module() -> SBModule {
        self.module()
    }

    field compile_unit() -> SBCompileUnit {
        self.compile_unit()
    }

    field function() -> SBFunction {
        self.function()
    }

    field symbol() -> SBSymbol {
        self.symbol()
    }

    field block() -> SBBlock {
        self.block()
    }

    field function_name() -> Option<&str> {
        self.function_name()
    }

    field display_function_name() -> Option<&str> {
        self.display_function_name()
    }

    field is_inlined() -> bool {
        self.is_inlined()
    }

    field frame_block() -> SBBlock {
        self.frame_block()
    }

    field line_entry() -> Option<SBLineEntry> {
        self.line_entry()
    }

    field thread() -> SBThread {
        self.thread()
    }
});
