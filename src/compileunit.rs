// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::filespec::SBFileSpec;
use super::stream::SBStream;
use super::LanguageType;
use std::fmt;
use sys;

/// A compilation unit or compiled source file.
pub struct SBCompileUnit {
    /// The underlying raw `SBCompileUnitRef`.
    pub raw: sys::SBCompileUnitRef,
}

impl SBCompileUnit {
    /// Construct a new `SBCompileUnit`.
    pub fn wrap(raw: sys::SBCompileUnitRef) -> SBCompileUnit {
        SBCompileUnit { raw }
    }

    /// Construct a new `Some(SBCompileUnit)` or `None`.
    pub fn maybe_wrap(raw: sys::SBCompileUnitRef) -> Option<SBCompileUnit> {
        if unsafe { sys::SBCompileUnitIsValid(raw) != 0 } {
            Some(SBCompileUnit { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBCompileUnit` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBCompileUnitIsValid(self.raw) != 0 }
    }

    /// The source file for the compile unit.
    pub fn filespec(&self) -> SBFileSpec {
        SBFileSpec::wrap(unsafe { sys::SBCompileUnitGetFileSpec(self.raw) })
    }

    /// The language for the compile unit.
    pub fn language(&self) -> LanguageType {
        unsafe { sys::SBCompileUnitGetLanguage(self.raw) }
    }
}

impl fmt::Debug for SBCompileUnit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBCompileUnitGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBCompileUnit {{ {} }}", stream.data())
    }
}

impl Drop for SBCompileUnit {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBCompileUnit(self.raw) };
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBCompileUnit: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field filespec() -> SBFileSpec {
        self.filespec()
    }
});
