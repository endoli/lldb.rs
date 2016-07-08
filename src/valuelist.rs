// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CString;
use super::value::SBValue;
use super::lldb_user_id_t;
use sys;

/// A list of [values].
///
/// [values]: struct.SBValue.html
#[derive(Debug)]
pub struct SBValueList {
    /// The underlying raw `SBValueListRef`.
    pub raw: sys::SBValueListRef,
}

impl SBValueList {
    /// Construct a new `SBValueList`.
    pub fn wrap(raw: sys::SBValueListRef) -> SBValueList {
        SBValueList { raw: raw }
    }

    /// Construct a new `Some(SBValueList)` or `None`.
    pub fn maybe_wrap(raw: sys::SBValueListRef) -> Option<SBValueList> {
        if unsafe { sys::SBValueListIsValid(raw) != 0 } {
            Some(SBValueList { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBValueList` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBValueListIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn append(&self, value: &SBValue) {
        unsafe { sys::SBValueListAppend(self.raw, value.raw) };
    }

    #[allow(missing_docs)]
    pub fn append_list(&self, values: &SBValueList) {
        unsafe { sys::SBValueListAppend2(self.raw, values.raw) };
    }

    /// Is this value list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBValueListGetSize(self.raw) == 0 }
    }

    /// Clear this value list.
    pub fn clear(&self) {
        unsafe { sys::SBValueListClear(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn find_value_by_uid(&self, uid: lldb_user_id_t) -> Option<SBValue> {
        SBValue::maybe_wrap(unsafe { sys::SBValueListFindValueObjectByUID(self.raw, uid) })
    }

    #[allow(missing_docs)]
    pub fn get_first_value_by_name(&self, name: &str) -> Option<SBValue> {
        let name = CString::new(name).unwrap();
        SBValue::maybe_wrap(unsafe { sys::SBValueListGetFirstValueByName(self.raw, name.as_ptr()) })
    }

    /// Iterate over this value list.
    pub fn iter(&self) -> SBValueListIter {
        SBValueListIter {
            value_list: self,
            idx: 0,
        }
    }
}

impl Drop for SBValueList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBValueList(self.raw) };
    }
}

/// An iterator over the [values] in an [`SBValueList`].
///
/// [values]: struct.SBValue.html
/// [`SBValueList`]: struct.SBValueList.html
pub struct SBValueListIter<'d> {
    value_list: &'d SBValueList,
    idx: usize,
}

impl<'d> Iterator for SBValueListIter<'d> {
    type Item = SBValue;

    fn next(&mut self) -> Option<SBValue> {
        if self.idx < unsafe { sys::SBValueListGetSize(self.value_list.raw) as usize } {
            let r = SBValue::wrap(unsafe {
                sys::SBValueListGetValueAtIndex(self.value_list.raw, self.idx as u32)
            });
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }
}
