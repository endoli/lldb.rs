// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # LLDB
//!
//! ## Installation
//!
//! This crate works with Cargo and is on [crates.io].
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! lldb = "0.0.1"
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
//! ## Usage
//!
//! LLDB must be initialized before the functionality is used. This
//! is done with `SBDebugger::initialize()`:
//!
//! ```
//! use lldb;
//!
//! lldb::SBDebugger::initialize();
//! ```
//!
//! Similarly, it must be terminated after you are done using it:
//!
//! ```
//! use lldb;
//!
//! lldb::SBDebugger::initialize();
//! // Use LLDB functionality ...
//! lldb::SBDebugger::terminate();
//! ```
//!
//! The primary entry point from here will be [`SBDebugger`]. This will
//! be how you create a debug target and begin the actually interesting
//! stuff.
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
//! ## Development Guidelines
//!
//! The official LLDB bindings for C++ and Python maintain very
//! strict backwards compatibility. This has resulted in them
//! having methods that are considered to be deprecated in favor
//! of newer methods with more comprehensive arguments. The Rust
//! bindings don't have that problem (yet), so we should try to
//! only write bindings for the recommended usages and not the
//! deprecated methods.
//!
//! The documentation for the official bindings is also somewhat
//! spotty. While it would be great to contribute upstream to
//! improve that, we should seek to have these bindings be well
//! documented and with more examples.
//!
//! People should be jealous of the quality of our bindings.
//! Sic itur ad astra.
//!
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

mod address;
mod block;
mod breakpoint;
mod compileunit;
mod data;
mod debugger;
mod filespec;
mod frame;
mod function;
mod instruction;
mod instructionlist;
mod lineentry;
mod module;
mod modulespec;
mod platform;
mod process;
mod symbol;
mod symbolcontext;
mod target;
mod thread;
mod value;

pub use self::address::SBAddress;
pub use self::block::SBBlock;
pub use self::breakpoint::SBBreakpoint;
pub use self::compileunit::SBCompileUnit;
pub use self::data::SBData;
pub use self::debugger::SBDebugger;
pub use self::filespec::SBFileSpec;
pub use self::frame::SBFrame;
pub use self::function::SBFunction;
pub use self::instruction::SBInstruction;
pub use self::instructionlist::SBInstructionList;
pub use self::lineentry::SBLineEntry;
pub use self::module::SBModule;
pub use self::modulespec::SBModuleSpec;
pub use self::platform::SBPlatform;
pub use self::process::SBProcess;
pub use self::symbol::SBSymbol;
pub use self::symbolcontext::SBSymbolContext;
pub use self::target::SBTarget;
pub use self::thread::SBThread;
pub use self::value::SBValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
