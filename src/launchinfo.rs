// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;
use sys;

/// Configuration for launching a process.
///
/// See [`SBTarget::launch`].
///
/// [`SBTarget::launch`]: struct.SBTarget.html#method.launch
#[derive(Debug)]
pub struct SBLaunchInfo {
    /// The underlying raw `SBLaunchInfoRef`.
    pub raw: sys::SBLaunchInfoRef,
}

impl SBLaunchInfo {
    /// Construct a new `SBLaunchInfo`.
    pub fn new() -> SBLaunchInfo {
        SBLaunchInfo::wrap(unsafe { sys::CreateSBLaunchInfo(ptr::null_mut()) })
    }

    /// Construct a new `SBLaunchInfo`.
    pub fn wrap(raw: sys::SBLaunchInfoRef) -> SBLaunchInfo {
        SBLaunchInfo { raw: raw }
    }
}

impl Drop for SBLaunchInfo {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBLaunchInfo(self.raw) };
    }
}
