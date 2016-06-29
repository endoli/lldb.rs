// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::address::SBAddress;
use super::filespec::SBFileSpec;
use sys;

/// Specifies an association with a contiguous range of
/// instructions and a source file location.
#[derive(Debug)]
pub struct SBLineEntry {
    /// The underlying raw `SBLineEntryRef`.
    pub raw: sys::SBLineEntryRef,
}

impl SBLineEntry {
    /// Construct a new `SBLineEntry`.
    pub fn new(raw: sys::SBLineEntryRef) -> SBLineEntry {
        SBLineEntry { raw: raw }
    }

    /// Construct a new `Some(SBLineEntry)` or `None`.
    pub fn maybe(raw: sys::SBLineEntryRef) -> Option<SBLineEntry> {
        if unsafe { sys::SBLineEntryIsValid(raw) != 0 } {
            Some(SBLineEntry { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBLineEntry` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBLineEntryIsValid(self.raw) != 0 }
    }

    /// The start address for this line entry.
    pub fn start_address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBLineEntryGetStartAddress(self.raw) } }
    }

    /// The end address for this line entry.
    pub fn end_address(&self) -> SBAddress {
        SBAddress { raw: unsafe { sys::SBLineEntryGetEndAddress(self.raw) } }
    }

    /// The file (`SBFileSpec`) for this line entry.
    pub fn filespec(&self) -> SBFileSpec {
        SBFileSpec { raw: unsafe { sys::SBLineEntryGetFileSpec(self.raw) } }
    }

    /// The 1-based line number for this line entry.
    ///
    /// A return value of `0` indicates that no line information is
    /// available.
    pub fn line(&self) -> u32 {
        unsafe { sys::SBLineEntryGetLine(self.raw) }
    }

    /// The 1-based column number for this line entry.
    ///
    /// A return value of `0` indicates that no column information is
    /// available.
    pub fn column(&self) -> u32 {
        unsafe { sys::SBLineEntryGetColumn(self.raw) }
    }
}
