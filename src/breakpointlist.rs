// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This bkpt may not be copied, modified, or distributed
// except according to those terms.

use super::breakpoint::SBBreakpoint;
use super::target::SBTarget;
use sys;

/// A list of [breakpoints].
///
/// [breakpoints]: struct.SBBreakpoint.html
pub struct SBBreakpointList {
    /// The underlying raw `SBBreakpointListRef`.
    pub raw: sys::SBBreakpointListRef,
}

impl SBBreakpointList {
    /// Construct a new `SBBreakpointList`.
    pub fn new(target: &SBTarget) -> SBBreakpointList {
        SBBreakpointList::wrap(unsafe { sys::CreateSBBreakpointList(target.raw) })
    }

    /// Construct a new `SBBreakpointList`.
    pub fn wrap(raw: sys::SBBreakpointListRef) -> SBBreakpointList {
        SBBreakpointList { raw }
    }

    #[allow(missing_docs)]
    pub fn find_breakpoint_by_id(&self, id: i32) -> Option<SBBreakpoint> {
        SBBreakpoint::maybe_wrap(unsafe { sys::SBBreakpointListFindBreakpointByID(self.raw, id) })
    }

    #[allow(missing_docs)]
    pub fn append(&self, bkpt: &SBBreakpoint) {
        unsafe { sys::SBBreakpointListAppend(self.raw, bkpt.raw) };
    }

    #[allow(missing_docs)]
    pub fn append_by_id(&self, bkpt_id: i32) {
        unsafe { sys::SBBreakpointListAppendByID(self.raw, bkpt_id) };
    }

    #[allow(missing_docs)]
    pub fn append_if_unique(&self, bkpt: &SBBreakpoint) {
        unsafe { sys::SBBreakpointListAppendIfUnique(self.raw, bkpt.raw) };
    }

    /// Is this breakpoint list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBBreakpointListGetSize(self.raw) == 0 }
    }

    /// Clear this breakpoint list.
    pub fn clear(&self) {
        unsafe { sys::SBBreakpointListClear(self.raw) };
    }

    /// Iterate over this breakpoint list.
    pub fn iter(&self) -> SBBreakpointListIter {
        SBBreakpointListIter {
            breakpoint_list: self,
            idx: 0,
        }
    }
}

impl Clone for SBBreakpointList {
    fn clone(&self) -> SBBreakpointList {
        SBBreakpointList {
            raw: unsafe { sys::CloneSBBreakpointList(self.raw) },
        }
    }
}

impl Drop for SBBreakpointList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBBreakpointList(self.raw) };
    }
}

unsafe impl Send for SBBreakpointList {}
unsafe impl Sync for SBBreakpointList {}

/// An iterator over the [breakpoints] in an [`SBBreakpointList`].
///
/// [breakpoints]: struct.SBBreakpoint.html
/// [`SBBreakpointList`]: struct.SBBreakpointList.html
pub struct SBBreakpointListIter<'d> {
    breakpoint_list: &'d SBBreakpointList,
    idx: usize,
}

impl<'d> Iterator for SBBreakpointListIter<'d> {
    type Item = SBBreakpoint;

    fn next(&mut self) -> Option<SBBreakpoint> {
        if self.idx < unsafe { sys::SBBreakpointListGetSize(self.breakpoint_list.raw) } {
            let r = SBBreakpoint::wrap(unsafe {
                sys::SBBreakpointListGetBreakpointAtIndex(self.breakpoint_list.raw, self.idx)
            });
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBBreakpointListGetSize(self.breakpoint_list.raw) };
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBBreakpointListIter<'d> {}
