// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use super::process::SBProcess;
use super::queueitem::SBQueueItem;
use super::thread::SBThread;
use sys;

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
/// [`process`]: struct.SBProcess.html
/// [`SBThread::queue()`]: struct.SBThread.html#method.queue
pub struct SBQueue {
    /// The underlying raw `SBQueueRef`.
    pub raw: sys::SBQueueRef,
}

impl SBQueue {
    /// Construct a new `SBQueue`.
    pub fn wrap(raw: sys::SBQueueRef) -> SBQueue {
        SBQueue { raw }
    }

    /// Construct a new `Some(SBQueue)` or `None`.
    pub fn maybe_wrap(raw: sys::SBQueueRef) -> Option<SBQueue> {
        if unsafe { sys::SBQueueIsValid(raw) != 0 } {
            Some(SBQueue { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBQueue` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBQueueIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn process(&self) -> SBProcess {
        SBProcess::wrap(unsafe { sys::SBQueueGetProcess(self.raw) })
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
    /// [threads]: struct.SBThread.html
    pub fn threads(&self) -> SBQueueThreadIter {
        SBQueueThreadIter {
            queue: self,
            idx: 0,
        }
    }

    /// Get an iterator over the [pending items] known to this queue.
    ///
    /// [pending items]: struct.SBQueueItem.html
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

impl Drop for SBQueue {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBQueue(self.raw) };
    }
}

/// Iterate over the [threads] associated with a [queue].
///
/// [threads]: struct.SBThread.html
/// [queue]: struct.SBQueue.html
pub struct SBQueueThreadIter<'d> {
    queue: &'d SBQueue,
    idx: usize,
}

impl<'d> Iterator for SBQueueThreadIter<'d> {
    type Item = SBThread;

    fn next(&mut self) -> Option<SBThread> {
        if self.idx < unsafe { sys::SBQueueGetNumThreads(self.queue.raw) as usize } {
            let r = Some(SBThread::wrap(unsafe {
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
/// [queue items]: struct.SBQueueItem.html
/// [queue]: struct.SBQueue.html
pub struct SBQueueQueueItemIter<'d> {
    queue: &'d SBQueue,
    idx: usize,
}

impl<'d> Iterator for SBQueueQueueItemIter<'d> {
    type Item = SBQueueItem;

    fn next(&mut self) -> Option<SBQueueItem> {
        if self.idx < unsafe { sys::SBQueueGetNumPendingItems(self.queue.raw) as usize } {
            let r = Some(SBQueueItem::wrap(unsafe {
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
graphql_object!(SBQueue: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    // TODO(bm) This should be u64
    field queue_id() -> i64 {
        self.queue_id() as i64
    }

    field name() -> &str {
        self.name()
    }

    field threads() -> Vec<SBThread> {
        self.threads().collect()
    }

    field pending_items() -> Vec<SBQueueItem> {
        self.pending_items().collect()
    }

    // TODO(bm) This should be u32
    field num_running_items() -> i64 {
        self.num_running_items() as i64
    }
});
