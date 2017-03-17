// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{CStr, CString};
use std::fmt;
use super::attachinfo::SBAttachInfo;
use super::breakpoint::SBBreakpoint;
use super::broadcaster::SBBroadcaster;
use super::debugger::SBDebugger;
use super::error::SBError;
use super::event::SBEvent;
use super::expressionoptions::SBExpressionOptions;
use super::filespec::SBFileSpec;
use super::launchinfo::SBLaunchInfo;
use super::module::SBModule;
use super::modulespec::SBModuleSpec;
use super::platform::SBPlatform;
use super::process::SBProcess;
use super::stream::SBStream;
use super::value::SBValue;
use super::watchpoint::SBWatchpoint;
use super::{DescriptionLevel, lldb_addr_t};
use sys;

/// The target program running under the debugger.
///
/// # Process Management
///
/// Starting a debug session is done by launching the target,
/// attaching to a running process, or loading a core file.
///
/// ## Launching
///
/// Launching a process can be done by creating and filling
/// out an [`SBLaunchInfo`] and calling [`launch`].
///
/// ```no_run
/// use lldb::*;
/// fn launch_target(target: &SBTarget) -> Result<SBProcess, SBError> {
///     let launch_info = SBLaunchInfo::new();
///     launch_info.set_launch_flags(LAUNCH_FLAG_STOP_AT_ENTRY);
///     // Probably want to set up a listener here.
///     target.launch(launch_info)
/// }
/// ```
///
/// ## Attaching
///
/// Attaching to a process can be done by creating and filling
/// out an [`SBAttachInfo`] and calling [`attach`].
///
/// ```no_run
/// use lldb::{lldb_pid_t, SBAttachInfo, SBError, SBProcess, SBTarget};
/// fn attach_to_pid(target: &SBTarget, pid: lldb_pid_t) -> Result<SBProcess, SBError> {
///     let attach_info = SBAttachInfo::new_with_pid(pid);
///     // Probably want to set up a listener here.
///     target.attach(attach_info)
/// }
/// ```
///
/// ## Core Files
///
/// ...
///
/// # Breakpoints and Watchpoints
///
/// ...
///
/// # Modules
///
/// ...
///
/// # Events
///
/// ...
///
/// [`SBLaunchInfo`]: struct.SBLaunchInfo.html
/// [`launch`]: #method.launch
/// [`SBAttachInfo`]: struct.SBAttachInfo.html
/// [`attach`]: #method.attach
pub struct SBTarget {
    /// The underlying raw `SBTargetRef`.
    pub raw: sys::SBTargetRef,
}

impl SBTarget {
    /// Construct a new `SBTarget`.
    pub fn wrap(raw: sys::SBTargetRef) -> SBTarget {
        SBTarget { raw: raw }
    }

    /// Construct a new `Some(SBTarget)` or `None`.
    pub fn maybe_wrap(raw: sys::SBTargetRef) -> Option<SBTarget> {
        if unsafe { sys::SBTargetIsValid(raw) != 0 } {
            Some(SBTarget { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBTarget` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBTargetIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn broadcaster_class_name() -> &'static str {
        unsafe {
            match CStr::from_ptr(sys::SBTargetGetBroadcasterClassName()).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Get the [`SBPlatform`] associated with this target.
    ///
    /// After return, the platform object should be checked for validity.
    ///
    /// [`SBPlatform`]: strut.SBPlatform.html
    pub fn platform(&self) -> SBPlatform {
        unsafe { SBPlatform { raw: sys::SBTargetGetPlatform(self.raw) } }
    }

    /// Get the [`SBProcess`] associated with this target.
    ///
    /// [`SBProcess`]: strut.SBProcess.html
    pub fn process(&self) -> SBProcess {
        unsafe { SBProcess { raw: sys::SBTargetGetProcess(self.raw) } }
    }

    /// Launch a target for debugging.
    pub fn launch(&self, launch_info: SBLaunchInfo) -> Result<SBProcess, SBError> {
        let error: SBError = SBError::new();
        let process =
            SBProcess::wrap(unsafe { sys::SBTargetLaunch2(self.raw, launch_info.raw, error.raw) });
        if error.is_success() {
            Ok(process)
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn attach(&self, attach_info: SBAttachInfo) -> Result<SBProcess, SBError> {
        let error: SBError = SBError::new();
        let process =
            SBProcess::wrap(unsafe { sys::SBTargetAttach(self.raw, attach_info.raw, error.raw) });
        if error.is_success() {
            Ok(process)
        } else {
            Err(error)
        }
    }

    /// Get a filespec for the executable.
    pub fn executable(&self) -> Option<SBFileSpec> {
        SBFileSpec::maybe_wrap(unsafe { sys::SBTargetGetExecutable(self.raw) })
    }

    /// Add a module to the target.
    pub fn add_module(&self, module: &SBModule) -> bool {
        unsafe { sys::SBTargetAddModule(self.raw, module.raw) != 0 }
    }

    /// Add a module to the target using an `SBModuleSpec`.
    pub fn add_module_spec(&self, module_spec: &SBModuleSpec) -> Option<SBModule> {
        SBModule::maybe_wrap(unsafe { sys::SBTargetAddModuleSpec(self.raw, module_spec.raw) })
    }

    /// Remove a module from the target.
    pub fn remove_module(&self, module: &SBModule) -> bool {
        unsafe { sys::SBTargetRemoveModule(self.raw, module.raw) != 0 }
    }

    /// Get the debugger controlling this target.
    pub fn debugger(&self) -> SBDebugger {
        SBDebugger { raw: unsafe { sys::SBTargetGetDebugger(self.raw) } }
    }

    /// Find the module for the given `SBFileSpec`.
    pub fn find_module(&self, file_spec: &SBFileSpec) -> Option<SBModule> {
        SBModule::maybe_wrap(unsafe { sys::SBTargetFindModule(self.raw, file_spec.raw) })
    }

    #[allow(missing_docs)]
    pub fn delete_breakpoint(&self, break_id: i32) {
        unsafe { sys::SBTargetBreakpointDelete(self.raw, break_id) };
    }

    #[allow(missing_docs)]
    pub fn find_breakpoint_by_id(&self, break_id: i32) -> Option<SBBreakpoint> {
        SBBreakpoint::maybe_wrap(unsafe { sys::SBTargetFindBreakpointByID(self.raw, break_id) })
    }

    #[allow(missing_docs)]
    pub fn enable_all_breakpoints(&self) {
        unsafe { sys::SBTargetEnableAllBreakpoints(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn disable_all_breakpoints(&self) {
        unsafe { sys::SBTargetDisableAllBreakpoints(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn delete_all_breakpoints(&self) {
        unsafe { sys::SBTargetDeleteAllBreakpoints(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn breakpoints(&self) -> SBTargetBreakpointIter {
        SBTargetBreakpointIter {
            target: self,
            idx: 0,
        }
    }

    #[allow(missing_docs)]
    pub fn delete_watchpoint(&self, watch_id: i32) {
        unsafe { sys::SBTargetDeleteWatchpoint(self.raw, watch_id) };
    }

    #[allow(missing_docs)]
    pub fn find_watchpoint_by_id(&self, watch_id: i32) -> Option<SBWatchpoint> {
        SBWatchpoint::maybe_wrap(unsafe { sys::SBTargetFindWatchpointByID(self.raw, watch_id) })
    }

    #[allow(missing_docs)]
    pub fn enable_all_watchpoints(&self) {
        unsafe { sys::SBTargetEnableAllWatchpoints(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn disable_all_watchpoints(&self) {
        unsafe { sys::SBTargetDisableAllWatchpoints(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn delete_all_watchpoints(&self) {
        unsafe { sys::SBTargetDeleteAllWatchpoints(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn watch_address(&self,
                         addr: lldb_addr_t,
                         size: usize,
                         read: bool,
                         write: bool)
                         -> Result<SBWatchpoint, SBError> {
        let error: SBError = SBError::new();
        let watchpoint = unsafe {
            sys::SBTargetWatchAddress(self.raw, addr, size, read as u8, write as u8, error.raw)
        };
        if error.is_success() {
            Ok(SBWatchpoint::wrap(watchpoint))
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn watchpoints(&self) -> SBTargetWatchpointIter {
        SBTargetWatchpointIter {
            target: self,
            idx: 0,
        }
    }

    #[allow(missing_docs)]
    pub fn broadcaster(&self) -> SBBroadcaster {
        SBBroadcaster::wrap(unsafe { sys::SBTargetGetBroadcaster(self.raw) })
    }

    /// Evaluate an expression.
    pub fn evaluate_expression(&self, expression: &str, options: &SBExpressionOptions) -> SBValue {
        let expression = CString::new(expression).unwrap();
        SBValue::wrap(unsafe {
                          sys::SBTargetEvaluateExpression(self.raw,
                                                          expression.as_ptr(),
                                                          options.raw)
                      })
    }

    #[allow(missing_docs)]
    pub fn event_as_target_event(event: &SBEvent) -> Option<SBTargetEvent> {
        if unsafe { sys::SBTargetEventIsTargetEvent(event.raw) != 0 } {
            Some(SBTargetEvent::new(event))
        } else {
            None
        }
    }
}

impl fmt::Debug for SBTarget {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBTargetGetDescription(self.raw, stream.raw, DescriptionLevel::Brief) };
        write!(fmt, "SBTarget {{ {} }}", stream.data())
    }
}

impl Drop for SBTarget {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBTarget(self.raw) };
    }
}

/// Iterate over the [breakpoints] in a [target].
///
/// [breakpoints]: struct.SBBreakpoint.html
/// [target]: struct.SBTarget.html
pub struct SBTargetBreakpointIter<'d> {
    target: &'d SBTarget,
    idx: usize,
}

impl<'d> Iterator for SBTargetBreakpointIter<'d> {
    type Item = SBBreakpoint;

    fn next(&mut self) -> Option<SBBreakpoint> {
        if self.idx < unsafe { sys::SBTargetGetNumBreakpoints(self.target.raw) as usize } {
            let r =
                Some(SBBreakpoint::wrap(unsafe {
                                            sys::SBTargetGetBreakpointAtIndex(self.target.raw,
                                                                              self.idx as u32)
                                        }));
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

/// Iterate over the [watchpoints] in a [target].
///
/// [watchpoints]: struct.SBWatchpoint.html
/// [target]: struct.SBTarget.html
pub struct SBTargetWatchpointIter<'d> {
    target: &'d SBTarget,
    idx: usize,
}

impl<'d> Iterator for SBTargetWatchpointIter<'d> {
    type Item = SBWatchpoint;

    fn next(&mut self) -> Option<SBWatchpoint> {
        if self.idx < unsafe { sys::SBTargetGetNumWatchpoints(self.target.raw) as usize } {
            let r =
                Some(SBWatchpoint::wrap(unsafe {
                                            sys::SBTargetGetWatchpointAtIndex(self.target.raw,
                                                                              self.idx as u32)
                                        }));
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

#[allow(missing_docs)]
pub struct SBTargetEvent<'e> {
    event: &'e SBEvent,
}

#[allow(missing_docs)]
impl<'e> SBTargetEvent<'e> {
    pub fn new(event: &'e SBEvent) -> Self {
        SBTargetEvent { event: event }
    }

    pub fn target(&self) -> SBTarget {
        SBTarget::wrap(unsafe { sys::SBTargetGetTargetFromEvent(self.event.raw) })
    }

    pub fn modules(&self) -> SBTargetEventModuleIter {
        SBTargetEventModuleIter {
            event: self,
            idx: 0,
        }
    }
}

/// Iterate over the [modules] referenced from a [target event].
///
/// [modules]: struct.SBModule.html
/// [target event]: struct.SBTargetEvent.html
pub struct SBTargetEventModuleIter<'d> {
    event: &'d SBTargetEvent<'d>,
    idx: usize,
}

impl<'d> Iterator for SBTargetEventModuleIter<'d> {
    type Item = SBModule;

    fn next(&mut self) -> Option<SBModule> {
        if self.idx <
           unsafe { sys::SBTargetGetNumModulesFromEvent(self.event.event.raw) as usize } {
            let r =
                Some(SBModule::wrap(unsafe {
                                        sys::SBTargetGetModuleAtIndexFromEvent(self.idx as u32,
                                                                               self.event.event.raw)
                                    }));
            self.idx += 1;
            r
        } else {
            None
        }
    }
}
