// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{CStr, CString};
use std::ptr;
use super::error::SBError;
use super::platform::SBPlatform;
use super::target::SBTarget;
use sys;

/// Creates [`SBTarget`]s, provides access to them and manages
/// the overall debugging experience.
///
/// # Initialization and Teardown
///
/// LLDB must be initialized before the functionality is used. This
/// is done with `SBDebugger::initialize()`:
///
/// ```
/// use lldb::SBDebugger;
///
/// SBDebugger::initialize();
/// ```
///
/// Similarly, it must be terminated after you are done using it:
///
/// ```
/// use lldb::SBDebugger;
///
/// SBDebugger::initialize();
/// // Use LLDB functionality ...
/// SBDebugger::terminate();
/// ```
///
/// Once you've initialized LLDB, you're ready to create an instance
/// of `SBDebugger`:
///
/// ```
/// use lldb::SBDebugger;
///
/// SBDebugger::initialize();
///
/// let debugger = SBDebugger::create(false);
/// // ... configure the debugger if needed ...
/// // ... create a target and do stuff ...
///
/// SBDebugger::terminate();
/// ```
///
/// # Configuration
///
/// ## Async Mode
///
/// While it is best to use LLDB in asynchronous mode, it does offer a
/// synchronous mode, which can be easier to use for quick experiments
/// or scripts.
///
/// In synchronous mode, calls to the LLDB API do not return until the
/// underlying action has been completed. This means that the thread
/// from which you call LLDB will be blocked during that time, so this
/// is not an ideal way to use LLDB for building interactive tools
/// or a new user interface.
///
/// In asynchronous mode, calls to the LLDB API will return immediately
/// without waiting for the action to complete. This means that actions
/// like launching a target, continuing the execution of a process and
/// so on won't be completed immediately and you must process events
/// to see what the results of an action are.
///
/// Synchronous mode can be enabled by using [`set_async`] and passing it
/// a `false` value. You can see if you're in asynchronous mode or not
/// by calling [`async`].
///
/// # Target Management
///
/// The `SBDebugger` instance tracks the various targets that are
/// currently known to the debugger.
///
/// Typically, you create a target with [`create_target`],
/// [`create_target_simple`] or one of the related methods.
///
/// Sometimes, you'll want to create a target without an associated
/// executable. A common use case for this is to attach to a process
/// by name or process ID where you don't know the executable in advance.
/// The most convenient way to do this is:
///
/// ```
/// # use lldb::SBDebugger;
/// # SBDebugger::initialize();
/// let debugger = SBDebugger::create(false);
/// if let Some(target) = debugger.create_target_simple("") {
///     println!("Got a target: {:?}", target);
///     // Now, maybe we'd like to attach to something.
/// }
/// # SBDebugger::terminate();
/// ```
///
/// You can iterate over these targets which have been created by
/// using [`targets`]:
///
/// ```no_run
/// # use lldb::{SBDebugger, SBTarget};
/// # fn look_at_targets(debugger: SBDebugger) {
/// // Iterate over the targets...
/// for target in debugger.targets() {
///     println!("Hello {:?}!", target);
/// }
/// // Or collect them into a vector!
/// let targets = debugger.targets().collect::<Vec<SBTarget>>();
/// # }
/// ```
///
/// # Platform Management
///
/// ...
///
/// [`SBTarget`]: struct.SBTarget.html
/// [`set_async`]: #method.set_async
/// [`async`]: #method.async
/// [`create_target`]: #method.create_target
/// [`create_target_simple`]: #method.create_target_simple
/// [`targets`]: #method.targets
#[derive(Debug)]
pub struct SBDebugger {
    /// The underlying raw `SBDebuggerRef`.
    pub raw: sys::SBDebuggerRef,
}

impl SBDebugger {
    /// Initialize LLDB.
    ///
    /// This should be called before LLDB functionality is used.
    pub fn initialize() {
        unsafe { sys::SBDebuggerInitialize() };
    }

    /// Tear down LLDB.
    ///
    /// This should be called once the application no longer needs
    /// to use LLDB functionality. Typically, this is called as the
    /// application exits.
    pub fn terminate() {
        unsafe { sys::SBDebuggerTerminate() };
    }

    /// Create a new instance of `SBDebugger`.
    ///
    /// If `source_init_files` is `true`, then `~/.lldbinit` will
    /// be processed.
    pub fn create(source_init_files: bool) -> SBDebugger {
        SBDebugger { raw: unsafe { sys::SBDebuggerCreate2(source_init_files as u8) } }
    }

    /// Get whether or not the debugger is in async mode.
    ///
    /// When in async mode, the debugger returns immediately when
    /// stepping or continuing without waiting for the process
    /// to change state.
    pub fn async(&self) -> bool {
        unsafe { sys::SBDebuggerGetAsync(self.raw) != 0 }
    }

    /// Set the debugger to be in async mode or not.
    ///
    /// When in async mode, the debugger returns immediately when
    /// stepping or continuing without waiting for the process
    /// to change state.
    pub fn set_async(&mut self, async: bool) {
        unsafe { sys::SBDebuggerSetAsync(self.raw, async as u8) }
    }

    /// Get the LLDB version string.
    pub fn version() -> String {
        unsafe {
            match CStr::from_ptr(sys::SBDebuggerGetVersionString()).to_str() {
                Ok(s) => s.to_owned(),
                _ => panic!("No version string?"),
            }
        }
    }

    /// Create a target.
    ///
    /// The executable name may be an empty string to create
    /// an empty target.
    pub fn create_target(&self,
                         executable: &str,
                         target_triple: Option<&str>,
                         platform_name: Option<&str>,
                         add_dependent_modules: bool)
                         -> Result<SBTarget, SBError> {
        let executable = CString::new(executable).unwrap().as_ptr();
        let target_triple =
            target_triple.map(CString::new).map(|s| s.unwrap().as_ptr()).unwrap_or(ptr::null());
        let platform_name =
            platform_name.map(CString::new).map(|s| s.unwrap().as_ptr()).unwrap_or(ptr::null());
        let error = SBError::new();
        let target = unsafe {
            sys::SBDebuggerCreateTarget(self.raw,
                                        executable,
                                        target_triple,
                                        platform_name,
                                        add_dependent_modules as u8,
                                        error.raw)
        };
        if error.is_success() {
            Ok(SBTarget::wrap(target))
        } else {
            Err(error)
        }
    }

    /// Create a target from just an executable name.
    ///
    /// The executable name may be an empty string to create
    /// an empty target.
    ///
    /// Using [`create_target`] is preferred in most cases as
    /// that provides access to an `SBError` to inform the caller
    /// about what might have gone wrong.
    pub fn create_target_simple(&self, executable: &str) -> Option<SBTarget> {
        let executable = CString::new(executable).unwrap();
        SBTarget::maybe_wrap(unsafe { sys::SBDebuggerCreateTarget2(self.raw, executable.as_ptr()) })
    }

    /// Get an iterator over the [targets] known to this debugger instance.
    ///
    /// [targets]: struct.SBTarget.html
    pub fn targets(&self) -> DebuggerTargetIter {
        DebuggerTargetIter {
            debugger: self,
            idx: 0,
        }
    }

    /// Get the currently selected [`SBPlatform`].
    ///
    /// [`SBPlatform`]: struct.SBPlatform.html
    pub fn selected_platform(&self) -> SBPlatform {
        unsafe { SBPlatform { raw: sys::SBDebuggerGetSelectedPlatform(self.raw) } }
    }

    /// Set the selected [`SBPlatform`].
    ///
    /// [`SBPlatform`]: struct.SBPlatform.html
    pub fn set_selected_platform(&mut self, platform: &SBPlatform) {
        unsafe { sys::SBDebuggerSetSelectedPlatform(self.raw, platform.raw) };
    }
}

#[doc(hidden)]
pub struct DebuggerTargetIter<'d> {
    debugger: &'d SBDebugger,
    idx: usize,
}

impl<'d> Iterator for DebuggerTargetIter<'d> {
    type Item = SBTarget;

    fn next(&mut self) -> Option<SBTarget> {
        if self.idx < unsafe { sys::SBDebuggerGetNumTargets(self.debugger.raw) as usize } {
            let r = Some(SBTarget {
                raw: unsafe { sys::SBDebuggerGetTargetAtIndex(self.debugger.raw, self.idx as u32) },
            });
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

impl Drop for SBDebugger {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBDebugger(self.raw) };
    }
}

#[cfg(test)]
mod tests {
    use super::SBDebugger;

    #[test]
    fn it_works() {
        assert!(!SBDebugger::version().is_empty());
    }
}
