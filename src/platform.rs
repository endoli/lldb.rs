// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    /// Check whether or not this is a valid `SBPlatform` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBPlatformIsValid(self.raw) != 0 }
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
}
