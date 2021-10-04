// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, LanguageType, SBFileSpec, SBLineEntry, SBStream, SBTypeList, TypeClass};
use std::fmt;

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
        if unsafe { sys::SBCompileUnitIsValid(raw) } {
            Some(SBCompileUnit { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBCompileUnit` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBCompileUnitIsValid(self.raw) }
    }

    /// The source file for the compile unit.
    pub fn filespec(&self) -> SBFileSpec {
        SBFileSpec::from(unsafe { sys::SBCompileUnitGetFileSpec(self.raw) })
    }

    /// The [line entries][SBLineEntry] for the compilation unit.
    ///
    /// These come from the line table in the debug data.
    pub fn line_entries(&self) -> SBCompileUnitLineEntryIter {
        SBCompileUnitLineEntryIter {
            source: self,
            idx: 0,
        }
    }

    /// Get all types matching `type_mask` from the debug info in this
    /// compile unit.
    ///
    /// `type_mask` is a bitfield consisting of one or more type classes.
    /// This allows you to request only structure types, or only class,
    /// structure, and union types. Passing in [`TypeClass::ANY`] will
    /// return all types found in the debug information for this compile
    /// unit.
    pub fn types(&self, type_mask: TypeClass) -> SBTypeList {
        SBTypeList::from(unsafe { sys::SBCompileUnitGetTypes(self.raw, type_mask.bits()) })
    }

    /// The language for the compile unit.
    pub fn language(&self) -> LanguageType {
        unsafe { sys::SBCompileUnitGetLanguage(self.raw) }
    }
}

impl Clone for SBCompileUnit {
    fn clone(&self) -> SBCompileUnit {
        SBCompileUnit {
            raw: unsafe { sys::CloneSBCompileUnit(self.raw) },
        }
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

impl From<sys::SBCompileUnitRef> for SBCompileUnit {
    fn from(raw: sys::SBCompileUnitRef) -> SBCompileUnit {
        SBCompileUnit { raw }
    }
}

unsafe impl Send for SBCompileUnit {}
unsafe impl Sync for SBCompileUnit {}

/// Iterate over the [line entries] in a [compile unit].
///
/// [line entries]: SBLineEntry
/// [compile unit]: SBCompileUnit
pub struct SBCompileUnitLineEntryIter<'d> {
    source: &'d SBCompileUnit,
    idx: u32,
}

impl<'d> Iterator for SBCompileUnitLineEntryIter<'d> {
    type Item = SBLineEntry;

    fn next(&mut self) -> Option<SBLineEntry> {
        if self.idx < unsafe { sys::SBCompileUnitGetNumLineEntries(self.source.raw) } {
            let r = Some(SBLineEntry::from(unsafe {
                sys::SBCompileUnitGetLineEntryAtIndex(self.source.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBCompileUnitGetNumLineEntries(self.source.raw) } as usize;
        (sz - self.idx as usize, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBCompileUnitLineEntryIter<'d> {}

#[cfg(feature = "graphql")]
graphql_object!(SBCompileUnit: crate::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field filespec() -> SBFileSpec {
        self.filespec()
    }
});
