// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{lldb_pid_t, sys, SBFileSpec};
use std::ffi::CStr;

/// Describes an existing process and any discoverable information that
/// pertains to that process.
#[derive(Debug)]
pub struct SBProcessInfo {
    /// The underlying raw `SBProcessInfoRef`.
    pub raw: sys::SBProcessInfoRef,
}

impl SBProcessInfo {
    /// Construct a new `SBProcessInfo`.
    pub(crate) fn wrap(raw: sys::SBProcessInfoRef) -> SBProcessInfo {
        SBProcessInfo { raw }
    }

    #[allow(missing_docs)]
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBProcessInfoGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn executable_file(&self) -> SBFileSpec {
        SBFileSpec::wrap(unsafe { sys::SBProcessInfoGetExecutableFile(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBProcessInfoGetProcessID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn user_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn group_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn effective_user_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoEffectiveUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetEffectiveUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn effective_group_id(&self) -> Option<u32> {
        if unsafe { sys::SBProcessInfoEffectiveGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBProcessInfoGetEffectiveGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn parent_process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBProcessInfoGetParentProcessID(self.raw) }
    }

    /// Return the target triple (arch-vendor-os) for the described process.
    pub fn triple(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBProcessInfoGetTriple(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }
}

impl Clone for SBProcessInfo {
    fn clone(&self) -> SBProcessInfo {
        SBProcessInfo {
            raw: unsafe { sys::CloneSBProcessInfo(self.raw) },
        }
    }
}

impl Drop for SBProcessInfo {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBProcessInfo(self.raw) };
    }
}

unsafe impl Send for SBProcessInfo {}
unsafe impl Sync for SBProcessInfo {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBProcessInfo {
    fn name() -> &str {
        self.name()
    }

    fn executable_file() -> SBFileSpec {
        self.executable_file()
    }

    // TODO(bm) This should be lldb_pid_t
    fn process_id() -> i32 {
        self.process_id() as i32
    }

    // TODO(bm) This should be u32
    fn user_id() -> Option<i32> {
        self.user_id().map(|i| i as i32)
    }

    // TODO(bm) This should be u32
    fn group_id() -> Option<i32> {
        self.group_id().map(|i| i as i32)
    }

    // TODO(bm) This should be u32
    fn effective_user_id() -> Option<i32> {
        self.effective_user_id().map(|i| i as i32)
    }

    // TODO(bm) This should be u32
    fn effective_group_id() -> Option<i32> {
        self.effective_group_id().map(|i| i as i32)
    }

    // TODO(bm) This should be lldb_pid_t
    fn parent_process_id() -> i32 {
        self.parent_process_id() as i32
    }

    fn triple() -> &str {
        self.triple()
    }
}
