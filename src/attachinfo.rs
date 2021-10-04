// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{lldb_pid_t, sys, SBFileSpec, SBListener};
use std::ffi::{CStr, CString};

/// Configuration for attaching to a process.
///
/// See [`SBTarget::attach()`].
///
/// [`SBTarget::attach()`]: crate::SBTarget::attach()
#[derive(Debug)]
pub struct SBAttachInfo {
    /// The underlying raw `SBAttachInfoRef`.
    pub raw: sys::SBAttachInfoRef,
}

impl SBAttachInfo {
    /// Construct a new `SBAttachInfo`.
    pub fn new() -> SBAttachInfo {
        SBAttachInfo::from(unsafe { sys::CreateSBAttachInfo() })
    }

    /// Construct a new `SBAttachInfo` for a given process ID (pid).
    pub fn new_with_pid(pid: lldb_pid_t) -> SBAttachInfo {
        SBAttachInfo::from(unsafe { sys::CreateSBAttachInfo2(pid) })
    }

    /// Attach to a process by name.
    ///
    /// Future calls to `SBTarget::attach(...)` will be synchronous or
    /// asynchronous depending on the `async` argument.
    ///
    /// * `path`: A full or partial name for the process to attach to.
    /// * `wait_for`: If `false`, attach to an existing process whose name
    ///   matches. If `true`, then wait for the next process whose name
    ///   matches.
    /// * `async`: If `false`, then the `SBTarget::attach` call will be
    ///   synchronous with no way to cancel the attach while it is in
    ///   progress. If `true`, then the `SBTarget::attach` call will return
    ///   immediately and clients are expected to wait for a process
    ///   `eStateStopped` event if a suitable process is eventually found.
    ///   If the client wants to cancel the event, `SBProcess::stop` can be
    ///   called and an `eStateExited` process event will be delivered.
    pub fn new_with_path(path: &str, wait_for: bool, async: bool) -> SBAttachInfo {
        let p = CString::new(path).unwrap();
        SBAttachInfo::from(unsafe { sys::CreateSBAttachInfo4(p.as_ptr(), wait_for, async) })
    }

    #[allow(missing_docs)]
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBAttachInfoGetProcessID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_process_id(&self, pid: lldb_pid_t) {
        unsafe { sys::SBAttachInfoSetProcessID(self.raw, pid) };
    }

    #[allow(missing_docs)]
    pub fn set_executable_path(&self, path: &str) {
        let p = CString::new(path).unwrap();
        unsafe { sys::SBAttachInfoSetExecutable(self.raw, p.as_ptr()) }
    }

    #[allow(missing_docs)]
    pub fn set_executable_filespec(&self, exe_file: SBFileSpec) {
        unsafe { sys::SBAttachInfoSetExecutable2(self.raw, exe_file.raw) }
    }

    #[allow(missing_docs)]
    pub fn wait_for_launch(&self) -> bool {
        unsafe { sys::SBAttachInfoGetWaitForLaunch(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_wait_for_launch(&self, wait: bool, async: bool) {
        unsafe { sys::SBAttachInfoSetWaitForLaunch2(self.raw, wait, async) };
    }

    #[allow(missing_docs)]
    pub fn ignore_existing(&self) -> bool {
        unsafe { sys::SBAttachInfoGetIgnoreExisting(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_ignore_existing(&self, b: bool) {
        unsafe { sys::SBAttachInfoSetIgnoreExisting(self.raw, b) }
    }

    #[allow(missing_docs)]
    pub fn resume_count(&self) -> u32 {
        unsafe { sys::SBAttachInfoGetResumeCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_resume_count(&self, c: u32) {
        unsafe { sys::SBAttachInfoSetResumeCount(self.raw, c) }
    }

    #[allow(missing_docs)]
    pub fn process_plugin_name(&self) -> Option<&str> {
        unsafe {
            match CStr::from_ptr(sys::SBAttachInfoGetProcessPluginName(self.raw)).to_str() {
                Ok(s) => Some(s),
                _ => None,
            }
        }
    }

    #[allow(missing_docs)]
    pub fn set_process_plugin_name(&self, plugin: &str) {
        let plugin = CString::new(plugin).unwrap();
        unsafe { sys::SBAttachInfoSetProcessPluginName(self.raw, plugin.as_ptr()) };
    }

    #[allow(missing_docs)]
    pub fn user_id(&self) -> Option<u32> {
        if unsafe { sys::SBAttachInfoUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBAttachInfoGetUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_user_id(&self, uid: u32) {
        unsafe { sys::SBAttachInfoSetUserID(self.raw, uid) };
    }

    #[allow(missing_docs)]
    pub fn group_id(&self) -> Option<u32> {
        if unsafe { sys::SBAttachInfoGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBAttachInfoGetGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_group_id(&self, gid: u32) {
        unsafe { sys::SBAttachInfoSetGroupID(self.raw, gid) };
    }

    #[allow(missing_docs)]
    pub fn effective_user_id(&self) -> Option<u32> {
        if unsafe { sys::SBAttachInfoEffectiveUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBAttachInfoGetEffectiveUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_effective_user_id(&self, uid: u32) {
        unsafe { sys::SBAttachInfoSetEffectiveUserID(self.raw, uid) };
    }

    #[allow(missing_docs)]
    pub fn effective_group_id(&self) -> Option<u32> {
        if unsafe { sys::SBAttachInfoEffectiveGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBAttachInfoGetEffectiveGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_effective_group_id(&self, gid: u32) {
        unsafe { sys::SBAttachInfoSetEffectiveGroupID(self.raw, gid) };
    }

    #[allow(missing_docs)]
    pub fn parent_process_id(&self) -> Option<lldb_pid_t> {
        if unsafe { sys::SBAttachInfoParentProcessIDIsValid(self.raw) } {
            Some(unsafe { sys::SBAttachInfoGetParentProcessID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_parent_process_id(&self, ppid: lldb_pid_t) {
        unsafe { sys::SBAttachInfoSetParentProcessID(self.raw, ppid) };
    }

    /// Get the listener that will be used to receive process events.
    ///
    /// If no listener has been set via a call to
    /// `SBAttachInfo::set_listener()`, then `None` will be returned.
    /// If a listener has been set, then the listener object will be returned.
    pub fn listener(&self) -> Option<SBListener> {
        SBListener::maybe_wrap(unsafe { sys::SBAttachInfoGetListener(self.raw) })
    }

    /// Set the listener that will be used to receive process events.
    ///
    /// By default the [`SBDebugger`], which has a listener,
    /// that the [`SBTarget`] belongs to will listen for the
    /// process events. Calling this function allows a different
    /// listener to be used to listen for process events.
    ///
    /// [`SBDebugger`]: struct.SBDebugger.html
    /// [`SBTarget`]: struct.SBTarget.html
    pub fn set_listener(&self, listener: SBListener) {
        unsafe { sys::SBAttachInfoSetListener(self.raw, listener.raw) };
    }
}

impl Clone for SBAttachInfo {
    fn clone(&self) -> SBAttachInfo {
        SBAttachInfo {
            raw: unsafe { sys::CloneSBAttachInfo(self.raw) },
        }
    }
}

impl Default for SBAttachInfo {
    fn default() -> SBAttachInfo {
        SBAttachInfo::new()
    }
}

impl Drop for SBAttachInfo {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBAttachInfo(self.raw) };
    }
}

impl From<sys::SBAttachInfoRef> for SBAttachInfo {
    fn from(raw: sys::SBAttachInfoRef) -> SBAttachInfo {
        SBAttachInfo { raw }
    }
}

unsafe impl Send for SBAttachInfo {}
unsafe impl Sync for SBAttachInfo {}
