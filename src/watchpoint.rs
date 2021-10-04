// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{lldb_addr_t, sys, DescriptionLevel, SBError, SBStream};
use std::fmt;

/// An instance of a watch point for a specific target program.
///
/// A watchpoint is determined by the address the byte size that
/// resulted in this particular instantiation. Each watchpoint has
/// its own settable options.
///
/// # To Hit or Not
///
/// A watchpoint has multiple ways of controlling whether
/// or not it should be considered active.
///
/// * Enabled. This is controlled via [`is_enabled`] and
///   [`set_enabled`].
/// * Ignore count. If set, this watchpoint will be ignored
///   the first *ignore count* times that it is hit. This is
///   controlled via [`ignore_count`] and [`set_ignore_count`].
///
/// A count of how many times a watchpoint has been it is
/// available via [`hit_count`].
///
/// [`is_enabled`]: #method.is_enabled
/// [`set_enabled`]: #method.set_enabled
/// [`ignore_count`]: #method.ignore_count
/// [`set_ignore_count`]: #method.set_ignore_count
/// [`hit_count`]: #method.hit_count
pub struct SBWatchpoint {
    /// The underlying raw `SBWatchpointRef`.
    pub raw: sys::SBWatchpointRef,
}

impl SBWatchpoint {
    /// Construct a new `Some(SBWatchpoint)` or `None`.
    pub fn maybe_wrap(raw: sys::SBWatchpointRef) -> Option<SBWatchpoint> {
        if unsafe { sys::SBWatchpointIsValid(raw) } {
            Some(SBWatchpoint { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBWatchpoint` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBWatchpointIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn error(&self) -> Option<SBError> {
        SBError::maybe_wrap(unsafe { sys::SBWatchpointGetError(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn id(&self) -> i32 {
        unsafe { sys::SBWatchpointGetID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn hardware_index(&self) -> Option<i32> {
        let idx = unsafe { sys::SBWatchpointGetHardwareIndex(self.raw) };
        if idx == -1 {
            None
        } else {
            Some(idx)
        }
    }

    #[allow(missing_docs)]
    pub fn watch_address(&self) -> lldb_addr_t {
        unsafe { sys::SBWatchpointGetWatchAddress(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn watch_size(&self) -> usize {
        unsafe { sys::SBWatchpointGetWatchSize(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn is_enabled(&self) -> bool {
        unsafe { sys::SBWatchpointIsEnabled(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe { sys::SBWatchpointSetEnabled(self.raw, enabled) }
    }

    #[allow(missing_docs)]
    pub fn hit_count(&self) -> u32 {
        unsafe { sys::SBWatchpointGetHitCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn ignore_count(&self) -> u32 {
        unsafe { sys::SBWatchpointGetIgnoreCount(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_ignore_count(&self, count: u32) {
        unsafe { sys::SBWatchpointSetIgnoreCount(self.raw, count) }
    }
}

impl Clone for SBWatchpoint {
    fn clone(&self) -> SBWatchpoint {
        SBWatchpoint {
            raw: unsafe { sys::CloneSBWatchpoint(self.raw) },
        }
    }
}

impl fmt::Debug for SBWatchpoint {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBWatchpointGetDescription(self.raw, stream.raw, DescriptionLevel::Brief) };
        write!(fmt, "SBWatchpoint {{ {} }}", stream.data())
    }
}

impl Drop for SBWatchpoint {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBWatchpoint(self.raw) };
    }
}

impl From<sys::SBWatchpointRef> for SBWatchpoint {
    fn from(raw: sys::SBWatchpointRef) -> SBWatchpoint {
        SBWatchpoint { raw }
    }
}

unsafe impl Send for SBWatchpoint {}
unsafe impl Sync for SBWatchpoint {}

#[cfg(feature = "graphql")]
graphql_object!(SBWatchpoint: crate::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    field id() -> i32 {
        self.id()
    }

    field hardware_index() -> Option<i32> {
        self.hardware_index()
    }

    // TODO(bm) This should be u64
    field watch_address() -> i32 {
        self.watch_address() as i32
    }

    // TODO(bm) This should be u32
    field watch_size() -> i32 {
        self.watch_size() as i32
    }

    field is_enabled() -> bool {
        self.is_enabled()
    }

    // TODO(bm) This should be u32
    field hit_count() -> i32 {
        self.hit_count() as i32
    }

    // TODO(bm) This should be u32
    field ignore_count() -> i32 {
        self.ignore_count() as i32
    }
});
