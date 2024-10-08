// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{lldb_pid_t, sys, LaunchFlags, SBFileSpec, SBListener};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Configuration for launching a process.
///
/// See [`SBTarget::launch()`].
///
/// [`SBTarget::launch()`]: crate::SBTarget::launch()
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
    pub(crate) fn wrap(raw: sys::SBLaunchInfoRef) -> SBLaunchInfo {
        SBLaunchInfo { raw }
    }

    #[allow(missing_docs)]
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBLaunchInfoGetProcessID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn user_id(&self) -> Option<u32> {
        if unsafe { sys::SBLaunchInfoUserIDIsValid(self.raw) } {
            Some(unsafe { sys::SBLaunchInfoGetUserID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_user_id(&self, user_id: u32) {
        unsafe { sys::SBLaunchInfoSetUserID(self.raw, user_id) };
    }

    #[allow(missing_docs)]
    pub fn group_id(&self) -> Option<u32> {
        if unsafe { sys::SBLaunchInfoGroupIDIsValid(self.raw) } {
            Some(unsafe { sys::SBLaunchInfoGetGroupID(self.raw) })
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn set_group_id(&self, group_id: u32) {
        unsafe { sys::SBLaunchInfoSetGroupID(self.raw, group_id) };
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
    /// [`SBTarget::launch(...)`]: crate::SBTarget::launch()
    pub fn set_executable_file(&self, filespec: &SBFileSpec, add_as_first_arg: bool) {
        unsafe { sys::SBLaunchInfoSetExecutableFile(self.raw, filespec.raw, add_as_first_arg) };
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
    /// [`SBDebugger`]: crate::SBDebugger
    /// [`SBTarget`]: crate::SBTarget
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

    /// Specify the command line arguments.
    pub fn set_arguments<'a>(&self, args: impl IntoIterator<Item = &'a str>, append: bool) {
        let cstrs: Vec<CString> = args.into_iter().map(|a| CString::new(a).unwrap()).collect();
        let mut ptrs: Vec<*const c_char> = cstrs.iter().map(|cs| cs.as_ptr()).collect();
        ptrs.push(ptr::null());
        let argv = ptrs.as_ptr();
        unsafe { sys::SBLaunchInfoSetArguments(self.raw, argv, append) };
    }

    /// Returns an iterator over the command line arguments.
    pub fn arguments(&self) -> impl Iterator<Item = &str> {
        SBLaunchInfoArgumentsIter {
            launch_info: self,
            index: 0,
        }
    }

    #[allow(missing_docs)]
    fn num_arguments(&self) -> u32 {
        unsafe { sys::SBLaunchInfoGetNumArguments(self.raw) }
    }

    #[allow(missing_docs)]
    fn argument_at_index(&self, index: u32) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBLaunchInfoGetArgumentAtIndex(self.raw, index)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
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
        unsafe { sys::SBLaunchInfoGetShellExpandArguments(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_shell_expand_arguments(&self, expand: bool) {
        unsafe { sys::SBLaunchInfoSetShellExpandArguments(self.raw, expand) };
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
        unsafe { sys::SBLaunchInfoAddCloseFileAction(self.raw, fd) }
    }

    #[allow(missing_docs)]
    pub fn add_duplicate_file_action(&self, fd: i32, dup_fd: i32) -> bool {
        unsafe { sys::SBLaunchInfoAddDuplicateFileAction(self.raw, fd, dup_fd) }
    }

    #[allow(missing_docs)]
    pub fn add_open_file_action(&self, fd: i32, path: &str, read: bool, write: bool) -> bool {
        let path = CString::new(path).unwrap();
        unsafe { sys::SBLaunchInfoAddOpenFileAction(self.raw, fd, path.as_ptr(), read, write) }
    }

    #[allow(missing_docs)]
    pub fn add_suppress_file_action(&self, fd: i32, read: bool, write: bool) -> bool {
        unsafe { sys::SBLaunchInfoAddSuppressFileAction(self.raw, fd, read, write) }
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
        unsafe { sys::SBLaunchInfoGetDetachOnError(self.raw) }
    }
    #[allow(missing_docs)]
    pub fn set_detach_on_error(&self, detach: bool) {
        unsafe { sys::SBLaunchInfoSetDetachOnError(self.raw, detach) };
    }
}

impl Clone for SBLaunchInfo {
    fn clone(&self) -> SBLaunchInfo {
        SBLaunchInfo {
            raw: unsafe { sys::CloneSBLaunchInfo(self.raw) },
        }
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

unsafe impl Send for SBLaunchInfo {}
unsafe impl Sync for SBLaunchInfo {}

pub struct SBLaunchInfoArgumentsIter<'d> {
    launch_info: &'d SBLaunchInfo,
    index: u32,
}

impl<'d> Iterator for SBLaunchInfoArgumentsIter<'d> {
    type Item = &'d str;

    fn next(&mut self) -> Option<&'d str> {
        if self.index < self.launch_info.num_arguments() {
            self.index += 1;
            Some(self.launch_info.argument_at_index(self.index - 1))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.launch_info.num_arguments();
        (sz as usize - self.index as usize, Some(sz as usize))
    }
}

impl<'d> ExactSizeIterator for SBLaunchInfoArgumentsIter<'d> {}
