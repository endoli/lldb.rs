// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::event::SBEvent;
use super::listener::SBListener;
use sys;

/// An entity which can broadcast events.
///
/// A default broadcaster is associated with an `SBCommandInterpreter`,
/// `SBProcess`, and `SBTarget`.
///
/// Use an `SBListener` to listen for events.
#[derive(Debug)]
pub struct SBBroadcaster {
    /// The underlying raw `SBBroadcasterRef`.
    pub raw: sys::SBBroadcasterRef,
}

impl SBBroadcaster {
    /// Construct a new `SBBroadcaster`.
    pub fn new() -> SBBroadcaster {
        SBBroadcaster::wrap(unsafe { sys::CreateSBBroadcaster() })
    }

    /// Construct a new `SBBroadcaster`.
    pub fn wrap(raw: sys::SBBroadcasterRef) -> SBBroadcaster {
        SBBroadcaster { raw: raw }
    }

    /// Construct a new `Some(SBBroadcaster)` or `None`.
    pub fn maybe_wrap(raw: sys::SBBroadcasterRef) -> Option<SBBroadcaster> {
        if unsafe { sys::SBBroadcasterIsValid(raw) != 0 } {
            Some(SBBroadcaster { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBBroadcaster` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBBroadcasterIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn broadcast_event_by_type(&self, event_type: u32, unique: bool) {
        unsafe { sys::SBBroadcasterBroadcastEventByType(self.raw, event_type, unique as u8) };
    }

    #[allow(missing_docs)]
    pub fn broadcast_event(&self, event: &SBEvent, unique: bool) {
        unsafe { sys::SBBroadcasterBroadcastEvent(self.raw, event.raw, unique as u8) };
    }

    #[allow(missing_docs)]
    pub fn add_initial_events_to_listener(&self, listener: &SBListener, requested_events: u32) {
        unsafe {
            sys::SBBroadcasterAddInitialEventsToListener(self.raw, listener.raw, requested_events)
        };
    }

    #[allow(missing_docs)]
    pub fn add_listener(&self, listener: &SBListener, event_mask: u32) -> u32 {
        unsafe { sys::SBBroadcasterAddListener(self.raw, listener.raw, event_mask) }
    }

    #[allow(missing_docs)]
    pub fn event_type_has_listeners(&self, event_type: u32) -> bool {
        unsafe { sys::SBBroadcasterEventTypeHasListeners(self.raw, event_type) != 0 }
    }

    #[allow(missing_docs)]
    pub fn remove_listener(&self, listener: &SBListener, event_mask: u32) -> bool {
        unsafe { sys::SBBroadcasterRemoveListener(self.raw, listener.raw, event_mask) != 0 }
    }
}

impl Drop for SBBroadcaster {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBBroadcaster(self.raw) };
    }
}
