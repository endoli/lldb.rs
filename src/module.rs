// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::filespec::SBFileSpec;
use sys;

/// An executable image and its associated object and symbol files.
#[derive(Debug)]
pub struct SBModule {
    /// The underlying raw `SBModuleRef`.
    pub raw: sys::SBModuleRef,
}

impl SBModule {
    /// Construct a new `SBModule`.
    pub fn new(raw: sys::SBModuleRef) -> SBModule {
        SBModule { raw: raw }
    }

    /// Construct a new `Some(SBModule)` or `None`.
    pub fn maybe(raw: sys::SBModuleRef) -> Option<SBModule> {
        if unsafe { sys::SBModuleIsValid(raw) != 0 } {
            Some(SBModule { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBModule` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBModuleIsValid(self.raw) != 0 }
    }

    /// The file for the module on the host system that is running LLDB.
    ///
    /// This can differ from the path on the platform since we might
    /// be doing remote debugging.
    pub fn filespec(&self) -> SBFileSpec {
        SBFileSpec { raw: unsafe { sys::SBModuleGetFileSpec(self.raw) } }
    }

    /// The file for the module as it is known on the remote system on
    /// which it is being debugged.
    ///
    /// For local debugging this is always the same as `SBModule::filespec`.
    /// But remote debugging might mention a file `/usr/lib/liba.dylib`
    /// which might be locally downloaded and cached. In this case the
    /// platform file could be something like:
    /// `/tmp/lldb/platform-cache/remote.host.computer/usr/lib/liba.dylib`
    /// The file could also be cached in a local developer kit directory.
    pub fn platform_filespec(&self) -> SBFileSpec {
        SBFileSpec { raw: unsafe { sys::SBModuleGetPlatformFileSpec(self.raw) } }
    }
}
