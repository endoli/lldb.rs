// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    lldb_addr_t, sys, SBBreakpointLocation, SBStream, SBStringList, SBStructuredData, SBTarget,
};
use std::ffi::CString;
use std::fmt;

/// A logical breakpoint and its associated settings.
///
/// # To Hit or Not
///
/// A breakpoint has multiple ways of controlling whether
/// or not it should be considered active.
///
/// * Enabled. This is controlled via [`SBBreakpoint::is_enabled()`]
///   and [`SBBreakpoint::set_enabled()`].
/// * One shot. If set, this will be disabled once it has
///   been hit. This is controlled via [`SBBreakpoint::is_oneshot()`]
///   and [`SBBreakpoint::set_oneshot()`].
/// * Ignore count. If set, this breakpoint will be ignored
///   the first *ignore count* times that it is hit. This is
///   controlled via [`SBBreakpoint::ignore_count()`] and
///   [`SBBreakpoint::set_ignore_count()`].
///
/// A count of how many times a breakpoint has been it is
/// available via [`SBBreakpoint::hit_count()`].
///
/// # Breakpoint Names and Aliases
///
/// Breakpoints can have names associated with them. These are
/// actually more like tags in that the same name can be applied
/// to multiple breakpoints so that a single command invocation
/// can work on multiple breakpoints at once.
///
/// A common use case for this is setting up families of breakpoints,
/// for example on `malloc`, `realloc`, and `free` and giving them
/// all a name of `memory`. Then, you can make it easy for the user
/// enable or disable them all in a single shot.
///
/// Names are managed via [`SBBreakpoint::add_name()`],
/// [`SBBreakpoint::remove_name()`], [`SBBreakpoint::matches_name()`]
/// and [`SBBreakpoint::names()`].
///
/// # Breakpoint Locations
///
/// ...
pub struct SBBreakpoint {
    /// The underlying raw `SBBreakpointRef`.
    pub raw: sys::SBBreakpointRef,
}

impl SBBreakpoint {
    /// Construct a new `SBBreakpoint`.
    pub(crate) fn wrap(raw: sys::SBBreakpointRef) -> SBBreakpoint {
        SBBreakpoint { raw }
    }

    /// Construct a new `Some(SBBreakpoint)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBBreakpointRef) -> Option<SBBreakpoint> {
        if unsafe { sys::SBBreakpointIsValid(raw) } {
            Some(SBBreakpoint { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBBreakpoint` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBBreakpointIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn id(&self) -> i32 {
        unsafe { sys::SBBreakpointGetID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_enabled(&self) -> bool {
        unsafe { sys::SBBreakpointIsEnabled(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::SBBreakpointSetEnabled(self.raw, enabled) }
    }

    #[allow(missing_docs)]
    pub fn is_oneshot(&self) -> bool {
        unsafe { sys::SBBreakpointIsOneShot(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_oneshot(&self, oneshot: bool) {
        unsafe { sys::SBBreakpointSetOneShot(self.raw, oneshot) }
    }

    #[allow(missing_docs)]
    pub fn is_internal(&self) -> bool {
        unsafe { sys::SBBreakpointIsInternal(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn hit_count(&self) -> u32 {
        unsafe { sys::SBBreakpointGetHitCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn ignore_count(&self) -> u32 {
        unsafe { sys::SBBreakpointGetIgnoreCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_ignore_count(&self, count: u32) {
        unsafe { sys::SBBreakpointSetIgnoreCount(self.raw, count) }
    }

    #[allow(missing_docs)]
    pub fn add_name(&self, name: &str) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { sys::SBBreakpointAddName(self.raw, name.as_ptr()) }
    }

    #[allow(missing_docs)]
    pub fn remove_name(&self, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe { sys::SBBreakpointRemoveName(self.raw, name.as_ptr()) };
    }

    #[allow(missing_docs)]
    pub fn matches_name(&self, name: &str) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { sys::SBBreakpointMatchesName(self.raw, name.as_ptr()) }
    }

    #[allow(missing_docs)]
    pub fn names(&self) -> SBStringList {
        let names = SBStringList::new();
        unsafe { sys::SBBreakpointGetNames(self.raw, names.raw) };
        names
    }

    #[allow(missing_docs)]
    pub fn clear_all_breakpoint_sites(&self) {
        unsafe { sys::SBBreakpointClearAllBreakpointSites(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn target(&self) -> Option<SBTarget> {
        SBTarget::maybe_wrap(unsafe { sys::SBBreakpointGetTarget(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn find_location_by_address(&self, address: lldb_addr_t) -> Option<SBBreakpointLocation> {
        SBBreakpointLocation::maybe_wrap(unsafe {
            sys::SBBreakpointFindLocationByAddress(self.raw, address)
        })
    }

    #[allow(missing_docs)]
    pub fn find_location_id_by_address(&self, address: lldb_addr_t) -> i32 {
        unsafe { sys::SBBreakpointFindLocationIDByAddress(self.raw, address) }
    }

    #[allow(missing_docs)]
    pub fn find_location_by_id(&self, id: i32) -> Option<SBBreakpointLocation> {
        SBBreakpointLocation::maybe_wrap(unsafe { sys::SBBreakpointFindLocationByID(self.raw, id) })
    }

    #[allow(missing_docs)]
    pub fn locations(&self) -> SBBreakpointLocationIter {
        SBBreakpointLocationIter {
            breakpoint: self,
            idx: 0,
        }
    }

    #[allow(missing_docs)]
    pub fn is_hardware(&self) -> bool {
        unsafe { sys::SBBreakpointIsHardware(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn serialize_to_structured_data(&self) -> SBStructuredData {
        SBStructuredData::wrap(unsafe { sys::SBBreakpointSerializeToStructuredData(self.raw) })
    }
}

impl Clone for SBBreakpoint {
    fn clone(&self) -> SBBreakpoint {
        SBBreakpoint {
            raw: unsafe { sys::CloneSBBreakpoint(self.raw) },
        }
    }
}

impl fmt::Debug for SBBreakpoint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBBreakpointGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBBreakpoint {{ {} }}", stream.data())
    }
}

impl Drop for SBBreakpoint {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBBreakpoint(self.raw) };
    }
}

unsafe impl Send for SBBreakpoint {}
unsafe impl Sync for SBBreakpoint {}

/// An iterator over the [locations] in an [`SBBreakpoint`].
///
/// [locations]: SBBreakpointLocation
pub struct SBBreakpointLocationIter<'d> {
    breakpoint: &'d SBBreakpoint,
    idx: usize,
}

impl<'d> Iterator for SBBreakpointLocationIter<'d> {
    type Item = SBBreakpointLocation;

    fn next(&mut self) -> Option<SBBreakpointLocation> {
        if self.idx < unsafe { sys::SBBreakpointGetNumLocations(self.breakpoint.raw) } {
            let r = SBBreakpointLocation::maybe_wrap(unsafe {
                sys::SBBreakpointGetLocationAtIndex(self.breakpoint.raw, self.idx as u32)
            });
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBBreakpointGetNumLocations(self.breakpoint.raw) };
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBBreakpointLocationIter<'d> {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBBreakpoint {
    fn id() -> i32 {
        self.id()
    }

    fn is_enabled() -> bool {
        self.is_enabled()
    }

    fn is_oneshot() -> bool {
        self.is_oneshot()
    }

    fn is_internal() -> bool {
        self.is_internal()
    }

    // TODO(bm) This should be u32
    fn ignore_count() -> i32 {
        self.ignore_count() as i32
    }

    // TODO(bm) This should be u32
    fn hit_count() -> i32 {
        self.hit_count() as i32
    }

    // TODO(bm) Make this work. (Lifetimes.)
    // fn names() -> Vec<&str> {
    //     self.names().iter().collect()
    // }

    fn locations() -> Vec<SBBreakpointLocation> {
        self.locations().collect()
    }
}
