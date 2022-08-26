// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBProcess, SBQueueItem, SBThread};
use std::ffi::CStr;

/// A `libdispatch` (aka Grand Central Dispatch) queue.
///
/// A program using `libdispatch` will create queues, put work items
/// (functions, blocks) on the queues.  The system will create /
/// reassign pthreads to execute the work items for the queues.  A
/// serial queue will be associated with a single thread (or possibly
/// no thread, if it is not doing any work).  A concurrent queue may
/// be associated with multiple threads.
///
/// The available queues within a process can be found discovered by
/// inspecting the [`process`]:
///
/// ```no_run
/// # use lldb::{SBProcess, SBQueue};
/// # fn look_at_queues(process: SBProcess) {
/// // Iterate over the queues...
/// for queue in process.queues() {
///     println!("Hello {}!", queue.queue_id());
/// }
/// # }
/// ```
///
/// If a queue is associated with a thread, it can be discovered
/// from the thread via [`SBThread::queue()`].
///
/// [`process`]: SBProcess
pub struct SBQueue {
    /// The underlying raw `SBQueueRef`.
    pub raw: sys::SBQueueRef,
}

impl SBQueue {
    /// Construct a new `Some(SBQueue)` or `None`.
    pub fn maybe_wrap(raw: sys::SBQueueRef) -> Option<SBQueue> {
        if unsafe { sys::SBQueueIsValid(raw) } {
            Some(SBQueue { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBQueue` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBQueueIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn process(&self) -> SBProcess {
        SBProcess::from(unsafe { sys::SBQueueGetProcess(self.raw) })
    }

    /// Returns a unique identifying number for this queue that will not
    /// be used by any other queue during this process' execution.
    ///
    /// These ID numbers often start at 1 with the first system-created
    /// queues and increment from there.
    pub fn queue_id(&self) -> u64 {
        unsafe { sys::SBQueueGetQueueID(self.raw) }
    }

    /// The name of this queue.
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBQueueGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Get an iterator over the [threads] associated with this queue.
    ///
    /// [threads]: SBThread
    pub fn threads(&self) -> SBQueueThreadIter {
        SBQueueThreadIter {
            queue: self,
            idx: 0,
        }
    }

    /// Get an iterator over the [pending items] known to this queue.
    ///
    /// [pending items]: SBQueueItem
    pub fn pending_items(&self) -> SBQueueQueueItemIter {
        SBQueueQueueItemIter {
            queue: self,
            idx: 0,
        }
    }

    /// The number of work items that this queue is currently running.
    ///
    /// For a serial queue, this will be `0` or `1`.  For a concurrent
    /// queue, this may be any number.
    pub fn num_running_items(&self) -> u32 {
        unsafe { sys::SBQueueGetNumRunningItems(self.raw) }
    }

    /// The kind of this queue, serial or concurrent.
    pub fn kind(&self) -> sys::QueueKind {
        unsafe { sys::SBQueueGetKind(self.raw) }
    }
}

impl Clone for SBQueue {
    fn clone(&self) -> SBQueue {
        SBQueue {
            raw: unsafe { sys::CloneSBQueue(self.raw) },
        }
    }
}

impl Drop for SBQueue {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBQueue(self.raw) };
    }
}

impl From<sys::SBQueueRef> for SBQueue {
    fn from(raw: sys::SBQueueRef) -> SBQueue {
        SBQueue { raw }
    }
}

unsafe impl Send for SBQueue {}
unsafe impl Sync for SBQueue {}

/// Iterate over the [threads] associated with a [queue].
///
/// [threads]: SBThread
/// [queue]: SBQueue
pub struct SBQueueThreadIter<'d> {
    queue: &'d SBQueue,
    idx: usize,
}

impl<'d> Iterator for SBQueueThreadIter<'d> {
    type Item = SBThread;

    fn next(&mut self) -> Option<SBThread> {
        if self.idx < unsafe { sys::SBQueueGetNumThreads(self.queue.raw) as usize } {
            let r = Some(SBThread::from(unsafe {
                sys::SBQueueGetThreadAtIndex(self.queue.raw, self.idx as u32)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBQueueGetNumThreads(self.queue.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBQueueThreadIter<'d> {}

/// Iterate over the [queue items] in a [queue].
///
/// [queue items]: SBQueueItem
/// [queue]: SBQueue
pub struct SBQueueQueueItemIter<'d> {
    queue: &'d SBQueue,
    idx: usize,
}

impl<'d> Iterator for SBQueueQueueItemIter<'d> {
    type Item = SBQueueItem;

    fn next(&mut self) -> Option<SBQueueItem> {
        if self.idx < unsafe { sys::SBQueueGetNumPendingItems(self.queue.raw) as usize } {
            let r = Some(SBQueueItem::from(unsafe {
                sys::SBQueueGetPendingItemAtIndex(self.queue.raw, self.idx as u32)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBQueueGetNumPendingItems(self.queue.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBQueueQueueItemIter<'d> {}

#[cfg(feature = "graphql")]
#[graphql_object]
impl SBQueue {
    fn is_valid() -> bool {
        self.is_valid()
    }

    // TODO(bm) This should be u64
    fn queue_id() -> i32 {
        self.queue_id() as i32
    }

    fn name() -> &str {
        self.name()
    }

    fn threads() -> Vec<SBThread> {
        self.threads().collect()
    }

    fn pending_items() -> Vec<SBQueueItem> {
        self.pending_items().collect()
    }

    // TODO(bm) This should be u32
    fn num_running_items() -> i32 {
        self.num_running_items() as i32
    }
}
