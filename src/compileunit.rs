// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::filespec::SBFileSpec;
use sys;

/// A compilation unit or compiled source file.
#[derive(Debug)]
pub struct SBCompileUnit {
    /// The underlying raw `SBCompileUnitRef`.
    pub raw: sys::SBCompileUnitRef,
}

impl SBCompileUnit {
    /// Construct a new `SBCompileUnit`.
    pub fn new(raw: sys::SBCompileUnitRef) -> SBCompileUnit {
        SBCompileUnit { raw: raw }
    }

    /// Construct a new `Some(SBCompileUnit)` or `None`.
    pub fn maybe(raw: sys::SBCompileUnitRef) -> Option<SBCompileUnit> {
        if unsafe { sys::SBCompileUnitIsValid(raw) != 0 } {
            Some(SBCompileUnit { raw: raw })
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
        SBFileSpec::new(unsafe { sys::SBCompileUnitGetFileSpec(self.raw) })
    }

    /// The language for the compile unit.
    pub fn language(&self) -> sys::LanguageType {
        unsafe { sys::SBCompileUnitGetLanguage(self.raw) }
    }
}
