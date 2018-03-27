// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use super::broadcaster::SBBroadcaster;
use super::stream::SBStream;
use sys;

/// An event.
pub struct SBEvent {
    /// The underlying raw `SBEventRef`.
    pub raw: sys::SBEventRef,
}

impl SBEvent {
    /// Construct a new `SBEvent`.
    pub fn wrap(raw: sys::SBEventRef) -> SBEvent {
        SBEvent { raw }
    }

    /// Construct a new `Some(SBEvent)` or `None`.
    pub fn maybe_wrap(raw: sys::SBEventRef) -> Option<SBEvent> {
        if unsafe { sys::SBEventIsValid(raw) != 0 } {
            Some(SBEvent { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBEvent` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBEventIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn data_flavor(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBEventGetDataFlavor(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn event_type(&self) -> u32 {
        unsafe { sys::SBEventGetType(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn broadcaster(&self) -> SBBroadcaster {
        SBBroadcaster::wrap(unsafe { sys::SBEventGetBroadcaster(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn broadcaster_class(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBEventGetBroadcasterClass(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn broadcaster_matches_ref(&self, broadcaster: &SBBroadcaster) -> bool {
        unsafe { sys::SBEventBroadcasterMatchesRef(self.raw, broadcaster.raw) != 0 }
    }
}

impl fmt::Debug for SBEvent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBEventGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBEvent {{ {} }}", stream.data())
    }
}

impl Drop for SBEvent {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBEvent(self.raw) };
    }
}
