// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::filespec::SBFileSpec;
use super::listener::SBListener;
use super::lldb_pid_t;
use std::ffi::CString;
use sys;

/// Configuration for attaching to a process.
///
/// See [`SBTarget::attach`].
///
/// [`SBTarget::attach`]: struct.SBTarget.html#method.attach
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

    /// Get the listener that will be used to receive process events.
    ///
    /// If no listener has been set via a call to
    /// `SBAttachInfo::set_listener()`, then an invalid `SBListener` will be
    /// returned (`SBListener::is_valid()` will return `false`). If a listener
    /// has been set, then the valid listener object will be returned.
    pub fn listener(&self) -> SBListener {
        SBListener::from(unsafe { sys::SBAttachInfoGetListener(self.raw) })
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
