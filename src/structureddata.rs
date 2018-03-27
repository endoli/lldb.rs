// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CString;
use std::fmt;
use std::ptr;
use super::error::SBError;
use super::stream::SBStream;
use sys;

/// The value of a variable, register or expression.
pub struct SBStructuredData {
    /// The underlying raw `SBStructuredDataRef`.
    pub raw: sys::SBStructuredDataRef,
}

impl SBStructuredData {
    /// Construct a new `SBStructuredData`.
    pub fn wrap(raw: sys::SBStructuredDataRef) -> SBStructuredData {
        SBStructuredData { raw }
    }

    /// Construct a new `Some(SBStructuredData)` or `None`.
    pub fn maybe_wrap(raw: sys::SBStructuredDataRef) -> Option<SBStructuredData> {
        if unsafe { sys::SBStructuredDataIsValid(raw) != 0 } {
            Some(SBStructuredData { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBStructuredData` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBStructuredDataIsValid(self.raw) != 0 }
    }

    #[allow(missing_docs)]
    pub fn clear(&self) {
        unsafe { sys::SBStructuredDataClear(self.raw) };
    }

    #[allow(missing_docs)]
    pub fn set_from_json(&self, stream: &SBStream) -> Result<(), SBError> {
        let e = SBError::wrap(unsafe { sys::SBStructuredDataSetFromJSON(self.raw, stream.raw) });
        if e.is_success() {
            Ok(())
        } else {
            Err(e)
        }
    }

    #[allow(missing_docs)]
    pub fn get_as_json(&self) -> Result<SBStream, SBError> {
        let stream = SBStream::new();
        let e = SBError::wrap(unsafe { sys::SBStructuredDataGetAsJSON(self.raw, stream.raw) });
        if e.is_success() {
            Ok(stream)
        } else {
            Err(e)
        }
    }

    /// Return the type of data in this data structure.
    pub fn data_type(&self) -> sys::StructuredDataType {
        unsafe { sys::SBStructuredDataGetType(self.raw) }
    }

    /// Return the size (number of elements) in this data structure
    /// if it is an array or dictionary type. For other types,
    /// `0` will be returned.
    pub fn size(&self) -> usize {
        unsafe { sys::SBStructuredDataGetSize(self.raw) }
    }

    /// Return the value corresponding to a key if this data structure
    /// is a dictionary type.
    pub fn value_for_key(&self, key: &str) -> Option<SBStructuredData> {
        let key = CString::new(key).unwrap();
        SBStructuredData::maybe_wrap(unsafe {
            sys::SBStructuredDataGetValueForKey(self.raw, key.as_ptr())
        })
    }

    /// Return the value corresponding to an index if this data structure
    /// is array.
    pub fn item_at_index(&self, idx: usize) -> Option<SBStructuredData> {
        SBStructuredData::maybe_wrap(unsafe { sys::SBStructuredDataGetItemAtIndex(self.raw, idx) })
    }

    /// Return the integer value if this data structure is an integer type.
    pub fn integer_value(&self) -> Option<u64> {
        if self.data_type() == sys::StructuredDataType::Integer {
            Some(unsafe { sys::SBStructuredDataGetIntegerValue(self.raw, 0) })
        } else {
            None
        }
    }

    /// Return the floating point value if this data structure is a floating
    /// type.
    pub fn float_value(&self) -> Option<f64> {
        if self.data_type() == sys::StructuredDataType::Float {
            Some(unsafe { sys::SBStructuredDataGetFloatValue(self.raw, 0.0) })
        } else {
            None
        }
    }

    /// Return the boolean value if this data structure is a boolean type.
    pub fn boolean_value(&self) -> Option<bool> {
        if self.data_type() == sys::StructuredDataType::Boolean {
            Some(unsafe { sys::SBStructuredDataGetBooleanValue(self.raw, false as u8) != 0 })
        } else {
            None
        }
    }

    /// Provides the string value if this data structure is a string type.
    pub fn string_value(&self) -> Option<String> {
        if self.data_type() == sys::StructuredDataType::String {
            unsafe {
                let sz = sys::SBStructuredDataGetStringValue(self.raw, ptr::null_mut(), 0);
                let mut buf: Vec<u8> = Vec::with_capacity(sz);
                sys::SBStructuredDataGetStringValue(self.raw, buf.as_mut_ptr() as *mut i8, sz);
                buf.set_len(sz);
                String::from_utf8(buf).ok()
            }
        } else {
            None
        }
    }
}

impl fmt::Debug for SBStructuredData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBStructuredDataGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBStructuredData {{ {} }}", stream.data())
    }
}

impl Drop for SBStructuredData {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBStructuredData(self.raw) };
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBStructuredData: super::debugger::SBDebugger | &self | {
    field is_valid() -> bool {
        self.is_valid()
    }
});
