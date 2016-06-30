// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::attachinfo::SBAttachInfo;
use super::broadcaster::SBBroadcaster;
use super::debugger::SBDebugger;
use super::error::SBError;
use super::filespec::SBFileSpec;
use super::launchinfo::SBLaunchInfo;
use super::module::SBModule;
use super::modulespec::SBModuleSpec;
use super::platform::SBPlatform;
use super::process::SBProcess;
use sys;

/// The target program running under the debugger.
#[derive(Debug)]
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
        SBModule::maybe_wrap(unsafe { sys::SBTargetAddModule4(self.raw, module_spec.raw) })
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
    pub fn broadcaster(&self) -> SBBroadcaster {
        SBBroadcaster::wrap(unsafe { sys::SBTargetGetBroadcaster(self.raw) })
    }
}

impl Drop for SBTarget {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBTarget(self.raw) };
    }
}
