// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::error::SBError;
use super::launchinfo::SBLaunchInfo;
use super::lldb_pid_t;
use std::ffi::CStr;
use sys;

/// A platform that can represent the current host or a
/// remote host debug platform.
///
/// The `SBPlatform` class represents the current host, or a remote host.
/// It can be connected to a remote platform in order to provide ways
/// to remotely launch and attach to processes, upload/download files,
/// create directories, run remote shell commands, find locally cached
/// versions of files from the remote system, and much more.
///
/// `SBPlatform` objects can be created and then used to connect to a remote
/// platform which allows the `SBPlatform` to be used to get a list of the
/// current processes on the remote host, attach to one of those processes,
/// install programs on the remote system, attach and launch processes,
/// and much more.
///
/// Every [`SBTarget`] has a corresponding `SBPlatform`. The platform can be
/// specified upon target creation, or the currently selected platform
/// will attempt to be used when creating the target automatically as long
/// as the currently selected platform matches the target architecture
/// and executable type. If the architecture or executable type do not match,
/// a suitable platform will be found automatically.
///
/// [`SBTarget`]: struct.SBTarget.html
#[derive(Debug)]
pub struct SBPlatform {
    /// The underlying raw `SBPlatformRef`.
    pub raw: sys::SBPlatformRef,
}

impl SBPlatform {
    /// Construct a new `Some(SBPlatform)` or `None`.
    pub fn maybe_wrap(raw: sys::SBPlatformRef) -> Option<SBPlatform> {
        if unsafe { sys::SBPlatformIsValid(raw) } {
            Some(SBPlatform { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBPlatform` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBPlatformIsValid(self.raw) }
    }

    /// The working directory for this platform.
    pub fn working_directory(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetWorkingDirectory(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The name of the platform.
    ///
    /// When debugging on the host platform, this would be `"host"`.
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The triple used to describe this platform.
    ///
    /// An example value might be `"x86_64-apple-macosx"`.
    pub fn triple(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetTriple(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The hostname for this platform.
    pub fn hostname(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetHostname(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The build ID for the platforms' OS version.
    pub fn os_build(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetOSBuild(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The long form description of the platform's OS version.
    ///
    /// On Mac OS X, this might look like `"Darwin Kernel Version 15.5.0:
    /// Tue Apr 19 18:36:36 PDT 2016; root:xnu-3248.50.21~8/RELEASE_X86_64"`.
    pub fn os_description(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBPlatformGetOSDescription(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The major component of the platform's OS version.
    ///
    /// On Mac OS X 10.11.4, this would have the value `10`.
    pub fn os_major_version(&self) -> u32 {
        unsafe { sys::SBPlatformGetOSMajorVersion(self.raw) }
    }

    /// The minor component of the platform's OS version.
    ///
    /// On Mac OS X 10.11.4, this would have the value `11`.
    pub fn os_minor_version(&self) -> u32 {
        unsafe { sys::SBPlatformGetOSMinorVersion(self.raw) }
    }

    /// The patch or update component of the platform's OS version.
    ///
    /// On Mac OS X 10.11.4, this would have the value `4`.
    pub fn os_update_version(&self) -> u32 {
        unsafe { sys::SBPlatformGetOSUpdateVersion(self.raw) }
    }

    /// Launch a process. This is not for debugging that process.
    pub fn launch(&self, launch_info: &SBLaunchInfo) -> Result<(), SBError> {
        let error = SBError::from(unsafe { sys::SBPlatformLaunch(self.raw, launch_info.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Kill a process.
    pub fn kill(&self, pid: lldb_pid_t) -> Result<(), SBError> {
        let error = SBError::from(unsafe { sys::SBPlatformKill(self.raw, pid) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

impl Clone for SBPlatform {
    fn clone(&self) -> SBPlatform {
        SBPlatform {
            raw: unsafe { sys::CloneSBPlatform(self.raw) },
        }
    }
}

impl Drop for SBPlatform {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBPlatform(self.raw) };
    }
}

impl From<sys::SBPlatformRef> for SBPlatform {
    fn from(raw: sys::SBPlatformRef) -> SBPlatform {
        SBPlatform { raw }
    }
}

unsafe impl Send for SBPlatform {}
unsafe impl Sync for SBPlatform {}

#[cfg(feature = "graphql")]
graphql_object!(SBPlatform: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field working_directory() -> &str {
        self.working_directory()
    }

    field name() -> &str {
        self.name()
    }

    field triple() -> &str {
        self.triple()
    }

    field hostname() -> &str {
        self.hostname()
    }

    field os_build() -> &str {
        self.os_build()
    }

    field os_description() -> &str {
        self.os_description()
    }

    // TODO(bm) This should be u32
    field os_major_version() -> i32 {
        self.os_major_version() as i32
    }

    // TODO(bm) This should be u32
    field os_minor_version() -> i32 {
        self.os_minor_version() as i32
    }

    // TODO(bm) This should be u32
    field os_update_version() -> i32 {
        self.os_update_version() as i32
    }
});
