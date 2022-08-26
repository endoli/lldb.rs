// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBAddress, SBFileSpec, SBStream};
use std::ffi::CStr;
use std::fmt;

/// A lexical block.
pub struct SBBlock {
    /// The underlying raw `SBBlockRef`.
    pub raw: sys::SBBlockRef,
}

impl SBBlock {
    /// Construct a new `SBBlock`.
    pub(crate) fn wrap(raw: sys::SBBlockRef) -> SBBlock {
        SBBlock { raw }
    }

    /// Construct a new `Some(SBBlock)` or `None`.
    pub(crate) fn maybe_wrap(raw: sys::SBBlockRef) -> Option<SBBlock> {
        if unsafe { sys::SBBlockIsValid(raw) } {
            Some(SBBlock { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBBlock` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBBlockIsValid(self.raw) }
    }

    /// Does this block represent an inlined function?
    pub fn is_inlined(&self) -> bool {
        unsafe { sys::SBBlockIsInlined(self.raw) }
    }

    /// Get the function name if this block represents an inlined function.
    pub fn inlined_name(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBBlockGetInlinedName(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// Get the call site file if this block represents an inlined function.
    pub fn inlined_call_site_file(&self) -> Option<SBFileSpec> {
        SBFileSpec::maybe_wrap(unsafe { sys::SBBlockGetInlinedCallSiteFile(self.raw) })
    }

    /// Get the call site line number if this block represents an inlined function.
    pub fn inlined_call_site_line(&self) -> Option<u32> {
        let line = unsafe { sys::SBBlockGetInlinedCallSiteLine(self.raw) };
        if line > 0 {
            Some(line)
        } else {
            None
        }
    }

    /// Get the call site column number if this block represents an inlined function.
    pub fn inlined_call_site_column(&self) -> Option<u32> {
        let column = unsafe { sys::SBBlockGetInlinedCallSiteColumn(self.raw) };
        if column > 0 {
            Some(column)
        } else {
            None
        }
    }

    /// Get the parent block
    pub fn parent(&self) -> Option<SBBlock> {
        SBBlock::maybe_wrap(unsafe { sys::SBBlockGetParent(self.raw) })
    }

    /// Get the inlined block that is or contains this block.
    pub fn containing_inlined_block(&self) -> Option<SBBlock> {
        SBBlock::maybe_wrap(unsafe { sys::SBBlockGetContainingInlinedBlock(self.raw) })
    }

    /// Get the sibling block for this block.
    pub fn sibling(&self) -> Option<SBBlock> {
        SBBlock::maybe_wrap(unsafe { sys::SBBlockGetSibling(self.raw) })
    }

    /// Get the first child block for this block.
    pub fn first_child(&self) -> Option<SBBlock> {
        SBBlock::maybe_wrap(unsafe { sys::SBBlockGetFirstChild(self.raw) })
    }

    /// The number of address ranges associated with this block.
    pub fn num_ranges(&self) -> u32 {
        unsafe { sys::SBBlockGetNumRanges(self.raw) }
    }

    /// Get the start address of an address range.
    pub fn range_start_address(&self, idx: u32) -> SBAddress {
        SBAddress {
            raw: unsafe { sys::SBBlockGetRangeStartAddress(self.raw, idx) },
        }
    }

    /// Get the end address of an address range.
    pub fn range_end_address(&self, idx: u32) -> SBAddress {
        SBAddress {
            raw: unsafe { sys::SBBlockGetRangeEndAddress(self.raw, idx) },
        }
    }

    /// Given an address, find out which address range it is part of.
    pub fn range_index_for_block_address(&self, block_address: &SBAddress) -> u32 {
        unsafe { sys::SBBlockGetRangeIndexForBlockAddress(self.raw, block_address.raw) }
    }
}

impl Clone for SBBlock {
    fn clone(&self) -> SBBlock {
        SBBlock {
            raw: unsafe { sys::CloneSBBlock(self.raw) },
        }
    }
}

impl fmt::Debug for SBBlock {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBBlockGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBBlock {{ {} }}", stream.data())
    }
}

impl Drop for SBBlock {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBBlock(self.raw) };
    }
}

unsafe impl Send for SBBlock {}
unsafe impl Sync for SBBlock {}

#[cfg(feature = "graphql")]
#[graphql_object]
impl SBBlock {
    fn is_valid() -> bool {
        self.is_valid()
    }

    fn is_inlined() -> bool {
        self.is_inlined()
    }

    fn inlined_name() -> &str {
        self.inlined_name()
    }

    fn inlined_call_site_file() -> Option<SBFileSpec> {
        self.inlined_call_site_file()
    }

    // TODO(bm) This should be u32
    fn inlined_call_site_line() -> Option<i32> {
        self.inlined_call_site_line().map(|i| i as i32)
    }

    // TODO(bm) This should be u32
    fn inlined_call_site_column() -> Option<i32> {
        self.inlined_call_site_column().map(|i| i as i32)
    }
}
