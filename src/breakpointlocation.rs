// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::address::SBAddress;
use super::breakpoint::SBBreakpoint;
use super::stream::SBStream;
use super::{lldb_addr_t, DescriptionLevel};
use std::fmt;
use sys;

/// One unique instance (by address) of a logical breakpoint.
///
/// A breakpoint location is defined by the breakpoint that
/// produces it, and the address that resulted in this
/// particular instantiation.  Each breakpoint location has
/// its settable options.
///
/// `SBBreakpoint` contains `SBBreakpointLocation`(s).
/// See [`SBBreakpoint`] for retrieval of an `SBBreakpointLocation`
/// from an `SBBreakpoint`.
///
/// [`SBBreakpoint`]: struct.SBBreakpoint.html
pub struct SBBreakpointLocation {
    /// The underlying raw `SBBreakpointLocationRef`.
    pub raw: sys::SBBreakpointLocationRef,
}

impl SBBreakpointLocation {
    /// Construct a new `Some(SBBreakpointLocation)` or `None`.
    pub fn maybe_wrap(raw: sys::SBBreakpointLocationRef) -> Option<SBBreakpointLocation> {
        if unsafe { sys::SBBreakpointLocationIsValid(raw) } {
            Some(SBBreakpointLocation { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBBreakpointLocation` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBBreakpointLocationIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn id(&self) -> i32 {
        unsafe { sys::SBBreakpointLocationGetID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn address(&self) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBBreakpointLocationGetAddress(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn load_address(&self) -> lldb_addr_t {
        unsafe { sys::SBBreakpointLocationGetLoadAddress(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_enabled(&self) -> bool {
        unsafe { sys::SBBreakpointLocationIsEnabled(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::SBBreakpointLocationSetEnabled(self.raw, enabled) }
    }

    #[allow(missing_docs)]
    pub fn hit_count(&self) -> u32 {
        unsafe { sys::SBBreakpointLocationGetHitCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn ignore_count(&self) -> u32 {
        unsafe { sys::SBBreakpointLocationGetIgnoreCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_ignore_count(&self, count: u32) {
        unsafe { sys::SBBreakpointLocationSetIgnoreCount(self.raw, count) }
    }

    #[allow(missing_docs)]
    pub fn is_resolved(&self) -> bool {
        unsafe { sys::SBBreakpointLocationIsResolved(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn breakpoint(&self) -> SBBreakpoint {
        SBBreakpoint::from(unsafe { sys::SBBreakpointLocationGetBreakpoint(self.raw) })
    }
}

impl Clone for SBBreakpointLocation {
    fn clone(&self) -> SBBreakpointLocation {
        SBBreakpointLocation {
            raw: unsafe { sys::CloneSBBreakpointLocation(self.raw) },
        }
    }
}

impl fmt::Debug for SBBreakpointLocation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe {
            sys::SBBreakpointLocationGetDescription(self.raw, stream.raw, DescriptionLevel::Brief)
        };
        write!(fmt, "SBBreakpointLocation {{ {} }}", stream.data())
    }
}

impl Drop for SBBreakpointLocation {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBBreakpointLocation(self.raw) };
    }
}

impl From<sys::SBBreakpointLocationRef> for SBBreakpointLocation {
    fn from(raw: sys::SBBreakpointLocationRef) -> SBBreakpointLocation {
        SBBreakpointLocation { raw }
    }
}

unsafe impl Send for SBBreakpointLocation {}
unsafe impl Sync for SBBreakpointLocation {}

#[cfg(feature = "graphql")]
graphql_object!(SBBreakpointLocation: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field id() -> i32 {
        self.id()
    }

    field address() -> Option<SBAddress> {
        self.address()
    }

    // TODO(bm) This should be u64
    field load_address() -> i32 {
        self.load_address() as i32
    }

    field is_enabled() -> bool {
        self.is_enabled()
    }

    // TODO(bm) This should be u32
    field ignore_count() -> i32 {
        self.ignore_count() as i32
    }

    field is_resolved() -> bool {
        self.is_resolved()
    }

    field breakpoint() -> SBBreakpoint {
        self.breakpoint()
    }
});
