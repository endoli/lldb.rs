// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use super::address::SBAddress;
use super::data::SBData;
use super::error::SBError;
use super::frame::SBFrame;
use super::process::SBProcess;
use super::stream::SBStream;
use super::target::SBTarget;
use super::thread::SBThread;
use super::watchpoint::SBWatchpoint;
use super::{lldb_addr_t, lldb_user_id_t, Format};
use sys;

/// The value of a variable, register or expression.
pub struct SBValue {
    /// The underlying raw `SBValueRef`.
    pub raw: sys::SBValueRef,
}

impl SBValue {
    /// Construct a new `SBValue`.
    pub fn wrap(raw: sys::SBValueRef) -> SBValue {
        SBValue { raw: raw }
    }

    /// Construct a new `Some(SBValue)` or `None`.
    pub fn maybe_wrap(raw: sys::SBValueRef) -> Option<SBValue> {
        if unsafe { sys::SBValueIsValid(raw) != 0 } {
            Some(SBValue { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBValue` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBValueIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn clear(&self) {
        unsafe { sys::SBValueClear(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn error(&self) -> Option<SBError> {
        SBError::maybe_wrap(unsafe { sys::SBValueGetError(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn id(&self) -> lldb_user_id_t {
        unsafe { sys::SBValueGetID(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBValueGetName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn type_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBValueGetTypeName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn display_type_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBValueGetDisplayTypeName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn byte_size(&self) -> usize {
        unsafe { sys::SBValueGetByteSize(self.raw) as usize }
    }

    #[allow(missing_docs)]
    pub fn is_in_scope(&self) -> bool {
        unsafe { sys::SBValueIsInScope(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn format(&self) -> Format {
        unsafe { sys::SBValueGetFormat(self.raw) }
    }

    #[allow(missing_docs)]
    pub fn set_format(&self, format: Format) {
        unsafe { sys::SBValueSetFormat(self.raw, format) }
    }

    #[allow(missing_docs)]
    pub fn value(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBValueGetValue(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    #[allow(missing_docs)]
    pub fn dereference(&self) -> Option<SBValue> {
        SBValue::maybe_wrap(unsafe { sys::SBValueDereference(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn address_of(&self) -> Option<SBValue> {
        SBValue::maybe_wrap(unsafe { sys::SBValueAddressOf(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn type_is_pointer_type(&self) -> bool {
        unsafe { sys::SBValueTypeIsPointerType(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn target(&self) -> SBTarget {
        SBTarget::wrap(unsafe { sys::SBValueGetTarget(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn process(&self) -> SBProcess {
        SBProcess::wrap(unsafe { sys::SBValueGetProcess(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn thread(&self) -> SBThread {
        SBThread::wrap(unsafe { sys::SBValueGetThread(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn frame(&self) -> SBFrame {
        SBFrame::wrap(unsafe { sys::SBValueGetFrame(self.raw) })
    }

    /// Find and watch a variable.
    pub fn watch(&self,
                 resolve_location: bool,
                 read: bool,
                 write: bool)
                 -> Result<SBWatchpoint, SBError> {
        let error = SBError::new();
        let wp = unsafe {
            sys::SBValueWatch(self.raw,
                              resolve_location as u8,
                              read as u8,
                              write as u8,
                              error.raw)
        };
        if error.is_success() {
            Ok(SBWatchpoint::wrap(wp))
        } else {
            Err(error)
        }
    }

    /// Find and watch the location pointed to by a variable.
    pub fn watch_pointee(&self,
                         resolve_location: bool,
                         read: bool,
                         write: bool)
                         -> Result<SBWatchpoint, SBError> {
        let error = SBError::new();
        let wp = unsafe {
            sys::SBValueWatchPointee(self.raw,
                                     resolve_location as u8,
                                     read as u8,
                                     write as u8,
                                     error.raw)
        };
        if error.is_success() {
            Ok(SBWatchpoint::wrap(wp))
        } else {
            Err(error)
        }
    }

    /// Get an `SBData` wrapping what this `SBValue` points to.
    ///
    /// This method will dereference the current `SBValue`, if its
    /// data type is a `T*` or `T[]`, and extract `item_count` elements
    /// of type `T` from it, copying their contents into an `SBData`.
    ///
    /// `item_idx` is the index of the first item to retrieve. For an array
    /// this is equivalent to `array[item_idx]`, for a pointer
    /// to `*(pointer + item_idx)`. In either case, the measurement
    /// unit for `item_idx` is the `sizeof(T)` rather than bytes.
    ///
    /// `item_count` is how many items should be copied into the output.
    /// By default only one item is copied, but more can be asked for.
    ///
    /// Returns `Some(SBData)` with the contents of the copied items, on success.
    /// `None` otherwise.
    pub fn pointee_data(&self, item_idx: u32, item_count: u32) -> Option<SBData> {
        SBData::maybe_wrap(unsafe { sys::SBValueGetPointeeData(self.raw, item_idx, item_count) })
    }

    /// Get an `SBData` wrapping the contents of this `SBValue`.
    ///
    /// This method will read the contents of this object in memory
    /// and copy them into an `SBData` for future use.
    ///
    /// Returns `Some(SBData)` with the contents of this `SBValue`, on success.
    /// `None` otherwise.
    pub fn data(&self) -> Option<SBData> {
        SBData::maybe_wrap(unsafe { sys::SBValueGetData(self.raw) })
    }

    #[allow(missing_docs)]
    pub fn set_data(&self, data: &SBData) -> Result<(), SBError> {
        let error = SBError::new();
        if unsafe { sys::SBValueSetData(self.raw, data.raw, error.raw) != 0 } {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(missing_docs)]
    pub fn load_address(&self) -> Option<lldb_addr_t> {
        let load_address = unsafe { sys::SBValueGetLoadAddress(self.raw) };
        if load_address != u64::max_value() {
            Some(load_address)
        } else {
            None
        }
    }

    #[allow(missing_docs)]
    pub fn address(&self) -> Option<SBAddress> {
        SBAddress::maybe_wrap(unsafe { sys::SBValueGetAddress(self.raw) })
    }
}

impl fmt::Debug for SBValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBValueGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBValue {{ {} }}", stream.data())
    }
}

impl Drop for SBValue {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBValue(self.raw) };
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBValue: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }

    // TODO(bm): This should be u64
    field id() -> i64 {
        self.id() as i64
    }

    field name() -> &str {
        self.name()
    }

    field type_name() -> &str {
        self.type_name()
    }

    field display_type_name() -> &str {
        self.display_type_name()
    }

    // TODO(bm): This should be usize.
    field byte_size() -> i64 {
        self.byte_size() as i64
    }

    field is_in_scope() -> bool {
        self.is_in_scope()
    }
});
