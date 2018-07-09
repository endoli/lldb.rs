// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::filespec::SBFileSpec;
use super::stream::SBStream;
use std::fmt;
use sys;

/// A list of [filespecs].
///
/// [filespecs]: struct.SBFileSpec.html
pub struct SBFileSpecList {
    /// The underlying raw `SBFileSpecListRef`.
    pub raw: sys::SBFileSpecListRef,
}

impl SBFileSpecList {
    /// Construct a new `SBFileSpecList`
    pub fn new() -> SBFileSpecList {
        SBFileSpecList::wrap(unsafe { sys::CreateSBFileSpecList() })
    }

    /// Construct a new `SBFileSpecList`.
    pub fn wrap(raw: sys::SBFileSpecListRef) -> SBFileSpecList {
        SBFileSpecList { raw }
    }

    #[allow(missing_docs)]
    pub fn append(&self, file: &SBFileSpec) {
        unsafe { sys::SBFileSpecListAppend(self.raw, file.raw) };
    }

    #[allow(missing_docs)]
    pub fn append_if_unique(&self, file: &SBFileSpec) {
        unsafe { sys::SBFileSpecListAppendIfUnique(self.raw, file.raw) };
    }

    /// Is this filespec list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBFileSpecListGetSize(self.raw) == 0 }
    }

    /// Clear this filespec list.
    pub fn clear(&self) {
        unsafe { sys::SBFileSpecListClear(self.raw) };
    }

    /// Iterate over this filespec list.
    pub fn iter(&self) -> SBFileSpecListIter {
        SBFileSpecListIter {
            filespec_list: self,
            idx: 0,
        }
    }
}

impl fmt::Debug for SBFileSpecList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBFileSpecListGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBFileSpecList {{ {} }}", stream.data())
    }
}

impl Default for SBFileSpecList {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SBFileSpecList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBFileSpecList(self.raw) };
    }
}

/// An iterator over the [filespecs] in an [`SBFileSpecList`].
///
/// [filespecs]: struct.SBFileSpec.html
/// [`SBFileSpecList`]: struct.SBFileSpecList.html
pub struct SBFileSpecListIter<'d> {
    filespec_list: &'d SBFileSpecList,
    idx: usize,
}

impl<'d> Iterator for SBFileSpecListIter<'d> {
    type Item = SBFileSpec;

    fn next(&mut self) -> Option<SBFileSpec> {
        if self.idx < unsafe { sys::SBFileSpecListGetSize(self.filespec_list.raw) as usize } {
            let r = SBFileSpec::wrap(unsafe {
                sys::SBFileSpecListGetFileSpecAtIndex(self.filespec_list.raw, self.idx as u32)
            });
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBFileSpecListGetSize(self.filespec_list.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBFileSpecListIter<'d> {}
