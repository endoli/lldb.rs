// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    lldb_addr_t, lldb_pid_t, lldb_tid_t, sys, Permissions, SBBroadcaster, SBError, SBEvent,
    SBFileSpec, SBMemoryRegionInfo, SBMemoryRegionInfoList, SBProcessInfo, SBQueue, SBStream,
    SBStructuredData, SBTarget, SBThread, StateType,
};
use std::ffi::{CStr, CString};
use std::fmt;

/// The process associated with the target program.
///
/// You get a process by attaching to or launching a target program.
/// See [`SBTarget`] for details.
///
/// # Process State
///
/// The OS process ID (`pid_t`) for the process is available via
/// [`SBProcess::process_id()`].
///
/// The [process state] can be obtained via [`SBProcess::state()`]. It
/// is common to just check to see if the process [`SBProcess::is_alive()`],
/// [`SBProcess::is_running()`] or [`SBProcess::is_stopped()`].
///
/// Once the process is in the `Exited` state, the
/// [`SBProcess::exit_status()`] and
/// [`SBProcess::exit_description()`] are available for inspection.
///
/// # Execution Control
///
/// Once you have a process, you can:
///
/// * [`SBProcess::continue_execution()`]
/// * [`SBProcess::stop()`]
/// * [`SBProcess::kill()`]
/// * [`SBProcess::detach()`]
///
/// # Threads
///
/// The process contains the [threads of execution] for the [target]. The
/// available threads can be iterated over with [`SBProcess::threads()`]:
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
/// Specific individual threads can be looked up via
/// [`SBProcess::thread_by_id()`] and [`SBProcess::thread_by_index_id()`]
/// methods.
///
/// Some functions operate on the 'currently selected thread'. This can
/// retrieved via [`SBProcess::selected_thread()`] and set via
/// [`SBProcess::set_selected_thread()`],
/// [`SBProcess::set_selected_thread_by_id()`], or
/// [`SBProcess::set_selected_thread_by_index_id()`].
///
/// # Queues
///
/// A process may also have a set of queues associated with it. This is used
/// on macOS, iOS and other Apple operating systems to support debugger
/// integration with `libdispatch`, also known as GCD or "Grand Central
/// Dispatch".
///
/// The active queues can be iterated over with [`SBProcess::queues()`]:
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
/// # Events
///
/// ... to be written ...
///
/// [`SBTarget`]: crate::SBTarget
/// [process state]: StateType
/// [threads of execution]: SBThread
/// [target]: crate::SBTarget
pub struct SBProcess {
    /// The underlying raw `SBProcessRef`.
    pub raw: sys::SBProcessRef,
}

impl SBProcess {
    /// Construct a new `SBProcess`.
    pub(crate) fn wrap(raw: sys::SBProcessRef) -> SBProcess {
        SBProcess { raw }
    }

    /// Construct a new `Some(SBProcess)` or `None`.
    #[allow(dead_code)]
    pub(crate) fn maybe_wrap(raw: sys::SBProcessRef) -> Option<SBProcess> {
        if unsafe { sys::SBProcessIsValid(raw) } {
            Some(SBProcess { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBProcess` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBProcessIsValid(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn broadcaster_class_name() -> &'static str {
        unsafe {
            match CStr::from_ptr(sys::SBProcessGetBroadcasterClassName()).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The current state of this process (running, stopped, exited, etc.).
    ///
    /// See also:
    ///
    /// - [`SBProcess::is_alive()`]
    /// - [`SBProcess::is_running()`]
    /// - [`SBProcess::is_stopped()`]
    /// - [`StateType`]
    pub fn state(&self) -> StateType {
        unsafe { sys::SBProcessGetState(self.raw) }
    }

    /// Returns `true` if the process is currently alive.
    ///
    /// This corresponds to the process being in the `Attaching`,
    /// `Launching`, `Stopped`, `Running`, `Stepping`, `Crashed`
    /// or `Suspended` states.
    ///
    /// See also:
    ///
    /// - [`SBProcess::is_running()`]
    /// - [`SBProcess::is_stopped()`]
    /// - [`SBProcess::state()`]
    /// - [`StateType`]
    pub fn is_alive(&self) -> bool {
        matches!(
            self.state(),
            StateType::Attaching
                | StateType::Launching
                | StateType::Stopped
                | StateType::Running
                | StateType::Stepping
                | StateType::Crashed
                | StateType::Suspended
        )
    }

    /// Returns `true` if the process is currently running.
    ///
    /// This corresponds to the process being in the `Running`
    /// or `Stepping` states.
    ///
    /// See also:
    ///
    /// - [`SBProcess::is_alive()`]
    /// - [`SBProcess::is_stopped()`]
    /// - [`SBProcess::state()`]
    /// - [`StateType`]
    pub fn is_running(&self) -> bool {
        matches!(self.state(), StateType::Running | StateType::Stepping)
    }

    /// Returns `true` if the process is currently stopped.
    ///
    /// This corresponds to the process being in the `Stopped`, `Crashed`,
    /// or `Suspended` states.
    ///
    /// See also:
    ///
    /// - [`SBProcess::is_alive()`]
    /// - [`SBProcess::is_running()`]
    /// - [`SBProcess::state()`]
    /// - [`StateType`]
    pub fn is_stopped(&self) -> bool {
        matches!(
            self.state(),
            StateType::Stopped | StateType::Crashed | StateType::Suspended
        )
    }

    /// The exit status of the process when the process state is
    /// `Exited`.
    ///
    /// See also:
    ///
    /// - [`SBProcess::exit_description()`]
    /// - [`SBProcess::state()`]
    /// - [`StateType`]
    pub fn exit_status(&self) -> i32 {
        unsafe { sys::SBProcessGetExitStatus(self.raw) }
    }

    /// The exit description of the process when the process state
    /// is `Exited`.
    ///
    /// See also:
    ///
    /// - [`SBProcess::exit_status()`]
    /// - [`SBProcess::state()`]
    /// - [`StateType`]
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

    /// Reads data from the current process's stdout stream until the end of the stream.
    pub fn get_stdout_all(&self) -> Option<String> {
        let dst_len = 0x1000;
        let mut output = String::new();
        let mut dst: Vec<u8> = Vec::with_capacity(dst_len);
        loop {
            let out_len =
                unsafe { sys::SBProcessGetSTDOUT(self.raw, dst.as_mut_ptr() as *mut i8, dst_len) };
            if out_len == 0 {
                break;
            }
            unsafe { dst.set_len(out_len) };
            output += std::str::from_utf8(&dst).ok()?;
        }

        Some(output)
    }

    /// Reads data from the current process's stdout stream.
    pub fn get_stdout(&self) -> Option<String> {
        let dst_len = 0x1000;
        let mut dst: Vec<u8> = Vec::with_capacity(dst_len);

        let out_len =
            unsafe { sys::SBProcessGetSTDOUT(self.raw, dst.as_mut_ptr() as *mut i8, dst_len) };

        unsafe { dst.set_len(out_len) };
        String::from_utf8(dst).ok()
    }

    /// Reads data from the current process's stderr stream until the end of the stream.
    pub fn get_stderr_all(&self) -> Option<String> {
        let dst_len = 0x1000;
        let mut output = String::new();
        let mut dst: Vec<u8> = Vec::with_capacity(dst_len);
        loop {
            let out_len =
                unsafe { sys::SBProcessGetSTDERR(self.raw, dst.as_mut_ptr() as *mut i8, dst_len) };
            if out_len == 0 {
                break;
            }
            unsafe { dst.set_len(out_len) };
            output += std::str::from_utf8(&dst).ok()?;
        }

        Some(output)
    }

    /// Reads data from the current process's stderr stream.
    pub fn get_stderr(&self) -> Option<String> {
        let dst_len = 0x1000;
        let mut dst: Vec<u8> = Vec::with_capacity(dst_len);

        let out_len =
            unsafe { sys::SBProcessGetSTDERR(self.raw, dst.as_mut_ptr() as *mut i8, dst_len) };

        unsafe { dst.set_len(out_len) };
        String::from_utf8(dst).ok()
    }

    #[allow(missing_docs)]
    pub fn broadcaster(&self) -> SBBroadcaster {
        SBBroadcaster::wrap(unsafe { sys::SBProcessGetBroadcaster(self.raw) })
    }

    /// Returns the process' extended crash information.
    pub fn get_extended_crash_information(&self) -> SBStructuredData {
        SBStructuredData::wrap(unsafe { sys::SBProcessGetExtendedCrashInformation(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn get_num_supported_hardware_watchpoints(&self) -> Result<u32, SBError> {
        let error = SBError::default();
        let num = unsafe { sys::SBProcessGetNumSupportedHardwareWatchpoints(self.raw, error.raw) };
        if error.is_success() {
            Ok(num)
        } else {
            Err(error)
        }
    }

    /// Get an iterator over the [threads] known to this process instance.
    ///
    /// [threads]: SBThread
    pub fn threads(&self) -> SBProcessThreadIter {
        SBProcessThreadIter {
            process: self,
            idx: 0,
        }
    }

    /// Get an iterator over the [queues] known to this process instance.
    ///
    /// [queues]: SBQueue
    pub fn queues(&self) -> SBProcessQueueIter {
        SBProcessQueueIter {
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
        unsafe { sys::SBProcessSetSelectedThread(self.raw, thread.raw) }
    }

    /// Set the selected thread by ID.
    pub fn set_selected_thread_by_id(&self, thread_id: lldb_tid_t) -> bool {
        unsafe { sys::SBProcessSetSelectedThreadByID(self.raw, thread_id) }
    }

    /// Set the selected thread by index ID.
    pub fn set_selected_thread_by_index_id(&self, thread_index_id: u32) -> bool {
        unsafe { sys::SBProcessSetSelectedThreadByIndexID(self.raw, thread_index_id) }
    }

    #[allow(missing_docs)]
    pub fn event_as_process_event(event: &SBEvent) -> Option<SBProcessEvent> {
        if unsafe { sys::SBProcessEventIsProcessEvent(event.raw) } {
            Some(SBProcessEvent::new(event))
        } else {
            None
        }
    }

    /// Save the state of the process in a core file (or mini dump on Windows).
    pub fn save_core(&self, file_name: &str) -> Result<(), SBError> {
        let f = CString::new(file_name).unwrap();
        let error = SBError::wrap(unsafe { sys::SBProcessSaveCore(self.raw, f.as_ptr()) });
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn process_info(&self) -> SBProcessInfo {
        SBProcessInfo::wrap(unsafe { sys::SBProcessGetProcessInfo(self.raw) })
    }

    /// Allocate memory within the process.
    ///
    /// This function will allocate `size` bytes in the process's address space.
    ///
    /// The `permissions` must be any of the [`Permissions`] bits OR'd together.
    /// The permissions on a given memory allocation can't be changed
    /// after allocation.  Note that a block that isn't set writable
    /// can still be written from lldb, just not by the process
    /// itself.
    ///
    /// Returns the address of the allocated buffer in the process or
    /// the error that occurred while trying to allocate.
    ///
    /// The allocated memory can be deallocated with [`SBProcess::deallocate_memory()`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use lldb::{Permissions, SBProcess};
    /// # fn look_at_threads(process: SBProcess) {
    /// if let Ok(data_addr) = process.allocate_memory(1024, Permissions::READABLE) {
    ///     // Do something with the address.
    /// }
    /// if let Ok(code_addr) = process.allocate_memory(1024, Permissions::READABLE | Permissions::EXECUTABLE) {
    ///     // Do something with the address.
    /// }
    /// # }
    pub fn allocate_memory(
        &self,
        size: usize,
        permissions: Permissions,
    ) -> Result<lldb_addr_t, SBError> {
        let error = SBError::default();
        let addr =
            unsafe { sys::SBProcessAllocateMemory(self.raw, size, permissions.bits(), error.raw) };
        if error.is_success() {
            Ok(addr)
        } else {
            Err(error)
        }
    }

    /// Deallocate memory in the process.
    ///
    /// This function will deallocate memory in the process's address
    /// space that was allocated with [`SBProcess::allocate_memory()`].
    ///
    /// If an error occurs while deallocating, it will be returned.
    ///
    /// # Safety
    ///
    /// The `ptr` must be a return value from [`SBProcess::allocate_memory()`],
    /// pointing to the memory you want to deallocate.
    pub unsafe fn deallocate_memory(&self, ptr: lldb_addr_t) -> Result<(), SBError> {
        let error = SBError::wrap(sys::SBProcessDeallocateMemory(self.raw, ptr));
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Query the address `load_addr` and return the details of the
    /// [memory region] that contains it.
    ///
    /// See also:
    ///
    /// - [`SBProcess::get_memory_regions()`]
    ///
    /// [memory region]: SBMemoryRegionInfo
    pub fn get_memory_region_info(
        &self,
        load_addr: lldb_addr_t,
    ) -> Result<SBMemoryRegionInfo, SBError> {
        let region_info = SBMemoryRegionInfo::default();
        let error = SBError::wrap(unsafe {
            sys::SBProcessGetMemoryRegionInfo(self.raw, load_addr, region_info.raw)
        });

        if error.is_success() {
            Ok(region_info)
        } else {
            Err(error)
        }
    }

    /// Return the [list] of [memory regions] within the process.
    ///
    /// See also:
    ///
    /// - [`SBProcess::get_memory_region_info()`]
    ///
    /// [list]: SBMemoryRegionInfoList
    /// [memory regions]: SBMemoryRegionInfo
    pub fn get_memory_regions(&self) -> SBMemoryRegionInfoList {
        SBMemoryRegionInfoList::wrap(unsafe { sys::SBProcessGetMemoryRegions(self.raw) })
    }

    /// Reads the memory at specified address in the process to the `buffer`
    pub fn read_memory(&self, addr: lldb_addr_t, buffer: &mut [u8]) -> Result<(), SBError> {
        // SBProcessReadMemory will return an error if the memory region is not allowed to read
        // and does not cause bad behavior so this method can be safe.
        let error = SBError::default();
        unsafe {
            sys::SBProcessReadMemory(
                self.raw,
                addr,
                buffer.as_mut_ptr() as *mut _,
                buffer.len(),
                error.raw,
            );
        }
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Writes the `buffer` data to the memory at specified address in the process
    pub fn write_memory(&self, addr: lldb_addr_t, buffer: &[u8]) -> Result<(), SBError> {
        let error = SBError::default();
        unsafe {
            sys::SBProcessWriteMemory(
                self.raw,
                addr,
                buffer.as_ptr() as *mut _,
                buffer.len(),
                error.raw,
            );
        }
        if error.is_success() {
            Ok(())
        } else {
            Err(error)
        }
    }

    /// Returns the byte order of target process
    pub fn byte_order(&self) -> crate::ByteOrder {
        unsafe { sys::SBProcessGetByteOrder(self.raw) }
    }

    /// Loads the specified image into the process.
    pub fn load_image(&self, file: &SBFileSpec) -> Result<ImageToken, SBError> {
        let error = SBError::default();
        let image_token = unsafe { sys::SBProcessLoadImage(self.raw, file.raw, error.raw) };
        if error.is_failure() {
            Err(error)
        } else {
            Ok(ImageToken(image_token))
        }
    }

    /// Unloads the image loaded with [`load_image`].
    ///
    /// [`load_image`]: Self::load_image
    pub fn unload_image(&self, image_token: ImageToken) -> Result<(), SBError> {
        // the method returns error if image_token is not valid, instead of causing undefined behavior.
        let error = SBError::wrap(unsafe { sys::SBProcessUnloadImage(self.raw, image_token.0) });
        if error.is_failure() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Returns the [`SBTarget`] corresponding to this `SBProcess`.
    ///
    /// This never return `None` if `self` is [`valid`].
    ///
    /// [`SBTarget`]: SBTarget
    /// [`valid`]: Self::is_valid
    pub fn target(&self) -> Option<SBTarget> {
        SBTarget::maybe_wrap(unsafe { sys::SBProcessGetTarget(self.raw) })
    }
}

/// Iterate over the [threads] in a [process].
///
/// [threads]: SBThread
/// [process]: SBProcess
pub struct SBProcessThreadIter<'d> {
    process: &'d SBProcess,
    idx: usize,
}

impl Iterator for SBProcessThreadIter<'_> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBProcessGetNumThreads(self.process.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

/// Iterate over the [queues] in a [process].
///
/// [queues]: SBQueue
/// [process]: SBProcess
pub struct SBProcessQueueIter<'d> {
    process: &'d SBProcess,
    idx: usize,
}

impl Iterator for SBProcessQueueIter<'_> {
    type Item = SBQueue;

    fn next(&mut self) -> Option<SBQueue> {
        if self.idx < unsafe { sys::SBProcessGetNumQueues(self.process.raw) as usize } {
            let r = Some(SBQueue::wrap(unsafe {
                sys::SBProcessGetQueueAtIndex(self.process.raw, self.idx)
            }));
            self.idx += 1;
            r
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBProcessGetNumQueues(self.process.raw) } as usize;
        (sz - self.idx, Some(sz))
    }
}

/// The token to unload image
pub struct ImageToken(pub u32);

impl Clone for SBProcess {
    fn clone(&self) -> SBProcess {
        SBProcess {
            raw: unsafe { sys::CloneSBProcess(self.raw) },
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

unsafe impl Send for SBProcess {}
unsafe impl Sync for SBProcess {}

#[allow(missing_docs)]
pub struct SBProcessEvent<'e> {
    event: &'e SBEvent,
}

#[allow(missing_docs)]
impl<'e> SBProcessEvent<'e> {
    pub fn new(event: &'e SBEvent) -> Self {
        SBProcessEvent { event }
    }

    pub fn process_state(&self) -> StateType {
        unsafe { sys::SBProcessGetStateFromEvent(self.event.raw) }
    }

    pub fn process(&self) -> SBProcess {
        SBProcess::wrap(unsafe { sys::SBProcessGetProcessFromEvent(self.event.raw) })
    }

    pub fn interrupted(&self) -> bool {
        unsafe { sys::SBProcessGetInterruptedFromEvent(self.event.raw) }
    }

    pub fn restarted(&self) -> bool {
        unsafe { sys::SBProcessGetRestartedFromEvent(self.event.raw) }
    }

    pub fn restarted_reasons(&self) -> SBProcessEventRestartedReasonIter {
        SBProcessEventRestartedReasonIter {
            event: self,
            idx: 0,
        }
    }

    #[allow(missing_docs)]
    pub const BROADCAST_BIT_STATE_CHANGED: u32 = (1 << 0);
    #[allow(missing_docs)]
    pub const BROADCAST_BIT_INTERRUPT: u32 = (1 << 1);
    #[allow(missing_docs)]
    pub const BROADCAST_BIT_STDOUT: u32 = (1 << 2);
    #[allow(missing_docs)]
    pub const BROADCAST_BIT_STDERR: u32 = (1 << 3);
    #[allow(missing_docs)]
    pub const BROADCAST_BIT_PROFILE_DATA: u32 = (1 << 4);
    #[allow(missing_docs)]
    pub const BROADCAST_BIT_STRUCTURED_DATA: u32 = (1 << 5);
}

/// Iterate over the restart reasons in a [process event].
///
/// [process event]: SBProcessEvent
pub struct SBProcessEventRestartedReasonIter<'d> {
    event: &'d SBProcessEvent<'d>,
    idx: usize,
}

impl<'d> Iterator for SBProcessEventRestartedReasonIter<'d> {
    type Item = &'d str;

    fn next(&mut self) -> Option<&'d str> {
        let raw = self.event.event.raw;
        if self.idx < unsafe { sys::SBProcessGetNumRestartedReasonsFromEvent(raw) } {
            let r = unsafe {
                let s = CStr::from_ptr(sys::SBProcessGetRestartedReasonAtIndexFromEvent(
                    raw, self.idx,
                ));
                match s.to_str() {
                    Ok(s) => s,
                    _ => panic!("Invalid string?"),
                }
            };
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBProcessGetNumRestartedReasonsFromEvent(self.event.event.raw) };
        (sz - self.idx, Some(sz))
    }
}

impl ExactSizeIterator for SBProcessEventRestartedReasonIter<'_> {}

#[cfg(feature = "graphql")]
#[juniper::graphql_object]
impl SBProcess {
    fn is_alive() -> bool {
        self.is_alive()
    }

    fn is_running() -> bool {
        self.is_running()
    }

    fn is_stopped() -> bool {
        self.is_stopped()
    }

    fn exit_status() -> i32 {
        self.exit_status()
    }

    fn exit_description() -> &str {
        self.exit_description()
    }

    // TODO(bm): This should be u64
    fn process_id() -> i32 {
        self.process_id() as i32
    }

    // TODO(bm) This should be u32
    fn unique_id() -> i32 {
        self.unique_id() as i32
    }

    // TODO(bm) This should be u32
    fn address_byte_size() -> i32 {
        self.address_byte_size() as i32
    }

    fn threads() -> Vec<SBThread> {
        self.threads().collect()
    }

    fn queues() -> Vec<SBQueue> {
        self.queues().collect()
    }

    fn selected_thread() -> SBThread {
        self.selected_thread()
    }

    fn process_info() -> SBProcessInfo {
        self.process_info()
    }

    fn memory_regions() -> Vec<SBMemoryRegionInfo> {
        self.get_memory_regions().iter().collect()
    }
}
