// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::value::SBValue;
use super::{lldb_tid_t, StopReason};
use sys;

/// A thread of execution.
///
/// `SBThread`s can be referred to by their ID, which maps to the system
/// specific thread identifier, or by `IndexID`.  The ID may or may not
/// be unique depending on whether the system reuses its thread identifiers.
/// The `IndexID` is a monotonically increasing identifier that will always
/// uniquely reference a particular thread, and when that thread goes
/// away it will not be reused.
#[derive(Debug)]
pub struct SBThread {
    /// The underlying raw `SBThreadRef`.
    pub raw: sys::SBThreadRef,
}

impl SBThread {
    /// Construct a new `SBThread`.
    pub fn wrap(raw: sys::SBThreadRef) -> SBThread {
        SBThread { raw: raw }
    }

    /// Construct a new `Some(SBThread)` or `None`.
    pub fn maybe_wrap(raw: sys::SBThreadRef) -> Option<SBThread> {
        if unsafe { sys::SBThreadIsValid(raw) != 0 } {
            Some(SBThread { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBThread` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBThreadIsValid(self.raw) != 0 }
    }

    /// Get the stop reason for this thread.
    pub fn stop_reason(&self) -> StopReason {
        unsafe { sys::SBThreadGetStopReason(self.raw) }
    }

    /// The return value from the last stop if we just stopped due
    /// to stepping out of a function
    pub fn stop_return_value(&self) -> Option<SBValue> {
        SBValue::maybe_wrap(unsafe { sys::SBThreadGetStopReturnValue(self.raw) })
    }

    /// Returns a unique thread identifier for the current `SBThread`
    /// that will remain constant throughout the thread's lifetime in
    /// this process and will not be reused by another thread during this
    /// process lifetime.  On Mac OS X systems, this is a system-wide
    /// unique thread identifier; this identifier is also used by
    /// other tools like sample which helps to associate data from
    /// those tools with lldb.  See related `SBThread::index_id`.
    pub fn thread_id(&self) -> lldb_tid_t {
        unsafe { sys::SBThreadGetThreadID(self.raw) }
    }

    /// Return the index number for this `SBThread`.  The index
    /// number is the same thing that a user gives as an argument
    /// to `thread select` in the command line lldb.
    ///
    /// These numbers start at `1` (for the first thread lldb sees
    /// in a debug session) and increments up throughout the process
    /// lifetime.  An index number will not be reused for a different
    /// thread later in a process - thread 1 will always be associated
    /// with the same thread.  See related `SBThread::thread_id`.
    pub fn index_id(&self) -> u32 {
        unsafe { sys::SBThreadGetIndexID(self.raw) }
    }
}

impl Drop for SBThread {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBThread(self.raw) };
    }
}
