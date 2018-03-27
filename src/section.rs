// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::{CStr, CString};
use std::fmt;
use super::data::SBData;
use super::stream::SBStream;
use super::target::SBTarget;
use sys;

#[allow(missing_docs)]
pub struct SBSection {
    /// The underlying raw `SBSectionRef`.
    pub raw: sys::SBSectionRef,
}

impl SBSection {
    /// Construct a new `SBSection`.
    pub fn wrap(raw: sys::SBSectionRef) -> SBSection {
        SBSection { raw }
    }

    /// Construct a new `Some(SBSection)` or `None`.
    pub fn maybe_wrap(raw: sys::SBSectionRef) -> Option<SBSection> {
        if unsafe { sys::SBSectionIsValid(raw) != 0 } {
            Some(SBSection { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBSection` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSectionIsValid(self.raw) != 0 }
    }

    /// The section name.
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBSectionGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The section parent, if there is one.
    pub fn parent(&self) -> Option<SBSection> {
        SBSection::maybe_wrap(unsafe { sys::SBSectionGetParent(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn find_subsection(&self, name: &str) -> Option<SBSection> {
        let name = CString::new(name).unwrap();
        SBSection::maybe_wrap(unsafe { sys::SBSectionFindSubSection(self.raw, name.as_ptr()) })
    }

    /// Get an iterator over the [subsections] known to this section instance.
    ///
    /// [subsections]: struct.SBSection.html
    pub fn subsections(&self) -> SBSectionSubSectionIter {
        SBSectionSubSectionIter {
            section: self,
            idx: 0,
        }
    }

    #[allow(missing_docs)]
    pub fn file_address(&self) -> u64 {
        unsafe { sys::SBSectionGetFileAddress(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn load_address(&self, target: &SBTarget) -> u64 {
        unsafe { sys::SBSectionGetLoadAddress(self.raw, target.raw) }
    }

    #[allow(missing_docs)]
    pub fn byte_size(&self) -> u64 {
        unsafe { sys::SBSectionGetByteSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn file_offset(&self) -> u64 {
        unsafe { sys::SBSectionGetFileOffset(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn file_byte_size(&self) -> u64 {
        unsafe { sys::SBSectionGetFileByteSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn section_data(&self) -> SBData {
        SBData::wrap(unsafe { sys::SBSectionGetSectionData(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn section_data_slice(&self, offset: u64, size: u64) -> SBData {
        SBData::wrap(unsafe { sys::SBSectionGetSectionData2(self.raw, offset, size) })
    }

    #[allow(missing_docs)]
    pub fn section_type(&self) -> sys::SectionType {
        unsafe { sys::SBSectionGetSectionType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn target_byte_size(&self) -> u32 {
        unsafe { sys::SBSectionGetTargetByteSize(self.raw) }
    }
}

/// Iterate over the [subsections] in a [section].
///
/// [subsections]: struct.SBSection.html
/// [section]: struct.SBSection.html
pub struct SBSectionSubSectionIter<'d> {
    section: &'d SBSection,
    idx: usize,
}

impl<'d> Iterator for SBSectionSubSectionIter<'d> {
    type Item = SBSection;

    fn next(&mut self) -> Option<SBSection> {
        if self.idx < unsafe { sys::SBSectionGetNumSubSections(self.section.raw) as usize } {
            let r = Some(SBSection::wrap(unsafe {
                sys::SBSectionGetSubSectionAtIndex(self.section.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBSectionGetNumSubSections(self.section.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBSectionSubSectionIter<'d> {}

impl fmt::Debug for SBSection {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBSectionGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBSection {{ {} }}", stream.data())
    }
}

impl Drop for SBSection {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBSection(self.raw) };
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBSection: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field name() -> &str {
        self.name()
    }

    field subsections() -> Vec<SBSection> {
        self.subsections().collect()
    }

    field file_address() -> i64 {
        self.file_address() as i64
    }

    field byte_size() -> i64 {
        self.byte_size() as i64
    }

    field file_offset() -> i64 {
        self.file_offset() as i64
    }

    field file_byte_size() -> i64 {
        self.file_byte_size() as i64
    }

    field target_byte_size() -> i64 {
        self.target_byte_size() as i64
    }
});
