// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use super::broadcaster::SBBroadcaster;
use super::error::SBError;
use super::stream::SBStream;
use super::thread::SBThread;
use super::{lldb_pid_t, lldb_tid_t, StateType};
use sys;

/// The process associated with the target program.
///
/// You get a process by attaching to or launching a target program.
/// See [`SBTarget`] for details.
///
/// # Process State
///
/// The OS process ID (`pid_t`) for the process is available via
/// [`process_id`].
///
/// The [process state] can be obtained via [`state`]. It is common to
/// just check to see if the process [`is_alive`], [`is_running`] or
/// [`is_stopped`].
///
/// Once the process is in the `Exited` state, the [`exit_status`] and
/// [`exit_description`] are available for inspection.
///
/// # Execution Control
///
/// Once you have a process, you can:
///
/// * [`continue_execution`]
/// * [`stop`]
/// * [`kill`]
/// * [`detach`]
///
/// # Threads
///
/// The process contains the [threads of execution] for the [target]. The
/// available threads can be iterated over with [`threads`]:
///
/// ```no_run
/// # use lldb::{SBProcess, SBThread};
/// # fn look_at_threads(process: SBProcess) {
/// // Iterate over the threads...
/// for thread in process.threads() {
///     println!("Hello {}!", thread.thread_id());
/// }
/// // Or collect them into a vector!
/// let threads = process.threads().collect::<Vec<SBThread>>();
/// # }
/// ```
///
/// Specific individual threads can be looked up via [`thread_by_id`]
/// and [`thread_by_index_id`] methods.
///
/// Some functions operate on the 'currently selected thread'. This can
/// retrieved via [`selected_thread`] and set via [`set_selected_thread`],
/// [`set_selected_thread_by_id`], or [`set_selected_thread_by_index_id`].
///
/// # Events
///
/// ... to be written ...
///
/// [`SBTarget`]: struct.SBTarget.html
/// [`process_id`]: #method.process_id
/// [process state]: enum.StateType.html
/// [`state`]: #method.state
/// [`is_alive`]: #method.is_alive
/// [`is_running`]: #method.is_running
/// [`is_stopped`]: #method.is_stopped
/// [`exit_status`]: #method.exit_status
/// [`exit_description`]: #method.exit_description
/// [`continue_execution`]: #method.continue_execution
/// [`stop`]: #method.stop
/// [`kill`]: #method.kill
/// [`detach`]: #method.detach
/// [threads of execution]: struct.SBThread.html
/// [target]: struct.SBTarget.html
/// [`threads`]: #method.threads
/// [`thread_by_id`]: #method.thread_by_id
/// [`thread_by_index_id`]: #method.thread_by_index_id
/// [`selected_thread`]: #method.selected_thread
/// [`set_selected_thread`]: #method.set_selected_thread
/// [`set_selected_thread_by_id`]: #method.set_selected_thread_by_id
/// [`set_selected_thread_by_index_id`]: #method.set_selected_thread_by_index_id
pub struct SBProcess {
    /// The underlying raw `SBProcessRef`.
    pub raw: sys::SBProcessRef,
}

impl SBProcess {
    /// Construct a new `SBProcess`.
    pub fn wrap(raw: sys::SBProcessRef) -> SBProcess {
        SBProcess { raw: raw }
    }

    /// Construct a new `Some(SBProcess)` or `None`.
    pub fn maybe_wrap(raw: sys::SBProcessRef) -> Option<SBProcess> {
        if unsafe { sys::SBProcessIsValid(raw) != 0 } {
            Some(SBProcess { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBProcess` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBProcessIsValid(self.raw) != 0 }
    }

    /// The current state of this process (running, stopped, exited, etc.).
    pub fn state(&self) -> StateType {
        unsafe { sys::SBProcessGetState(self.raw) }
    }

    /// Returns `true` if the process is currently alive.
    ///
    /// This corresponds to the process being in the `Attaching`,
    /// `Launching`, `Stopped`, `Running`, `Stepping`, `Crashed`
    /// or `Suspended` states.
    pub fn is_alive(&self) -> bool {
        match self.state() {
            StateType::Attaching | StateType::Launching | StateType::Stopped |
            StateType::Running | StateType::Stepping | StateType::Crashed |
            StateType::Suspended => true,
            _ => false,
        }
    }

    /// Returns `true` if the process is currently running.
    ///
    /// This corresponds to the process being in the `Running`
    /// or `Stepping` states.
    pub fn is_running(&self) -> bool {
        match self.state() {
            StateType::Running | StateType::Stepping => true,
            _ => false,
        }
    }

    /// Returns `true` if the process is currently stopped.
    ///
    /// This corresponds to the process being in the `Stopped`, `Crashed`,
    /// or `Suspended` states.
    pub fn is_stopped(&self) -> bool {
        match self.state() {
            StateType::Stopped | StateType::Crashed | StateType::Suspended => true,
            _ => false,
        }
    }

    /// The exit status of the process when the process state is
    /// `Exited`.
    pub fn exit_status(&self) -> i32 {
        unsafe { sys::SBProcessGetExitStatus(self.raw) }
    }

    /// The exit description of the process when the process state
    /// is `Exited`.
    pub fn exit_description(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBProcessGetExitDescription(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Returns the process ID of the process.
    pub fn process_id(&self) -> lldb_pid_t {
        unsafe { sys::SBProcessGetProcessID(self.raw) }
    }

    /// Returns an integer ID that is guaranteed to be unique across all
    /// process instances. This is not the process ID, just a unique
    /// integer for comparison and caching purposes.
    pub fn unique_id(&self) -> u32 {
        unsafe { sys::SBProcessGetUniqueID(self.raw) }
    }

    /// Get the size, in bytes, of an address.
    pub fn address_byte_size(&self) -> u32 {
        unsafe { sys::SBProcessGetAddressByteSize(self.raw) }
    }

    /// Kills the process and shuts down all threads that were spawned to
    /// track and monitor the process.
    pub fn destroy(&self) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBProcessDestroy(self.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn continue_execution(&self) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBProcessContinue(self.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn stop(&self) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBProcessStop(self.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Same as calling `destroy`.
    pub fn kill(&self) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBProcessKill(self.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn detach(&self) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBProcessDetach(self.raw) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Send the process a Unix signal.
    pub fn signal(&self, signal: i32) -> Result<(), SBError> {
        let error = SBError::wrap(unsafe { sys::SBProcessSignal(self.raw, signal) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn broadcaster(&self) -> SBBroadcaster {
        SBBroadcaster::wrap(unsafe { sys::SBProcessGetBroadcaster(self.raw) })
    }

    /// Get an iterator over the [threads] known to this process instance.
    ///
    /// [threads]: struct.SBThread.html
    pub fn threads(&self) -> ProcessThreadIter {
        ProcessThreadIter {
            process: self,
            idx: 0,
        }
    }

    /// Returns the thread with the given thread ID.
    pub fn thread_by_id(&self, thread_id: lldb_tid_t) -> Option<SBThread> {
        SBThread::maybe_wrap(unsafe { sys::SBProcessGetThreadByID(self.raw, thread_id) })
    }

    /// Returns the thread with the given thread index ID.
    pub fn thread_by_index_id(&self, thread_index_id: u32) -> Option<SBThread> {
        SBThread::maybe_wrap(unsafe { sys::SBProcessGetThreadByIndexID(self.raw, thread_index_id) })
    }

    /// Returns the currently selected thread.
    pub fn selected_thread(&self) -> SBThread {
        SBThread::wrap(unsafe { sys::SBProcessGetSelectedThread(self.raw) })
    }

    /// Set the selected thread.
    pub fn set_selected_thread(&self, thread: &SBThread) -> bool {
        unsafe { sys::SBProcessSetSelectedThread(self.raw, thread.raw) != 0 }
    }

    /// Set the selected thread by ID.
    pub fn set_selected_thread_by_id(&self, thread_id: lldb_tid_t) -> bool {
        unsafe { sys::SBProcessSetSelectedThreadByID(self.raw, thread_id) != 0 }
    }

    /// Set the selected thread by index ID.
    pub fn set_selected_thread_by_index_id(&self, thread_index_id: u32) -> bool {
        unsafe { sys::SBProcessSetSelectedThreadByIndexID(self.raw, thread_index_id) != 0 }
    }
}

#[doc(hidden)]
pub struct ProcessThreadIter<'d> {
    process: &'d SBProcess,
    idx: usize,
}

impl<'d> Iterator for ProcessThreadIter<'d> {
    type Item = SBThread;

    fn next(&mut self) -> Option<SBThread> {
        if self.idx < unsafe { sys::SBProcessGetNumThreads(self.process.raw) as usize } {
            let r = Some(SBThread::wrap(unsafe {
                sys::SBProcessGetThreadAtIndex(self.process.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

impl fmt::Debug for SBProcess {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBProcessGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBProcess {{ {} }}", stream.data())
    }
}

impl Drop for SBProcess {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBProcess(self.raw) };
    }
}
