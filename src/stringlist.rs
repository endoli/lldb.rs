// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sys;
use std::ffi::{CStr, CString};

/// A list of strings.
#[derive(Debug)]
pub struct SBStringList {
    /// The underlying raw `SBStringListRef`.
    pub raw: sys::SBStringListRef,
}

impl SBStringList {
    /// Construct a new `SBStringList`.
    pub fn new() -> SBStringList {
        SBStringList::from(unsafe { sys::CreateSBStringList() })
    }

    /// Construct a new `Some(SBStringList)` or `None`.
    pub fn maybe_wrap(raw: sys::SBStringListRef) -> Option<SBStringList> {
        if unsafe { sys::SBStringListIsValid(raw) } {
            Some(SBStringList { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBStringList` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBStringListIsValid(self.raw) }
    }

    /// Is this string list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBStringListGetSize(self.raw) == 0 }
    }

    /// Clear this string list.
    pub fn clear(&self) {
        unsafe { sys::SBStringListClear(self.raw) };
    }

    /// Append another string to this list.
    pub fn append_string(&self, string: &str) {
        let string = CString::new(string).unwrap();
        unsafe { sys::SBStringListAppendString(self.raw, string.as_ptr()) };
    }

    /// Append another string list to this one.
    pub fn append_list(&self, other: &SBStringList) {
        unsafe { sys::SBStringListAppendList2(self.raw, other.raw) };
    }

    /// Iterate over this string list.
    pub fn iter(&self) -> SBStringListIter {
        SBStringListIter {
            string_list: self,
            idx: 0,
        }
    }
}

impl Clone for SBStringList {
    fn clone(&self) -> SBStringList {
        SBStringList {
            raw: unsafe { sys::CloneSBStringList(self.raw) },
        }
    }
}

impl Default for SBStringList {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SBStringList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBStringList(self.raw) };
    }
}

impl From<sys::SBStringListRef> for SBStringList {
    fn from(raw: sys::SBStringListRef) -> SBStringList {
        SBStringList { raw }
    }
}

unsafe impl Send for SBStringList {}
unsafe impl Sync for SBStringList {}

/// An iterator over an `SBStringList`.
pub struct SBStringListIter<'d> {
    string_list: &'d SBStringList,
    idx: usize,
}

impl<'d> Iterator for SBStringListIter<'d> {
    type Item = &'d str;

    fn next(&mut self) -> Option<&'d str> {
        if self.idx < unsafe { sys::SBStringListGetSize(self.string_list.raw) as usize } {
            let r = unsafe {
                match CStr::from_ptr(sys::SBStringListGetStringAtIndex(
                    self.string_list.raw,
                    self.idx,
                ))
                .to_str()
                {
                    Ok(s) => s,
                    _ => panic!("Invalid string?"),
                }
            };
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBStringListGetSize(self.string_list.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBStringListIter<'d> {}
