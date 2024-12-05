// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBType};

/// A list of [types].
///
/// [types]: SBType
pub struct SBTypeList {
    /// The underlying raw `SBTypeListRef`.
    pub raw: sys::SBTypeListRef,
}

impl SBTypeList {
    /// Construct a new `SBTypeList`.
    pub(crate) fn wrap(raw: sys::SBTypeListRef) -> SBTypeList {
        SBTypeList { raw }
    }

    #[allow(missing_docs)]
    pub fn append(&self, t: &SBType) {
        unsafe { sys::SBTypeListAppend(self.raw, t.raw) };
    }

    /// Is this type list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBTypeListGetSize(self.raw) == 0 }
    }

    /// Iterate over this type list.
    pub fn iter(&self) -> SBTypeListIter {
        SBTypeListIter {
            type_list: self,
            idx: 0,
        }
    }
}

impl Clone for SBTypeList {
    fn clone(&self) -> SBTypeList {
        SBTypeList {
            raw: unsafe { sys::CloneSBTypeList(self.raw) },
        }
    }
}

impl Drop for SBTypeList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBTypeList(self.raw) };
    }
}

impl<'d> IntoIterator for &'d SBTypeList {
    type IntoIter = SBTypeListIter<'d>;
    type Item = SBType;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl Send for SBTypeList {}
unsafe impl Sync for SBTypeList {}

/// An iterator over the [types] in an [`SBTypeList`].
///
/// [types]: SBType
pub struct SBTypeListIter<'d> {
    type_list: &'d SBTypeList,
    idx: usize,
}

impl Iterator for SBTypeListIter<'_> {
    type Item = SBType;

    fn next(&mut self) -> Option<SBType> {
        if self.idx < unsafe { sys::SBTypeListGetSize(self.type_list.raw) as usize } {
            let r = SBType::wrap(unsafe {
                sys::SBTypeListGetTypeAtIndex(self.type_list.raw, self.idx as u32)
            });
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBTypeListGetSize(self.type_list.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl ExactSizeIterator for SBTypeListIter<'_> {}
