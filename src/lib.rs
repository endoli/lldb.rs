// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # LLDB
//!
//! This crate provides a safe binding to the public API for [LLDB], the
//! debugger provided by the [LLVM project]. LLDB provides a modern, high
//! performance debugger framework and is the default debugger for Mac OS X
//! and iOS.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on [crates.io].
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! lldb = "0.0.5"
//! ```
//!
//! ### Mac OS X Installation Notes
//!
//! On Mac OS X, this library relies upon being able to find
//! the `LLDB.framework` that is provided by Xcode.app.
//!
//! In your own application, you will need to configure the
//! `@rpath` for your execuable to point to the location
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
//! Support for building this has not yet been provided for Linux.
//! Contributions are welcome!
//!
//! ### Windows Installation Notes
//!
//! Support for building this has not yet been provided for Linux.
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
//! [LLDB]: http://lldb.llvm.org/
//! [LLVM project]: http://llvm.org/
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
#![deny(trivial_numeric_casts, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate lldb_sys as sys;

#[cfg(feature = "graphql")]
#[macro_use]
extern crate juniper;

pub use sys::{lldb_addr_t, lldb_offset_t, lldb_pid_t, lldb_tid_t, lldb_user_id_t};

pub use sys::{AccessType, AddressClass, BasicType, BreakpointEventType, ByteOrder,
              CommandArgumentType, CommandFlags, ConnectionStatus, DescriptionLevel,
              DynamicValueType, EmulateInstructionOptions, Encoding, ErrorType,
              ExpressionEvaluationPhase, ExpressionResults, FilePermissions, Format,
              FrameComparison, FunctionNameType, GdbSignal, InputReaderAction,
              InputReaderGranularity, InstrumentationRuntimeType, LanguageType, LaunchFlags,
              MatchType, MemberFunctionKind, PathType, Permissions, QueueItemKind, QueueKind,
              RegisterKind, ReturnStatus, RunMode, ScriptLanguage, SectionType, StateType,
              StopReason, SymbolContextItem, SymbolType, TemplateArgumentKind, TypeClass, TypeFlags,
              TypeOptions, TypeSummaryCapping, ValueType, WatchpointEventType, WatchpointKind};

pub use sys::{LAUNCH_FLAG_EXEC, LAUNCH_FLAG_DEBUG, LAUNCH_FLAG_STOP_AT_ENTRY,
              LAUNCH_FLAG_DISABLE_ASLR, LAUNCH_FLAG_DISABLE_STDIO, LAUNCH_FLAG_LAUNCH_IN_TTY,
              LAUNCH_FLAG_LAUNCH_IN_SHELL, LAUNCH_FLAG_LAUNCH_IN_SEPARATE_PROCESS_GROUP,
              LAUNCH_FLAG_DONT_SET_EXIT_STATUS, LAUNCH_FLAG_DETACH_ON_ERRROR,
              LAUNCH_FLAG_SHELL_EXPAND_ARGUMENTS, LAUNCH_FLAG_CLOSE_TTY_ON_EXIT};

pub use sys::{SYMBOL_CONTEXT_ITEM_TARGET, SYMBOL_CONTEXT_ITEM_MODULE, SYMBOL_CONTEXT_ITEM_COMPUNIT,
              SYMBOL_CONTEXT_ITEM_FUNCTION, SYMBOL_CONTEXT_ITEM_BLOCK,
              SYMBOL_CONTEXT_ITEM_LINE_ENTRY, SYMBOL_CONTEXT_ITEM_SYMBOL,
              SYMBOL_CONTEXT_ITEM_EVERYTHING, SYMBOL_CONTEXT_ITEM_VARIABLE};

pub use sys::{PERMISSIONS_WRITABLE, PERMISSIONS_READABLE, PERMISSIONS_EXECUTABLE};

pub use sys::{TYPE_CLASS_INVALID, TYPE_CLASS_ARRAY, TYPE_CLASS_BLOCKPOINTER, TYPE_CLASS_BUILTIN,
              TYPE_CLASS_CLASS, TYPE_CLASS_COMPLEX_FLOAT, TYPE_CLASS_COMPLEX_INTEGER,
              TYPE_CLASS_ENUMERATION, TYPE_CLASS_FUNCTION, TYPE_CLASS_MEMBER_POINTER,
              TYPE_CLASS_OBJC_OBJECT, TYPE_CLASS_OBJC_INTERFACE, TYPE_CLASS_OBJC_OBJECT_POINTER,
              TYPE_CLASS_POINTER, TYPE_CLASS_REFERENCE, TYPE_CLASS_STRUCT, TYPE_CLASS_TYPEDEF,
              TYPE_CLASS_UNION, TYPE_CLASS_VECTOR, TYPE_CLASS_OTHER, TYPE_CLASS_ANY};

pub use sys::{FILE_PERMISSIONS_WORLD_EXECUTE, FILE_PERMISSIONS_WORLD_WRITE,
              FILE_PERMISSIONS_WORLD_READ, FILE_PERMISSIONS_GROUP_EXECUTE,
              FILE_PERMISSIONS_GROUP_WRITE, FILE_PERMISSIONS_GROUP_READ,
              FILE_PERMISSIONS_USER_EXECUTE, FILE_PERMISSIONS_USER_WRITE,
              FILE_PERMISSIONS_USER_READ, FILE_PERMISSIONS_WORLD_RX, FILE_PERMISSIONS_WORLD_RW,
              FILE_PERMISSIONS_WORLD_RWX, FILE_PERMISSIONS_GROUP_RX, FILE_PERMISSIONS_GROUP_RW,
              FILE_PERMISSIONS_GROUP_RWX, FILE_PERMISSIONS_USER_RX, FILE_PERMISSIONS_USER_RW,
              FILE_PERMISSIONS_USER_RWX, FILE_PERMISSIONS_EVERYONE_R, FILE_PERMISSIONS_EVERYONE_W,
              FILE_PERMISSIONS_EVERYONE_X, FILE_PERMISSIONS_EVERYONE_RW,
              FILE_PERMISSIONS_EVERYONE_RX, FILE_PERMISSIONS_EVERYONE_RWX,
              FILE_PERMISSIONS_FILE_DEFAULT, FILE_PERMISSIONS_DIRECTORY_DEFAULT};

mod address;
mod attachinfo;
mod block;
mod breakpoint;
mod breakpointlocation;
mod broadcaster;
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
mod stream;
mod stringlist;
mod symbol;
mod symbolcontext;
mod target;
mod thread;
mod value;
mod valuelist;
mod variablesoptions;
mod watchpoint;

pub use self::address::SBAddress;
pub use self::attachinfo::SBAttachInfo;
pub use self::block::SBBlock;
pub use self::breakpoint::{SBBreakpoint, SBBreakpointLocationIter};
pub use self::breakpointlocation::SBBreakpointLocation;
pub use self::broadcaster::SBBroadcaster;
pub use self::compileunit::SBCompileUnit;
pub use self::data::SBData;
pub use self::debugger::{SBDebugger, SBDebuggerTargetIter};
pub use self::error::SBError;
pub use self::event::SBEvent;
pub use self::expressionoptions::SBExpressionOptions;
pub use self::filespec::SBFileSpec;
pub use self::filespeclist::SBFileSpecList;
pub use self::frame::SBFrame;
pub use self::function::SBFunction;
pub use self::instruction::SBInstruction;
pub use self::instructionlist::{SBInstructionList, SBInstructionListIter};
pub use self::launchinfo::SBLaunchInfo;
pub use self::lineentry::SBLineEntry;
pub use self::listener::SBListener;
pub use self::module::SBModule;
pub use self::modulespec::SBModuleSpec;
pub use self::platform::SBPlatform;
pub use self::process::{SBProcess, SBProcessEvent, SBProcessEventRestartedReasonIter,
                        SBProcessThreadIter};
pub use self::stream::SBStream;
pub use self::stringlist::{SBStringList, SBStringListIter};
pub use self::symbol::SBSymbol;
pub use self::symbolcontext::SBSymbolContext;
pub use self::target::{SBTarget, SBTargetBreakpointIter, SBTargetEvent, SBTargetEventModuleIter,
                       SBTargetWatchpointIter};
pub use self::thread::{SBThread, SBThreadEvent, SBThreadFrameIter};
pub use self::value::SBValue;
pub use self::valuelist::SBValueList;
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
