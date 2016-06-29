// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::filespec::SBFileSpec;
use sys;

/// A description of an `SBModule`.
#[derive(Debug)]
pub struct SBModuleSpec {
    /// The underlying raw `SBModuleSpecRef`.
    pub raw: sys::SBModuleSpecRef,
}

impl SBModuleSpec {
    /// Construct a new `SBModuleSpec`.
    pub fn new(raw: sys::SBModuleSpecRef) -> SBModuleSpec {
        SBModuleSpec { raw: raw }
    }

    /// Construct a new `Some(SBModuleSpec)` or `None`.
    pub fn maybe(raw: sys::SBModuleSpecRef) -> Option<SBModuleSpec> {
        if unsafe { sys::SBModuleSpecIsValid(raw) != 0 } {
            Some(SBModuleSpec { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBModuleSpec` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBModuleSpecIsValid(self.raw) != 0 }
    }

    /// The file for the module on the host system that is running LLDB.
    ///
    /// This can differ from the path on the platform since we might
    /// be doing remote debugging.
    pub fn filespec(&self) -> SBFileSpec {
        SBFileSpec { raw: unsafe { sys::SBModuleSpecGetFileSpec(self.raw) } }
    }

    /// Set the file for the module on the host system that is running LLDB.
    pub fn set_filespec(&self, filespec: &SBFileSpec) {
        unsafe { sys::SBModuleSpecSetFileSpec(self.raw, filespec.raw) }
    }

    /// The file for the module as it is known on the remote system which
    /// is being debugged.
    ///
    /// For local debugging, this is always the same as `SBModuleSpec::filespec`.
    /// But remote debugging might mention a file `/usr/lib/liba.dylib`
    /// which might be locally downloaded and cached. In this case, the
    /// platform file could be something like:
    /// `/tmp/lldb/platform-cache/remote.host.computer/usr/lib/liba.dylib`
    /// The file could also be cached in a local developer kit directory.
    pub fn platform_filespec(&self) -> SBFileSpec {
        SBFileSpec { raw: unsafe { sys::SBModuleSpecGetPlatformFileSpec(self.raw) } }
    }

    /// Set the file for the module as it is known on the remote system which
    /// is being debugged.
    pub fn set_platform_filespec(&self, filespec: &SBFileSpec) {
        unsafe { sys::SBModuleSpecSetPlatformFileSpec(self.raw, filespec.raw) }
    }

    #[allow(missing_docs)]
    pub fn symbol_filespec(&self) -> SBFileSpec {
        SBFileSpec { raw: unsafe { sys::SBModuleSpecGetSymbolFileSpec(self.raw) } }
    }

    #[allow(missing_docs)]
    pub fn set_symbol_filespec(&self, filespec: &SBFileSpec) {
        unsafe { sys::SBModuleSpecSetSymbolFileSpec(self.raw, filespec.raw) }
    }

    #[allow(missing_docs)]
    pub fn object_name(&self) -> &str {
        unimplemented!();
    }

    #[allow(missing_docs)]
    pub fn set_object_name(&self, _object_name: &str) {
        unimplemented!();
    }

    #[allow(missing_docs)]
    pub fn triple(&self) -> &str {
        unimplemented!();
    }

    #[allow(missing_docs)]
    pub fn set_triple(&self, _object_name: &str) {
        unimplemented!();
    }

    #[allow(missing_docs)]
    pub fn uuid_bytes(&self) -> &str {
        unimplemented!();
    }

    #[allow(missing_docs)]
    pub fn set_uuid_bytes(&self, _object_name: &str) {
        unimplemented!();
    }
}
