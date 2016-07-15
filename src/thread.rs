// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use super::event::SBEvent;
use super::frame::SBFrame;
use super::process::SBProcess;
use super::stream::SBStream;
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
///
/// # Thread State
///
/// ...
///
/// # Execution Control
///
/// ...
///
/// # Frames
///
/// The thread contains [stack frames]. These can be iterated
/// over with [`frames`]:
///
/// ```no_run
/// # use lldb::{SBFrame, SBThread};
/// # fn look_at_frames(thread: SBThread) {
/// // Iterate over the frames...
/// for frame in thread.frames() {
///     println!("Hello {:?}!", frame);
/// }
/// // Or collect them into a vector!
/// let frames = thread.frames().collect::<Vec<SBFrame>>();
/// # }
/// ```
///
/// Some functions operate on the 'currently selected frame'. This can
/// retrieved via [`selected_frame`] and set via [`set_selected_frame`].
///
///
/// # Events
///
/// ...
///
/// [stack frames]: struct.SBFrame.html
/// [`frames`]: #method.frames
/// [`selected_frame`]: #method.selected_frame
/// [`set_selected_frame`]: #method.set_selected_frame
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

    #[allow(missing_docs)]
    pub fn broadcaster_class_name() -> &'static str {
        unsafe {
            match CStr::from_ptr(sys::SBThreadGetBroadcasterClassName()).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
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

    /// Get an iterator over the [frames] known to this thread instance.
    ///
    /// [frames]: struct.SBFrame.html
    pub fn frames(&self) -> SBThreadFrameIter {
        SBThreadFrameIter {
            thread: self,
            idx: 0,
        }
    }

    /// Get the currently selected frame for this thread.
    pub fn selected_frame(&self) -> SBFrame {
        SBFrame::wrap(unsafe { sys::SBThreadGetSelectedFrame(self.raw) })
    }

    /// Set the currently selected frame for this thread. This takes a frame index.
    pub fn set_selected_frame(&self, frame_index: u32) -> Option<SBFrame> {
        SBFrame::maybe_wrap(unsafe { sys::SBThreadSetSelectedFrame(self.raw, frame_index) })
    }

    /// Get the process in which this thread is running.
    pub fn process(&self) -> SBProcess {
        SBProcess::wrap(unsafe { sys::SBThreadGetProcess(self.raw) })
    }


    #[allow(missing_docs)]
    pub fn event_as_thread_event(event: &SBEvent) -> Option<SBThreadEvent> {
        if unsafe { sys::SBThreadEventIsThreadEvent(event.raw) != 0 } {
            Some(SBThreadEvent::new(event))
        } else {
            None
        }
    }
}

/// Iterate over the [frames] in a [thread].
///
/// [frames]: struct.SBFrame.html
/// [thread]: struct.SBThread.html
pub struct SBThreadFrameIter<'d> {
    thread: &'d SBThread,
    idx: usize,
}

impl<'d> Iterator for SBThreadFrameIter<'d> {
    type Item = SBFrame;

    fn next(&mut self) -> Option<SBFrame> {
        if self.idx < unsafe { sys::SBThreadGetNumFrames(self.thread.raw) as usize } {
            let r = Some(SBFrame::wrap(unsafe {
                sys::SBThreadGetFrameAtIndex(self.thread.raw, self.idx as u32)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

impl fmt::Debug for SBThread {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBThreadGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBThread {{ {} }}", stream.data())
    }
}

impl Drop for SBThread {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBThread(self.raw) };
    }
}

#[allow(missing_docs)]
pub struct SBThreadEvent<'e> {
    event: &'e SBEvent,
}

#[allow(missing_docs)]
impl<'e> SBThreadEvent<'e> {
    pub fn new(event: &'e SBEvent) -> Self {
        SBThreadEvent { event: event }
    }

    pub fn thread(&self) -> SBThread {
        SBThread::wrap(unsafe { sys::SBThreadGetThreadFromEvent(self.event.raw) })
    }

    pub fn frame(&self) -> Option<SBFrame> {
        SBFrame::maybe_wrap(unsafe { sys::SBThreadGetStackFrameFromEvent(self.event.raw) })
    }
}
