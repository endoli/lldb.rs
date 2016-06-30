// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::listener::SBListener;
use super::lldb_pid_t;
use sys;

/// Configuration for attaching to a process.
///
/// See `SBTarget::attach`.
#[derive(Debug)]
pub struct SBAttachInfo {
    /// The underlying raw `SBAttachInfoRef`.
    pub raw: sys::SBAttachInfoRef,
}

impl SBAttachInfo {
    /// Construct a new `SBAttachInfo`.
    pub fn new() -> SBAttachInfo {
        SBAttachInfo::wrap(unsafe { sys::CreateSBAttachInfo() })
    }

    /// Construct a new `SBAttachInfo` for a given process ID (pid).
    pub fn new_with_pid(pid: lldb_pid_t) -> SBAttachInfo {
        SBAttachInfo::wrap(unsafe { sys::CreateSBAttachInfo2(pid) })
    }

    /// Construct a new `SBAttachInfo`.
    pub fn wrap(raw: sys::SBAttachInfoRef) -> SBAttachInfo {
        SBAttachInfo { raw: raw }
    }

    #[allow(missing_docs)]
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBAttachInfoGetProcessID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_process_id(&mut self, pid: lldb_pid_t) {
        unsafe { sys::SBAttachInfoSetProcessID(self.raw, pid) };
    }

    #[allow(missing_docs)]
    pub fn listener(&self) -> SBListener {
        SBListener::wrap(unsafe { sys::SBAttachInfoGetListener(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn set_listener(&mut self, listener: SBListener) {
        unsafe { sys::SBAttachInfoSetListener(self.raw, listener.raw) };
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
