// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, Permissions, SBData, SBStream, SBTarget};
use std::ffi::{CStr, CString};
use std::fmt;

/// Represents an executable image section.
///
/// `SBSection` supports iteration through its [subsections],
/// represented as `SBSection` as well.
///
/// See also:
///
/// - [`SBModule`](crate::SBModule)
/// - [`SBModule::find_section()`](crate::SBModule::find_section)
/// - [`SBModule::sections()`](crate::SBModule::sections)
///
/// [subsections]: SBSection::subsections
pub struct SBSection {
    /// The underlying raw `SBSectionRef`.
    pub raw: sys::SBSectionRef,
}

impl SBSection {
    /// Construct a new `SBSection`.
    pub(crate) fn wrap(raw: sys::SBSectionRef) -> SBSection {
        SBSection { raw }
    }

    /// Construct a new `Some(SBSection)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBSectionRef) -> Option<SBSection> {
        if unsafe { sys::SBSectionIsValid(raw) } {
            Some(SBSection { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBSection` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSectionIsValid(self.raw) }
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
    /// [subsections]: SBSection
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

    /// Gets the permissions (RWX) of the section of the object file.
    ///
    /// `None` is returned for sections without permissions. Invalid
    /// permissions bits are truncated.
    pub fn permissions(&self) -> Option<Permissions> {
        let perms = unsafe { sys::SBSectionGetPermissions(self.raw) };
        if perms != 0 {
            Some(Permissions::from_bits_truncate(perms))
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn target_byte_size(&self) -> u32 {
        unsafe { sys::SBSectionGetTargetByteSize(self.raw) }
    }
}

/// Iterate over the [subsections] in a [section].
///
/// [subsections]: SBSection
/// [section]: SBSection
pub struct SBSectionSubSectionIter<'d> {
    section: &'d SBSection,
    idx: usize,
}

impl<'d> Iterator for SBSectionSubSectionIter<'d> {
    type Item = SBSection;

    fn next(&mut self) -> Option<SBSection> {
        if self.idx < unsafe { sys::SBSectionGetNumSubSections(self.section.raw) } {
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
        let sz = unsafe { sys::SBSectionGetNumSubSections(self.section.raw) };
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBSectionSubSectionIter<'d> {}

impl Clone for SBSection {
    fn clone(&self) -> SBSection {
        SBSection {
            raw: unsafe { sys::CloneSBSection(self.raw) },
        }
    }
}

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

unsafe impl Send for SBSection {}
unsafe impl Sync for SBSection {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBSection {
    fn name() -> &str {
        self.name()
    }

    fn subsections() -> Vec<SBSection> {
        self.subsections().collect()
    }

    fn file_address() -> i32 {
        self.file_address() as i32
    }

    fn byte_size() -> i32 {
        self.byte_size() as i32
    }

    fn file_offset() -> i32 {
        self.file_offset() as i32
    }

    fn file_byte_size() -> i32 {
        self.file_byte_size() as i32
    }

    fn target_byte_size() -> i32 {
        self.target_byte_size() as i32
    }
}
