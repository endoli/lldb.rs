// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use super::{lldb_pid_t, StateType};
use sys;

/// The process associated with the target program.
#[derive(Debug)]
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
    pub fn is_alive(&self) -> bool {
        match self.state() {
            StateType::Attaching | StateType::Launching | StateType::Stopped |
            StateType::Running | StateType::Stepping | StateType::Crashed |
            StateType::Suspended => true,
            _ => false,
        }
    }

    /// Returns `true` if the process is currently running.
    pub fn is_running(&self) -> bool {
        match self.state() {
            StateType::Running | StateType::Stepping => true,
            _ => false,
        }
    }

    /// Returns `true` if the process is currently stopped.
    pub fn is_stopped(&self) -> bool {
        match self.state() {
            StateType::Stopped | StateType::Crashed | StateType::Suspended => true,
            _ => false,
        }
    }

    /// The exit status of the process when the process state is
    /// `eStateExited`.
    pub fn exit_status(&self) -> i32 {
        unsafe { sys::SBProcessGetExitStatus(self.raw) }
    }

    /// The exit description of the process when the process state
    /// is `eStateExited`.
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
}

impl Drop for SBProcess {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBProcess(self.raw) };
    }
}
