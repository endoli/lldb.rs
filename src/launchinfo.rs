// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{CStr, CString};
use std::ptr;
use super::filespec::SBFileSpec;
use super::listener::SBListener;
use super::{LaunchFlags, lldb_pid_t};
use sys;

/// Configuration for launching a process.
///
/// See [`SBTarget::launch`].
///
/// [`SBTarget::launch`]: struct.SBTarget.html#method.launch
#[derive(Debug)]
pub struct SBLaunchInfo {
    /// The underlying raw `SBLaunchInfoRef`.
    pub raw: sys::SBLaunchInfoRef,
}

impl SBLaunchInfo {
    /// Construct a new `SBLaunchInfo`.
    pub fn new() -> SBLaunchInfo {
        SBLaunchInfo::wrap(unsafe { sys::CreateSBLaunchInfo(ptr::null_mut()) })
    }

    /// Construct a new `SBLaunchInfo`.
    pub fn wrap(raw: sys::SBLaunchInfoRef) -> SBLaunchInfo {
        SBLaunchInfo { raw: raw }
    }

    #[allow(missing_docs)]
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBLaunchInfoGetProcessID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn user_id(&self) -> u32 {
        unsafe { sys::SBLaunchInfoGetUserID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_user_id(&self, user_id: u32) {
        unsafe { sys::SBLaunchInfoSetUserID(self.raw, user_id) };
    }

    #[allow(missing_docs)]
    pub fn user_id_is_valid(&self) -> bool {
        unsafe { sys::SBLaunchInfoUserIDIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn group_id(&self) -> u32 {
        unsafe { sys::SBLaunchInfoGetGroupID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_group_id(&self, group_id: u32) {
        unsafe { sys::SBLaunchInfoSetGroupID(self.raw, group_id) };
    }

    #[allow(missing_docs)]
    pub fn group_id_is_valid(&self) -> bool {
        unsafe { sys::SBLaunchInfoGroupIDIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn executable_file(&self) -> Option<SBFileSpec> {
        SBFileSpec::maybe_wrap(unsafe { sys::SBLaunchInfoGetExecutableFile(self.raw) })
    }

    /// Set the executable file that will be used to launch the process and
    /// optionally set it as the first argument in the argument vector.
    ///
    /// This only needs to be specified if clients wish to carefully control
    /// the exact path will be used to launch a binary. If you create a
    /// target with a symlink, that symlink will get resolved in the target
    /// and the resolved path will get used to launch the process. Calling
    /// this function can help you still launch your process using the
    /// path of your choice.
    ///
    /// If this function is not called prior to launching with
    /// [`SBTarget::launch(...)`], the target will use the resolved executable
    /// path that was used to create the target.
    ///
    /// `exe_file` is the override path to use when launching the executable.
    ///
    /// If `add_as_first_arg` is true, then the path will be inserted into
    /// the argument vector prior to launching. Otherwise the argument
    /// vector will be left alone.
    ///
    /// [`SBTarget::launch(...)`]: struct.SBTarget.html#method.launch
    pub fn set_executable_file(&self, filespec: &SBFileSpec, add_as_first_arg: bool) {
        unsafe {
            sys::SBLaunchInfoSetExecutableFile(self.raw, filespec.raw, add_as_first_arg as u8)
        };
    }

    /// Get the listener that will be used to receive process events.
    ///
    /// If no listener has been set via a call to
    /// `SBLaunchInfo::set_listener()`, then `None` will be returned.
    /// If a listener has been set, then the listener object will be returned.
    pub fn listener(&self) -> Option<SBListener> {
        SBListener::maybe_wrap(unsafe { sys::SBLaunchInfoGetListener(self.raw) })
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
    pub fn set_listener(&self, listener: &SBListener) {
        unsafe { sys::SBLaunchInfoSetListener(self.raw, listener.raw) };
    }

    #[allow(missing_docs)]
    pub fn launch_flags(&self) -> LaunchFlags {
        LaunchFlags::from_bits_truncate(unsafe { sys::SBLaunchInfoGetLaunchFlags(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn set_launch_flags(&self, launch_flags: LaunchFlags) {
        unsafe { sys::SBLaunchInfoSetLaunchFlags(self.raw, launch_flags.bits()) }
    }

    #[allow(missing_docs)]
    pub fn process_plugin_name(&self) -> Option<&str> {
        unsafe {
            match CStr::from_ptr(sys::SBLaunchInfoGetProcessPluginName(self.raw)).to_str() {
                Ok(s) => Some(s),
                _ => None,
            }
        }
    }

    #[allow(missing_docs)]
    pub fn set_process_plugin_name(&self, plugin: &str) {
        let plugin = CString::new(plugin).unwrap();
        unsafe { sys::SBLaunchInfoSetProcessPluginName(self.raw, plugin.as_ptr()) };
    }

    #[allow(missing_docs)]
    pub fn shell(&self) -> Option<&str> {
        unsafe {
            match CStr::from_ptr(sys::SBLaunchInfoGetShell(self.raw)).to_str() {
                Ok(s) => Some(s),
                _ => None,
            }
        }
    }

    #[allow(missing_docs)]
    pub fn set_shell(&self, shell: &str) {
        let shell = CString::new(shell).unwrap();
        unsafe { sys::SBLaunchInfoSetShell(self.raw, shell.as_ptr()) };
    }

    #[allow(missing_docs)]
    pub fn shell_expand_arguments(&self) -> bool {
        unsafe { sys::SBLaunchInfoGetShellExpandArguments(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_shell_expand_arguments(&self, expand: bool) {
        unsafe { sys::SBLaunchInfoSetShellExpandArguments(self.raw, expand as u8) };
    }

    #[allow(missing_docs)]
    pub fn resume_count(&self) -> u32 {
        unsafe { sys::SBLaunchInfoGetResumeCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_resume_count(&self, resume_count: u32) {
        unsafe { sys::SBLaunchInfoSetResumeCount(self.raw, resume_count) };
    }

    #[allow(missing_docs)]
    pub fn add_close_file_action(&self, fd: i32) -> bool {
        unsafe { sys::SBLaunchInfoAddCloseFileAction(self.raw, fd) != 0 }
    }

    #[allow(missing_docs)]
    pub fn add_duplicate_file_action(&self, fd: i32, dup_fd: i32) -> bool {
        unsafe { sys::SBLaunchInfoAddDuplicateFileAction(self.raw, fd, dup_fd) != 0 }
    }

    #[allow(missing_docs)]
    pub fn add_open_file_action(&self, fd: i32, path: &str, read: bool, write: bool) -> bool {
        let path = CString::new(path).unwrap();
        unsafe {
            sys::SBLaunchInfoAddOpenFileAction(
                self.raw,
                fd,
                path.as_ptr(),
                read as u8,
                write as u8,
            ) != 0
        }
    }

    #[allow(missing_docs)]
    pub fn add_suppress_file_action(&self, fd: i32, read: bool, write: bool) -> bool {
        unsafe {
            sys::SBLaunchInfoAddSuppressFileAction(self.raw, fd, read as u8, write as u8) != 0
        }
    }

    #[allow(missing_docs)]
    pub fn launch_event_data(&self) -> Option<&str> {
        unsafe {
            match CStr::from_ptr(sys::SBLaunchInfoGetLaunchEventData(self.raw)).to_str() {
                Ok(s) => Some(s),
                _ => None,
            }
        }
    }

    #[allow(missing_docs)]
    pub fn set_launch_event_data(&self, data: &str) {
        let data = CString::new(data).unwrap();
        unsafe { sys::SBLaunchInfoSetLaunchEventData(self.raw, data.as_ptr()) };
    }

    #[allow(missing_docs)]
    pub fn detach_on_error(&self) -> bool {
        unsafe { sys::SBLaunchInfoGetDetachOnError(self.raw) != 0 }
    }
    #[allow(missing_docs)]
    pub fn set_detach_on_error(&self, detach: bool) {
        unsafe { sys::SBLaunchInfoSetDetachOnError(self.raw, detach as u8) };
    }
}

impl Default for SBLaunchInfo {
    fn default() -> SBLaunchInfo {
        SBLaunchInfo::new()
    }
}

impl Drop for SBLaunchInfo {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBLaunchInfo(self.raw) };
    }
}
