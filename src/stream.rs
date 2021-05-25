// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use sys;

/// A destination for streaming data output. By default, this is
/// a string stream, but it can be redirected to a file.
#[derive(Debug)]
pub struct SBStream {
    /// The underlying raw `SBStreamRef`.
    pub raw: sys::SBStreamRef,
}

impl SBStream {
    /// Construct a new `SBStream`.
    pub fn new() -> SBStream {
        SBStream::wrap(unsafe { sys::CreateSBStream() })
    }

    /// Construct a new `SBStream`.
    pub fn wrap(raw: sys::SBStreamRef) -> SBStream {
        SBStream { raw }
    }

    /// Construct a new `Some(SBStream)` or `None`.
    pub fn maybe_wrap(raw: sys::SBStreamRef) -> Option<SBStream> {
        if unsafe { sys::SBStreamIsValid(raw) } {
            Some(SBStream { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBStream` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBStreamIsValid(self.raw) }
    }

    /// If the stream is directed to a file, forget about the file and
    /// if the ownership of the file was transferred to this object,
    /// close the file. If the stream is backed by a local cache, clear
    /// this cache.
    pub fn clear(&self) {
        unsafe { sys::SBStreamClear(self.raw) }
    }

    /// If this stream is not redirected to a file, this retrieves the
    /// locally cached data.
    pub fn data(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBStreamGetData(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// If this stream is not redirected to a file, this retrieves the
    /// length of the locally cached data.
    pub fn len(&self) -> usize {
        unsafe { sys::SBStreamGetSize(self.raw) }
    }

    /// Is this stream empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for SBStream {
    fn default() -> SBStream {
        SBStream::new()
    }
}

impl Drop for SBStream {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBStream(self.raw) };
    }
}

unsafe impl Send for SBStream {}
unsafe impl Sync for SBStream {}
