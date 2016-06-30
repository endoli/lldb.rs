// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// A logical breakpoint and its associated settings.
#[derive(Debug)]
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
}

impl Drop for SBBreakpoint {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBBreakpoint(self.raw) };
    }
}
