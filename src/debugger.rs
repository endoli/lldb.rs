// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    sys, SBCommandInterpreter, SBError, SBListener, SBPlatform, SBStream, SBStructuredData,
    SBTarget,
};
use std::ffi::{CStr, CString};
use std::fmt;
use std::iter;
use std::ptr;

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
/// # Platform Management
///
/// LLDB supports multiple platforms when debugging.
///
/// LLDB is aware of both available and active platforms. By default,
/// the `host` platform is active for debugging processes on the local
/// machine.
///
/// A number of additional platforms are
/// [available][SBDebugger::available_platforms()] and can be activated
/// via [`SBDebugger::set_current_platform()`].
///
/// The currently selected platform is controlled by
/// [`SBDebugger::set_selected_platform()`] typically using
/// instances of [`SBPlatform`].
///
/// When doing remote debugging, additional confirmation and work
/// is required. (See `SBPlatform::connect_remote()`. This is
/// not yet wrapped in this library.)
///
/// See also:
///
/// * [`SBDebugger::available_platforms()`]
/// * [`SBDebugger::platforms()`]
/// * [`SBDebugger::selected_platform()`]
/// * [`SBDebugger::set_current_platform()`]
/// * [`SBDebugger::set_selected_platform()`]
/// * [`SBPlatform`]
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
/// [`SBTarget`]: struct.SBTarget.html
/// [`set_async`]: #method.set_async
/// [`async`]: #method.async
/// [`create_target`]: #method.create_target
/// [`create_target_simple`]: #method.create_target_simple
/// [`targets`]: #method.targets
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
        SBDebugger {
            raw: unsafe { sys::SBDebuggerCreate2(source_init_files) },
        }
    }

    /// Get whether or not the debugger is in async mode.
    ///
    /// When in async mode, the debugger returns immediately when
    /// stepping or continuing without waiting for the process
    /// to change state.
    pub fn async(&self) -> bool {
        unsafe { sys::SBDebuggerGetAsync(self.raw) }
    }

    /// Set the debugger to be in async mode or not.
    ///
    /// When in async mode, the debugger returns immediately when
    /// stepping or continuing without waiting for the process
    /// to change state.
    pub fn set_async(&self, async: bool) {
        unsafe { sys::SBDebuggerSetAsync(self.raw, async) }
    }

    #[allow(missing_docs)]
    pub fn command_interpreter(&self) -> SBCommandInterpreter {
        SBCommandInterpreter::from(unsafe { sys::SBDebuggerGetCommandInterpreter(self.raw) })
    }

    /// Enable logging (defaults to `stderr`).
    ///
    /// `enable_log("lldb", &["default"])` is useful for troubleshooting in most
    /// cases. Include `"all"` in `categories` for extra verbosity.
    ///
    /// See invocations to `lldb_private::Log::Register` for more channels and
    /// categories.
    pub fn enable_log(&self, channel: &str, categories: &[&str]) -> bool {
        let channel = CString::new(channel).unwrap();
        let categories: Vec<_> = categories
            .iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let categories_ptr: Vec<_> = categories
            .iter()
            .map(|s| s.as_ptr())
            .chain(iter::once(ptr::null()))
            .collect();
        unsafe { sys::SBDebuggerEnableLog(self.raw, channel.as_ptr(), categories_ptr.as_ptr()) }
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
    pub fn create_target(
        &self,
        executable: &str,
        target_triple: Option<&str>,
        platform_name: Option<&str>,
        add_dependent_modules: bool,
    ) -> Result<SBTarget, SBError> {
        let executable = CString::new(executable).unwrap();
        let target_triple = target_triple.map(|s| CString::new(s).unwrap());
        let platform_name = platform_name.map(|s| CString::new(s).unwrap());
        let error = SBError::default();
        let target = unsafe {
            sys::SBDebuggerCreateTarget(
                self.raw,
                executable.as_ptr(),
                target_triple.map_or(ptr::null(), |s| s.as_ptr()),
                platform_name.map_or(ptr::null(), |s| s.as_ptr()),
                add_dependent_modules,
                error.raw,
            )
        };
        if error.is_success() {
            Ok(SBTarget::from(target))
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
    ///
    /// [`create_target`]: struct.SBDebugger.html#method.create_target
    pub fn create_target_simple(&self, executable: &str) -> Option<SBTarget> {
        let executable = CString::new(executable).unwrap();
        SBTarget::maybe_wrap(unsafe { sys::SBDebuggerCreateTarget2(self.raw, executable.as_ptr()) })
    }

    /// Get an iterator over the [targets] known to this debugger instance.
    ///
    /// [targets]: struct.SBTarget.html
    pub fn targets(&self) -> SBDebuggerTargetIter {
        SBDebuggerTargetIter {
            debugger: self,
            idx: 0,
        }
    }

    /// Get the default [SBListener] associated with the debugger.
    ///
    /// [SBListener]: struct.SBListener.html
    pub fn listener(&self) -> SBListener {
        SBListener::from(unsafe { sys::SBDebuggerGetListener(self.raw) })
    }

    /// Get the currently selected [`SBTarget`].
    ///
    /// [SBTarget]: struct.SBTarget.html
    pub fn selected_target(&self) -> Option<SBTarget> {
        SBTarget::maybe_wrap(unsafe { sys::SBDebuggerGetSelectedTarget(self.raw) })
    }

    /// Set the selected [`SBTarget`].
    ///
    /// [SBTarget]: struct.SBTarget.html
    pub fn set_selected_target(&self, target: &SBTarget) {
        unsafe { sys::SBDebuggerSetSelectedTarget(self.raw, target.raw) };
    }

    /// Get an iterator over the currently active [platforms][SBPlatform].
    ///
    /// By default, the `host` platform will be active. Additional
    /// platforms can be activated via [`SBDebugger::set_current_platform()`].
    ///
    /// See also:
    ///
    /// * [`SBDebugger::available_platforms()`]
    /// * [`SBDebugger::selected_platform()`]
    /// * [`SBDebugger::set_current_platform()`]
    /// * [`SBDebugger::set_selected_platform()`]
    pub fn platforms(&self) -> SBDebuggerPlatformIter {
        SBDebuggerPlatformIter {
            debugger: self,
            idx: 0,
        }
    }

    /// Get the currently selected [`SBPlatform`].
    ///
    /// See also:
    ///
    /// * [`SBDebugger::platforms()`]
    /// * [`SBDebugger::set_current_platform()`]
    /// * [`SBDebugger::set_selected_platform()`]
    pub fn selected_platform(&self) -> SBPlatform {
        unsafe {
            SBPlatform {
                raw: sys::SBDebuggerGetSelectedPlatform(self.raw),
            }
        }
    }

    /// Set the selected [`SBPlatform`].
    ///
    /// Selecting a platform by name rather than an instance of [`SBPlatform`]
    /// can be done via [`SBDebugger::set_current_platform()`].
    ///
    /// See also:
    ///
    /// * [`SBDebugger::platforms()`]
    /// * [`SBDebugger::selected_platform()`]
    /// * [`SBDebugger::set_current_platform()`]
    pub fn set_selected_platform(&self, platform: &SBPlatform) {
        unsafe { sys::SBDebuggerSetSelectedPlatform(self.raw, platform.raw) };
    }

    /// Get an iterator over the available [platforms][SBPlatform] known to
    /// this debugger instance.
    ///
    /// These correspond to the available platform plugins within LLDB. The
    /// platform name can be used with [`SBDebugger::set_current_platform()`]
    /// to activate and select it.
    ///
    /// The structured data will have 2 string keys:
    /// * `"name"` - Name of the platform plugin.
    /// * `"description"` - The description of the platform plugin.
    ///
    /// See also:
    ///
    /// * [`SBDebugger::platforms()`]
    /// * [`SBDebugger::selected_platform()`]
    /// * [`SBDebugger::set_current_platform()`]
    /// * [`SBDebugger::set_selected_platform()`]
    pub fn available_platforms(&self) -> SBDebuggerAvailablePlatformIter {
        SBDebuggerAvailablePlatformIter {
            debugger: self,
            idx: 0,
        }
    }

    /// Activate and select an available [platform][SBPlatform] by name.
    ///
    /// The list of available platforms can be found via
    /// [`SBDebugger::available_platforms()`].
    ///
    /// See also:
    ///
    /// * [`SBDebugger::available_platforms()`]
    /// * [`SBDebugger::platforms()`]
    /// * [`SBDebugger::selected_platform()`]
    /// * [`SBDebugger::set_selected_platform()`]
    pub fn set_current_platform(&self, platform_name: &str) {
        let platform_name = CString::new(platform_name).unwrap();
        unsafe { sys::SBDebuggerSetCurrentPlatform(self.raw, platform_name.as_ptr()) };
    }
}

/// Iterate over the [targets] known to a [debugger].
///
/// [targets]: struct.SBTarget.html
/// [debugger]: struct.SBDebugger.html
pub struct SBDebuggerTargetIter<'d> {
    debugger: &'d SBDebugger,
    idx: usize,
}

impl<'d> Iterator for SBDebuggerTargetIter<'d> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBDebuggerGetNumTargets(self.debugger.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBDebuggerTargetIter<'d> {}

impl Clone for SBDebugger {
    fn clone(&self) -> SBDebugger {
        SBDebugger {
            raw: unsafe { sys::CloneSBDebugger(self.raw) },
        }
    }
}

impl fmt::Debug for SBDebugger {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBDebuggerGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBDebugger {{ {} }}", stream.data())
    }
}

impl Drop for SBDebugger {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBDebugger(self.raw) };
    }
}

impl From<sys::SBDebuggerRef> for SBDebugger {
    fn from(raw: sys::SBDebuggerRef) -> SBDebugger {
        SBDebugger { raw }
    }
}

unsafe impl Send for SBDebugger {}
unsafe impl Sync for SBDebugger {}

/// Iterate over the [platforms].
///
/// [platforms]: struct.SBPlatform.html
pub struct SBDebuggerPlatformIter<'d> {
    debugger: &'d SBDebugger,
    idx: u32,
}

impl<'d> Iterator for SBDebuggerPlatformIter<'d> {
    type Item = SBPlatform;

    fn next(&mut self) -> Option<SBPlatform> {
        if self.idx < unsafe { sys::SBDebuggerGetNumPlatforms(self.debugger.raw) } {
            let r = Some(SBPlatform::from(unsafe {
                sys::SBDebuggerGetPlatformAtIndex(self.debugger.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBDebuggerGetNumPlatforms(self.debugger.raw) } as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBDebuggerPlatformIter<'d> {}

/// Iterate over the available platforms.
pub struct SBDebuggerAvailablePlatformIter<'d> {
    debugger: &'d SBDebugger,
    idx: u32,
}

impl<'d> Iterator for SBDebuggerAvailablePlatformIter<'d> {
    type Item = SBStructuredData;

    fn next(&mut self) -> Option<SBStructuredData> {
        if self.idx < unsafe { sys::SBDebuggerGetNumAvailablePlatforms(self.debugger.raw) } {
            let r = Some(SBStructuredData::from(unsafe {
                sys::SBDebuggerGetAvailablePlatformInfoAtIndex(self.debugger.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBDebuggerGetNumAvailablePlatforms(self.debugger.raw) } as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBDebuggerAvailablePlatformIter<'d> {}

#[cfg(feature = "graphql")]
impl ::juniper::Context for SBDebugger {}

#[cfg(feature = "graphql")]
graphql_object!(SBDebugger: SBDebugger | &self | {
    field targets() -> Vec<SBTarget> {
        self.targets().collect()
    }

    field selected_target() -> Option<SBTarget> {
        self.selected_target()
    }

    field selected_platform() -> SBPlatform {
        self.selected_platform()
    }

    field platforms() -> Vec<SBPlatform> {
        self.platforms().collect()
    }

    field available_platforms() -> Vec<SBStructuredData> {
        self.available_platforms().collect()
    }
});

#[cfg(test)]
mod tests {
    use super::SBDebugger;

    #[test]
    fn it_works() {
        assert!(!SBDebugger::version().is_empty());
    }
}
