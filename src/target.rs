// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    lldb_addr_t, sys, DescriptionLevel, MatchType, SBAddress, SBAttachInfo, SBBreakpoint,
    SBBroadcaster, SBDebugger, SBError, SBEvent, SBExpressionOptions, SBFileSpec, SBLaunchInfo,
    SBModule, SBModuleSpec, SBPlatform, SBProcess, SBStream, SBSymbolContextList, SBValue,
    SBWatchpoint, SymbolType,
};
use std::ffi::{CStr, CString};
use std::fmt;

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
/// out an [`SBLaunchInfo`] and calling [`SBTarget::launch()`].
///
/// ```no_run
/// use lldb::*;
/// fn launch_target(target: &SBTarget) -> Result<SBProcess, SBError> {
///     let launch_info = SBLaunchInfo::new();
///     launch_info.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);
///     // Probably want to set up a listener here.
///     target.launch(launch_info)
/// }
/// ```
///
/// ## Attaching
///
/// Attaching to a process can be done by creating and filling
/// out an [`SBAttachInfo`] and calling [`SBTarget::attach()`].
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
pub struct SBTarget {
    /// The underlying raw `SBTargetRef`.
    pub raw: sys::SBTargetRef,
}

impl SBTarget {
    /// Construct a new `SBTarget`.
    pub(crate) fn wrap(raw: sys::SBTargetRef) -> SBTarget {
        SBTarget { raw }
    }

    /// Construct a new `Some(SBTarget)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBTargetRef) -> Option<SBTarget> {
        if unsafe { sys::SBTargetIsValid(raw) } {
            Some(SBTarget { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBTarget` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBTargetIsValid(self.raw) }
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
    pub fn platform(&self) -> SBPlatform {
        unsafe {
            SBPlatform {
                raw: sys::SBTargetGetPlatform(self.raw),
            }
        }
    }

    /// Get the [`SBProcess`] associated with this target.
    pub fn process(&self) -> SBProcess {
        unsafe {
            SBProcess {
                raw: sys::SBTargetGetProcess(self.raw),
            }
        }
    }

    /// Launch a target for debugging.
    pub fn launch(&self, launch_info: SBLaunchInfo) -> Result<SBProcess, SBError> {
        let error: SBError = SBError::default();
        let process =
            SBProcess::wrap(unsafe { sys::SBTargetLaunch2(self.raw, launch_info.raw, error.raw) });
        if error.is_success() {
            Ok(process)
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn load_core(&self, core_file: &str) -> Result<SBProcess, SBError> {
        let error: SBError = SBError::default();
        let core_file = CString::new(core_file).unwrap();
        let process = SBProcess::wrap(unsafe {
            sys::SBTargetLoadCore(self.raw, core_file.as_ptr(), error.raw)
        });
        if error.is_success() {
            Ok(process)
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn attach(&self, attach_info: SBAttachInfo) -> Result<SBProcess, SBError> {
        let error: SBError = SBError::default();
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
        unsafe { sys::SBTargetAddModule(self.raw, module.raw) }
    }

    /// Add a module to the target using an `SBModuleSpec`.
    pub fn add_module_spec(&self, module_spec: &SBModuleSpec) -> Option<SBModule> {
        SBModule::maybe_wrap(unsafe { sys::SBTargetAddModuleSpec(self.raw, module_spec.raw) })
    }

    /// Remove a module from the target.
    pub fn remove_module(&self, module: &SBModule) -> bool {
        unsafe { sys::SBTargetRemoveModule(self.raw, module.raw) }
    }

    /// Get the debugger controlling this target.
    pub fn debugger(&self) -> SBDebugger {
        SBDebugger {
            raw: unsafe { sys::SBTargetGetDebugger(self.raw) },
        }
    }

    /// Get an iterator over the [modules] known to this target instance.
    ///
    /// [modules]: SBModule
    pub fn modules(&self) -> SBTargetModuleIter {
        SBTargetModuleIter {
            target: self,
            idx: 0,
        }
    }

    /// Find the module for the given `SBFileSpec`.
    pub fn find_module(&self, file_spec: &SBFileSpec) -> Option<SBModule> {
        SBModule::maybe_wrap(unsafe { sys::SBTargetFindModule(self.raw, file_spec.raw) })
    }

    /// Resolve a current file address into a section offset address.
    pub fn resolve_file_address(&self, file_addr: lldb_addr_t) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBTargetResolveFileAddress(self.raw, file_addr) })
    }

    /// Resolve a current load address into a section offset address.
    ///
    /// The return value will be `None` if the `vm_addr` doesn't resolve to
    /// a section within a module.
    pub fn resolve_load_address(&self, vm_addr: lldb_addr_t) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBTargetResolveLoadAddress(self.raw, vm_addr) })
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
    pub fn breakpoint_create_by_location(&self, file: &str, line: u32) -> SBBreakpoint {
        let file = CString::new(file).unwrap();
        SBBreakpoint::wrap(unsafe {
            sys::SBTargetBreakpointCreateByLocation(self.raw, file.as_ptr(), line)
        })
    }

    #[allow(missing_docs)]
    pub fn breakpoint_create_by_address(&self, address: lldb_addr_t) -> SBBreakpoint {
        SBBreakpoint::wrap(unsafe { sys::SBTargetBreakpointCreateByAddress(self.raw, address) })
    }

    #[allow(missing_docs)]
    pub fn breakpoint_create_by_sbaddress(&self, address: SBAddress) -> SBBreakpoint {
        SBBreakpoint::wrap(unsafe {
            sys::SBTargetBreakpointCreateBySBAddress(self.raw, address.raw)
        })
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
    pub fn watch_address(
        &self,
        addr: lldb_addr_t,
        size: usize,
        read: bool,
        write: bool,
    ) -> Result<SBWatchpoint, SBError> {
        let error: SBError = SBError::default();
        let watchpoint =
            unsafe { sys::SBTargetWatchAddress(self.raw, addr, size, read, write, error.raw) };
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

    #[allow(missing_docs)]
    pub fn find_functions(&self, name: &str, name_type_mask: u32) -> SBSymbolContextList {
        let name = CString::new(name).unwrap();
        SBSymbolContextList::wrap(unsafe {
            sys::SBTargetFindFunctions(self.raw, name.as_ptr(), name_type_mask)
        })
    }

    #[allow(missing_docs)]
    pub fn find_global_functions(
        &self,
        name: &str,
        max_matches: u32,
        matchtype: MatchType,
    ) -> SBSymbolContextList {
        let name = CString::new(name).unwrap();
        SBSymbolContextList::wrap(unsafe {
            sys::SBTargetFindGlobalFunctions(self.raw, name.as_ptr(), max_matches, matchtype)
        })
    }

    #[allow(missing_docs)]
    pub fn find_symbols(&self, name: &str, symbol_type: SymbolType) -> SBSymbolContextList {
        let name = CString::new(name).unwrap();
        SBSymbolContextList::wrap(unsafe {
            sys::SBTargetFindSymbols(self.raw, name.as_ptr(), symbol_type)
        })
    }

    /// Evaluate an expression.
    pub fn evaluate_expression(&self, expression: &str, options: &SBExpressionOptions) -> SBValue {
        let expression = CString::new(expression).unwrap();
        SBValue::wrap(unsafe {
            sys::SBTargetEvaluateExpression(self.raw, expression.as_ptr(), options.raw)
        })
    }

    #[allow(missing_docs)]
    pub fn event_as_target_event(event: &SBEvent) -> Option<SBTargetEvent> {
        if unsafe { sys::SBTargetEventIsTargetEvent(event.raw) } {
            Some(SBTargetEvent::new(event))
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn get_stack_red_zone_size(&self) -> lldb_addr_t {
        unsafe { sys::SBTargetGetStackRedZoneSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_loaded(&self, module: &SBModule) -> bool {
        unsafe { sys::SBTargetIsLoaded(self.raw, module.raw) }
    }

    #[allow(missing_docs)]
    pub fn get_launch_info(&self) -> SBLaunchInfo {
        SBLaunchInfo::wrap(unsafe { sys::SBTargetGetLaunchInfo(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn set_launch_info(&self, launch_info: SBLaunchInfo) {
        unsafe { sys::SBTargetSetLaunchInfo(self.raw, launch_info.raw) };
    }
}

impl Clone for SBTarget {
    fn clone(&self) -> SBTarget {
        SBTarget {
            raw: unsafe { sys::CloneSBTarget(self.raw) },
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

unsafe impl Send for SBTarget {}
unsafe impl Sync for SBTarget {}

/// Iterate over the [breakpoints] in a [target].
///
/// [breakpoints]: SBBreakpoint
/// [target]: SBTarget
pub struct SBTargetBreakpointIter<'d> {
    target: &'d SBTarget,
    idx: usize,
}

impl<'d> Iterator for SBTargetBreakpointIter<'d> {
    type Item = SBBreakpoint;

    fn next(&mut self) -> Option<SBBreakpoint> {
        if self.idx < unsafe { sys::SBTargetGetNumBreakpoints(self.target.raw) as usize } {
            let r = Some(SBBreakpoint::wrap(unsafe {
                sys::SBTargetGetBreakpointAtIndex(self.target.raw, self.idx as u32)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBTargetGetNumBreakpoints(self.target.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBTargetBreakpointIter<'d> {}

/// Iterate over the [watchpoints] in a [target].
///
/// [watchpoints]: SBWatchpoint
/// [target]: SBTarget
pub struct SBTargetWatchpointIter<'d> {
    target: &'d SBTarget,
    idx: usize,
}

impl<'d> Iterator for SBTargetWatchpointIter<'d> {
    type Item = SBWatchpoint;

    fn next(&mut self) -> Option<SBWatchpoint> {
        if self.idx < unsafe { sys::SBTargetGetNumWatchpoints(self.target.raw) as usize } {
            let r = Some(SBWatchpoint::wrap(unsafe {
                sys::SBTargetGetWatchpointAtIndex(self.target.raw, self.idx as u32)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBTargetGetNumWatchpoints(self.target.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBTargetWatchpointIter<'d> {}

#[allow(missing_docs)]
pub struct SBTargetEvent<'e> {
    event: &'e SBEvent,
}

#[allow(missing_docs)]
impl<'e> SBTargetEvent<'e> {
    pub fn new(event: &'e SBEvent) -> Self {
        SBTargetEvent { event }
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
/// [modules]: SBModule
/// [target event]: SBTargetEvent
pub struct SBTargetEventModuleIter<'d> {
    event: &'d SBTargetEvent<'d>,
    idx: usize,
}

impl<'d> Iterator for SBTargetEventModuleIter<'d> {
    type Item = SBModule;

    fn next(&mut self) -> Option<SBModule> {
        if self.idx < unsafe { sys::SBTargetGetNumModulesFromEvent(self.event.event.raw) as usize }
        {
            let r = Some(SBModule::wrap(unsafe {
                sys::SBTargetGetModuleAtIndexFromEvent(self.idx as u32, self.event.event.raw)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBTargetGetNumModulesFromEvent(self.event.event.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBTargetEventModuleIter<'d> {}

/// Iterate over the [modules] in a [target].
///
/// [modules]: SBModule
/// [target]: SBTarget
pub struct SBTargetModuleIter<'d> {
    target: &'d SBTarget,
    idx: u32,
}

impl<'d> Iterator for SBTargetModuleIter<'d> {
    type Item = SBModule;

    fn next(&mut self) -> Option<SBModule> {
        if self.idx < unsafe { sys::SBTargetGetNumModules(self.target.raw) } {
            let r = Some(SBModule::wrap(unsafe {
                sys::SBTargetGetModuleAtIndex(self.target.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBTargetGetNumModules(self.target.raw) } as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBTargetModuleIter<'d> {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBTarget {
    fn platform() -> SBPlatform {
        self.platform()
    }

    fn process() -> SBProcess {
        self.process()
    }

    fn executable() -> Option<SBFileSpec> {
        self.executable()
    }

    fn debugger() -> SBDebugger {
        self.debugger()
    }

    fn breakpoints() -> Vec<SBBreakpoint> {
        self.breakpoints().collect()
    }

    fn watchpoints() -> Vec<SBWatchpoint> {
        self.watchpoints().collect()
    }
}
