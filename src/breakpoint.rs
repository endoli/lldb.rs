// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CString;
use std::fmt;
use super::breakpointlocation::SBBreakpointLocation;
use super::stream::SBStream;
use super::stringlist::SBStringList;
use super::lldb_addr_t;
use sys;

/// A logical breakpoint and its associated settings.
///
/// # To Hit or Not
///
/// A breakpoint has multiple ways of controlling whether
/// or not it should be considered active.
///
/// * Enabled. This is controlled via [`is_enabled`] and
///   [`set_enabled`].
/// * One shot. If set, this will be disabled once it has
///   been hit. This is controlled via [`is_oneshot`] and
///   [`set_oneshot`].
/// * Ignore count. If set, this breakpoint will be ignored
///   the first *ignore count* times that it is hit. This is
///   controlled via [`ignore_count`] and [`set_ignore_count`].
///
/// A count of how many times a breakpoint has been it is
/// available via [`hit_count`].
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
/// Names are managed via [`add_name`], [`remove_name`],
/// [`matches_name`] and [`names`].
///
/// # Breakpoint Locations
///
/// ...
///
/// [`is_enabled`]: #method.is_enabled
/// [`set_enabled`]: #method.set_enabled
/// [`is_oneshot`]: #method.is_oneshot
/// [`set_oneshot`]: #method.set_oneshot
/// [`ignore_count`]: #method.ignore_count
/// [`set_ignore_count`]: #method.set_ignore_count
/// [`hit_count`]: #method.hit_count
/// [`add_name`]: #method.add_name
/// [`remove_name`]: #method.remove_name
/// [`matches_name`]: #method.matches_name
/// [`names`]: #method.names
pub struct SBBreakpoint {
    /// The underlying raw `SBBreakpointRef`.
    pub raw: sys::SBBreakpointRef,
}

impl SBBreakpoint {
    /// Construct a new `SBBreakpoint`.
    pub fn wrap(raw: sys::SBBreakpointRef) -> SBBreakpoint {
        SBBreakpoint { raw: raw }
    }

    /// Construct a new `Some(SBBreakpoint)` or `None`.
    pub fn maybe_wrap(raw: sys::SBBreakpointRef) -> Option<SBBreakpoint> {
        if unsafe { sys::SBBreakpointIsValid(raw) != 0 } {
            Some(SBBreakpoint { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBBreakpoint` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBBreakpointIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn is_enabled(&self) -> bool {
        unsafe { sys::SBBreakpointIsEnabled(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::SBBreakpointSetEnabled(self.raw, enabled as u8) }
    }

    #[allow(missing_docs)]
    pub fn is_oneshot(&self) -> bool {
        unsafe { sys::SBBreakpointIsOneShot(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn set_oneshot(&self, oneshot: bool) {
        unsafe { sys::SBBreakpointSetOneShot(self.raw, oneshot as u8) }
    }

    #[allow(missing_docs)]
    pub fn is_internal(&self) -> bool {
        unsafe { sys::SBBreakpointIsInternal(self.raw) != 0 }
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
        unsafe { sys::SBBreakpointAddName(self.raw, name.as_ptr()) != 0 }
    }

    #[allow(missing_docs)]
    pub fn remove_name(&self, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe { sys::SBBreakpointRemoveName(self.raw, name.as_ptr()) };
    }

    #[allow(missing_docs)]
    pub fn matches_name(&self, name: &str) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { sys::SBBreakpointMatchesName(self.raw, name.as_ptr()) != 0 }
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

/// An iterator over the [locations] in an [`SBBreakpoint`].
///
/// [locations]: struct.SBBreakpointLocation.html
/// [`SBBreakpoint`]: struct.SBBreakpoint.html
pub struct SBBreakpointLocationIter<'d> {
    breakpoint: &'d SBBreakpoint,
    idx: usize,
}

impl<'d> Iterator for SBBreakpointLocationIter<'d> {
    type Item = SBBreakpointLocation;

    fn next(&mut self) -> Option<SBBreakpointLocation> {
        if self.idx < unsafe { sys::SBBreakpointGetNumLocations(self.breakpoint.raw) as usize } {
            let r = SBBreakpointLocation::maybe_wrap(unsafe {
                sys::SBBreakpointGetLocationAtIndex(self.breakpoint.raw, self.idx as u32)
            });
            self.idx += 1;
            r
        } else {
            None
        }
    }
}
