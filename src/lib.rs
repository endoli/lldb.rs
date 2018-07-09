// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # LLDB
//!
//! This crate provides a safe binding to the public API for [LLDB], the
//! debugger provided by the [LLVM project]. LLDB provides a modern, high
//! performance debugger framework and is the default debugger for macOS
//! and iOS.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on [crates.io].
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! lldb = "0.0.7"
//! ```
//!
//! ### macOS Installation Notes
//!
//! On macOS, this library relies upon being able to find
//! the `LLDB.framework` that is provided by Xcode.app.
//!
//! In your own application, you will need to configure the
//! `@rpath` for your executable to point to the location
//! of the `LLDB.framework` that you are using. Doing this
//! automatically is not currently supported by Cargo.
//!
//! For testing, you can use the version provided by
//! Xcode.app by setting an environment variable:
//!
//! ```shell
//! export DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/SharedFrameworks
//! ```
//!
//! ### Linux Installation Notes
//!
//! Install the `lldb` and `lldb-dev` packages for your platform so that
//! you have both LLDB itself installed as well as the headers and
//! other support files required.
//!
//! ### Windows Installation Notes
//!
//! Support for building this has not yet been provided for Windows.
//! Contributions are welcome!
//!
//! ## Usage
//!
//! The primary entry point is [`SBDebugger`]. This will be how you
//! create a debug target and begin the actually interesting stuff.
//!
//! ## Important Classes
//!
//! The LLDB API provides many structs and a wide range of functionality. Some of the
//! most common usages will involve these structs and their corresponding methods:
//!
//! * [`SBDebugger`]: Manages the entire debug experience and creates [`SBTarget`]s.
//! * [`SBTarget`]: The target program running under the debugger.
//! * [`SBProcess`]: The process associated with the target program.
//! * [`SBThread`]: A thread of execution. [`SBProcess`] contains
//!   [`SBThread`]s.
//! * [`SBFrame`]: One of the stack frames associated with
//!   a thread. [`SBThread`] contains [`SBFrame`]s.
//! * [`SBSymbolContext`]: A container that stores various debugger
//!   related info.
//! * [`SBValue`]: The value of a variable, a register, or an
//!   expression.
//! * [`SBModule`]: An executable image and its associated object
//!   and symbol files.  [`SBTarget`] contains [`SBModule`]s.
//! * [`SBBreakpoint`]: A logical breakpoint and its associated
//!   settings. [`SBTarget`] contains [`SBBreakpoint`]s.
//! * [`SBSymbol`]: The symbol possibly associated with a stack frame.
//! * [`SBCompileUnit`]: A compilation unit, or compiled source file.
//! * [`SBFunction`]: A generic function, which can be inlined or not.
//! * [`SBBlock`]: A lexical block. [`SBFunction`] contains [`SBBlock`]s.
//! * [`SBLineEntry`]: Specifies an association with a contiguous range of
//!   instructions and a source file location. [`SBCompileUnit`] contains
//!   [`SBLineEntry`]s.
//!
//! ## Support and Maintenance
//!
//! I am developing this library largely on my own so far. I am able
//! to offer support and maintenance, but would very much appreciate
//! donations via [Patreon](https://patreon.com/endoli). I can also
//! provide commercial support, so feel free to
//! [contact me](mailto:bruce.mitchener@gmail.com).
//!
//! [LLDB]: https://lldb.llvm.org/
//! [LLVM project]: https://llvm.org/
//! [crates.io]: https://crates.io/crates/lldb
//! [`SBDebugger`]: struct.SBDebugger.html
//! [`SBTarget`]: struct.SBTarget.html
//! [`SBProcess`]: struct.SBProcess.html
//! [`SBThread`]: struct.SBThread.html
//! [`SBFrame`]: struct.SBFrame.html
//! [`SBSymbolContext`]: struct.SBSymbolContext.html
//! [`SBValue`]: struct.SBValue.html
//! [`SBModule`]: struct.SBModule.html
//! [`SBBreakpoint`]: struct.SBBreakpoint.html
//! [`SBSymbol`]: struct.SBSymbol.html
//! [`SBCompileUnit`]: struct.SBCompileUnit.html
//! [`SBFunction`]: struct.SBFunction.html
//! [`SBBlock`]: struct.SBBlock.html
//! [`SBLineEntry`]: struct.SBLineEntry.html

#![warn(missing_docs)]
#![deny(
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

extern crate lldb_sys as sys;

#[cfg(feature = "graphql")]
#[macro_use]
extern crate juniper;

pub use sys::{lldb_addr_t, lldb_offset_t, lldb_pid_t, lldb_tid_t, lldb_user_id_t};

pub use sys::{
    AccessType, AddressClass, BasicType, BreakpointEventType, ByteOrder, CommandArgumentType,
    CommandFlags, ConnectionStatus, DescriptionLevel, DynamicValueType, EmulateInstructionOptions,
    Encoding, ErrorType, ExpressionEvaluationPhase, ExpressionResults, FilePermissions, Format,
    FrameComparison, FunctionNameType, GdbSignal, InputReaderAction, InputReaderGranularity,
    InstrumentationRuntimeType, LanguageType, LaunchFlags, MatchType, MemberFunctionKind, PathType,
    Permissions, QueueItemKind, QueueKind, RegisterKind, ReturnStatus, RunMode, ScriptLanguage,
    SectionType, StateType, StopReason, SymbolContextItem, SymbolType, TemplateArgumentKind,
    TypeClass, TypeFlags, TypeOptions, TypeSummaryCapping, ValueType, WatchpointEventType,
    WatchpointKind,
};

mod address;
mod attachinfo;
mod block;
mod breakpoint;
mod breakpointlist;
mod breakpointlocation;
mod broadcaster;
mod commandinterpreter;
mod compileunit;
mod data;
mod debugger;
mod error;
mod event;
mod expressionoptions;
mod filespec;
mod filespeclist;
mod frame;
mod function;
mod instruction;
mod instructionlist;
mod launchinfo;
mod lineentry;
mod listener;
mod module;
mod modulespec;
mod platform;
mod process;
mod processinfo;
mod queue;
mod queueitem;
mod section;
mod stream;
mod stringlist;
mod structureddata;
mod symbol;
mod symbolcontext;
mod symbolcontextlist;
mod target;
mod thread;
mod typelist;
mod types;
mod value;
mod valuelist;
mod variablesoptions;
mod watchpoint;

pub use self::address::SBAddress;
pub use self::attachinfo::SBAttachInfo;
pub use self::block::SBBlock;
pub use self::breakpoint::{SBBreakpoint, SBBreakpointLocationIter};
pub use self::breakpointlist::{SBBreakpointList, SBBreakpointListIter};
pub use self::breakpointlocation::SBBreakpointLocation;
pub use self::broadcaster::SBBroadcaster;
pub use self::commandinterpreter::SBCommandInterpreter;
pub use self::compileunit::SBCompileUnit;
pub use self::data::SBData;
pub use self::debugger::{SBDebugger, SBDebuggerTargetIter};
pub use self::error::SBError;
pub use self::event::SBEvent;
pub use self::expressionoptions::SBExpressionOptions;
pub use self::filespec::SBFileSpec;
pub use self::filespeclist::{SBFileSpecList, SBFileSpecListIter};
pub use self::frame::SBFrame;
pub use self::function::SBFunction;
pub use self::instruction::SBInstruction;
pub use self::instructionlist::{SBInstructionList, SBInstructionListIter};
pub use self::launchinfo::SBLaunchInfo;
pub use self::lineentry::SBLineEntry;
pub use self::listener::SBListener;
pub use self::module::{SBModule, SBModuleSectionIter};
pub use self::modulespec::SBModuleSpec;
pub use self::platform::SBPlatform;
pub use self::process::{
    SBProcess, SBProcessEvent, SBProcessEventRestartedReasonIter, SBProcessQueueIter,
    SBProcessThreadIter,
};
pub use self::processinfo::SBProcessInfo;
pub use self::queue::{SBQueue, SBQueueQueueItemIter, SBQueueThreadIter};
pub use self::queueitem::SBQueueItem;
pub use self::section::{SBSection, SBSectionSubSectionIter};
pub use self::stream::SBStream;
pub use self::stringlist::{SBStringList, SBStringListIter};
pub use self::structureddata::SBStructuredData;
pub use self::symbol::SBSymbol;
pub use self::symbolcontext::SBSymbolContext;
pub use self::symbolcontextlist::SBSymbolContextList;
pub use self::target::{
    SBTarget, SBTargetBreakpointIter, SBTargetEvent, SBTargetEventModuleIter, SBTargetModuleIter,
    SBTargetWatchpointIter,
};
pub use self::thread::{SBThread, SBThreadEvent, SBThreadFrameIter};
pub use self::typelist::{SBTypeList, SBTypeListIter};
pub use self::types::SBType;
pub use self::value::SBValue;
pub use self::valuelist::{SBValueList, SBValueListIter};
pub use self::variablesoptions::SBVariablesOptions;
pub use self::watchpoint::SBWatchpoint;

/// Which syntax should be used in disassembly?
///
/// On x86, there are 2 syntaxes used for disassembly. Other
/// architectures need not be concerned by this and can just
/// use `DisassemblyFlavor::Default` all the time.
#[derive(Clone, Copy, Debug)]
pub enum DisassemblyFlavor {
    /// The primary syntax used by the GNU Assembler and in the
    /// Linux world.
    ///
    /// AT&T syntax:
    ///
    /// * Operations has a suffix, indicating the operand size.
    /// * Prefixes registers with `%` and immediate values with `$`.
    /// * Orders operands with source first, then destination.
    /// * Memory operands are somewhat complicated.
    ///
    /// For example:
    ///
    /// ```asm
    /// movb $0x05, %al
    /// ```
    ///
    /// This syntax is described in detail in [GAS Syntax].
    ///
    /// [GAS Syntax]: https://en.wikibooks.org/wiki/X86_Assembly/GAS_Syntax
    ATT,
    /// The default syntax.
    Default,
    /// The primary syntax used on Windows.
    ///
    /// This differs from AT&T syntax in that:
    ///
    /// * Operations are not suffixed.
    /// * Registers are not prefixed with `%`, immediate values have a suffix.
    /// * Operands are ordered so that the destination is first, then the source.
    ///
    /// For example:
    ///
    /// ```asm
    /// mov al, 05h
    /// ```
    ///
    Intel,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
