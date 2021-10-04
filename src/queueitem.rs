// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBAddress, SBThread};
use std::ffi::CString;

/// A work item enqueued on a libdispatch aka Grand Central
/// Dispatch (GCD) queue.
///
/// Most often, this will be a function or block.
///
/// "enqueued" here means that the work item has been added to a queue
/// but it has not yet started executing.  When it is "dequeued",
/// execution of the item begins.
pub struct SBQueueItem {
    /// The underlying raw `SBQueueItemRef`.
    pub raw: sys::SBQueueItemRef,
}

impl SBQueueItem {
    /// Construct a new `Some(SBQueueItem)` or `None`.
    pub fn maybe_wrap(raw: sys::SBQueueItemRef) -> Option<SBQueueItem> {
        if unsafe { sys::SBQueueItemIsValid(raw) } {
            Some(SBQueueItem { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBQueueItem` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBQueueItemIsValid(self.raw) }
    }

    /// The kind of this work item.
    pub fn kind(&self) -> sys::QueueItemKind {
        unsafe { sys::SBQueueItemGetKind(self.raw) }
    }

    /// The code address that will be executed when this work item
    /// is executed.
    ///
    /// Not all queue items will have an address associated with them.
    /// `QueueItemKind::Function` and `QueueItemKind::Block` work items
    /// should have an address.
    pub fn address(&self) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBQueueItemGetAddress(self.raw) })
    }

    /// Get an extended backtrace thread for this queue item, if available
    ///
    /// If the backtrace/thread information was collected when this item
    /// was enqueued, this call will provide it.
    ///
    /// The `thread_type` will typically be one of `"libdispatch"` or
    /// `"pthread"`.
    pub fn extended_backtrace_thread(&self, thread_type: &str) -> Option<SBThread> {
        let thread_type = CString::new(thread_type).unwrap();
        SBThread::maybe_wrap(unsafe {
            sys::SBQueueItemGetExtendedBacktraceThread(self.raw, thread_type.as_ptr())
        })
    }
}

impl Clone for SBQueueItem {
    fn clone(&self) -> SBQueueItem {
        SBQueueItem {
            raw: unsafe { sys::CloneSBQueueItem(self.raw) },
        }
    }
}

impl Drop for SBQueueItem {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBQueueItem(self.raw) };
    }
}

impl From<sys::SBQueueItemRef> for SBQueueItem {
    fn from(raw: sys::SBQueueItemRef) -> SBQueueItem {
        SBQueueItem { raw }
    }
}

unsafe impl Send for SBQueueItem {}
unsafe impl Sync for SBQueueItem {}

#[cfg(feature = "graphql")]
graphql_object!(SBQueueItem: crate::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }
});
