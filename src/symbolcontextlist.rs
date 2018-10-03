// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::symbolcontext::SBSymbolContext;
use sys;

/// A list of [symbol contexts].
///
/// [symbol contexts]: struct.SBSymbolContext.html
#[derive(Debug)]
pub struct SBSymbolContextList {
    /// The underlying raw `SBSymbolContextListRef`.
    pub raw: sys::SBSymbolContextListRef,
}

impl SBSymbolContextList {
    /// Construct a new `SBSymbolContextList`.
    pub fn wrap(raw: sys::SBSymbolContextListRef) -> SBSymbolContextList {
        SBSymbolContextList { raw }
    }

    /// Construct a new `Some(SBSymbolContextList)` or `None`.
    pub fn maybe_wrap(raw: sys::SBSymbolContextListRef) -> Option<SBSymbolContextList> {
        if unsafe { sys::SBSymbolContextListIsValid(raw) != 0 } {
            Some(SBSymbolContextList { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBSymbolContextList`.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSymbolContextListIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn append(&self, context: &SBSymbolContext) {
        unsafe { sys::SBSymbolContextListAppend(self.raw, context.raw) };
    }

    #[allow(missing_docs)]
    pub fn append_list(&self, contexts: &SBSymbolContextList) {
        unsafe { sys::SBSymbolContextListAppendList(self.raw, contexts.raw) };
    }

    /// Is this context list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBSymbolContextListGetSize(self.raw) == 0 }
    }

    /// Clear this context list.
    pub fn clear(&self) {
        unsafe { sys::SBSymbolContextListClear(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn get_context_at_index(&self, idx: u32) -> Option<SBSymbolContext> {
        SBSymbolContext::maybe_wrap(unsafe {
            sys::SBSymbolContextListGetContextAtIndex(self.raw, idx)
        })
    }

    /// Iterate over this context list.
    pub fn iter(&self) -> SBSymbolContextListIter {
        SBSymbolContextListIter {
            context_list: self,
            idx: 0,
        }
    }
}

impl Clone for SBSymbolContextList {
    fn clone(&self) -> SBSymbolContextList {
        SBSymbolContextList {
            raw: unsafe { sys::CloneSBSymbolContextList(self.raw) },
        }
    }
}

impl Drop for SBSymbolContextList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBSymbolContextList(self.raw) };
    }
}

unsafe impl Send for SBSymbolContextList {}
unsafe impl Sync for SBSymbolContextList {}

/// An iterator over the [contexts] in an [`SBSymbolContextList`].
///
/// [contexts]: struct.SBSymbolContext.html
/// [`SBSymbolContextList`]: struct.SBSymbolContextList.html
pub struct SBSymbolContextListIter<'d> {
    context_list: &'d SBSymbolContextList,
    idx: usize,
}

impl<'d> Iterator for SBSymbolContextListIter<'d> {
    type Item = SBSymbolContext;

    fn next(&mut self) -> Option<SBSymbolContext> {
        if self.idx < unsafe { sys::SBSymbolContextListGetSize(self.context_list.raw) as usize } {
            let r = SBSymbolContext::wrap(unsafe {
                sys::SBSymbolContextListGetContextAtIndex(self.context_list.raw, self.idx as u32)
            });
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBSymbolContextListGetSize(self.context_list.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBSymbolContextListIter<'d> {}
