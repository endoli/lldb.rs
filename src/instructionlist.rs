// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{sys, SBInstruction, SBStream};
use std::fmt;

/// A list of [machine instructions].
///
/// [machine instructions]: SBInstruction
pub struct SBInstructionList {
    /// The underlying raw `SBInstructionListRef`.
    pub raw: sys::SBInstructionListRef,
}

impl SBInstructionList {
    /// Construct a new `SBInstructionList`.
    pub(crate) fn wrap(raw: sys::SBInstructionListRef) -> SBInstructionList {
        SBInstructionList { raw }
    }

    /// Construct a new `Some(SBInstructionList)` or `None`.
    #[allow(dead_code)]
    pub(crate) fn maybe_wrap(raw: sys::SBInstructionListRef) -> Option<SBInstructionList> {
        if unsafe { sys::SBInstructionListIsValid(raw) } {
            Some(SBInstructionList { raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBInstructionList` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBInstructionListIsValid(self.raw) }
    }

    /// Is this instruction list empty?
    pub fn is_empty(&self) -> bool {
        unsafe { sys::SBInstructionListGetSize(self.raw) == 0 }
    }

    /// Clear this instruction list.
    pub fn clear(&self) {
        unsafe { sys::SBInstructionListClear(self.raw) };
    }

    /// Append an instruction to this list.
    pub fn append_instruction(&self, instruction: SBInstruction) {
        unsafe { sys::SBInstructionListAppendInstruction(self.raw, instruction.raw) };
    }

    /// Iterate over this instruction list.
    pub fn iter(&self) -> SBInstructionListIter {
        SBInstructionListIter {
            instruction_list: self,
            idx: 0,
        }
    }
}

impl Clone for SBInstructionList {
    fn clone(&self) -> SBInstructionList {
        SBInstructionList {
            raw: unsafe { sys::CloneSBInstructionList(self.raw) },
        }
    }
}

impl fmt::Debug for SBInstructionList {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBInstructionListGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBInstructionList {{ {} }}", stream.data())
    }
}

impl Drop for SBInstructionList {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBInstructionList(self.raw) };
    }
}

impl<'d> IntoIterator for &'d SBInstructionList {
    type IntoIter = SBInstructionListIter<'d>;
    type Item = SBInstruction;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

unsafe impl Send for SBInstructionList {}
unsafe impl Sync for SBInstructionList {}

/// An iterator over the [instructions] in an [`SBInstructionList`].
///
/// [instructions]: SBInstruction
pub struct SBInstructionListIter<'d> {
    instruction_list: &'d SBInstructionList,
    idx: usize,
}

impl<'d> Iterator for SBInstructionListIter<'d> {
    type Item = SBInstruction;

    fn next(&mut self) -> Option<SBInstruction> {
        if self.idx < unsafe { sys::SBInstructionListGetSize(self.instruction_list.raw) } {
            let r = SBInstruction::wrap(unsafe {
                sys::SBInstructionListGetInstructionAtIndex(
                    self.instruction_list.raw,
                    self.idx as u32,
                )
            });
            self.idx += 1;
            Some(r)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = unsafe { sys::SBInstructionListGetSize(self.instruction_list.raw) };
        (sz - self.idx, Some(sz))
    }
}

impl<'d> ExactSizeIterator for SBInstructionListIter<'d> {}
