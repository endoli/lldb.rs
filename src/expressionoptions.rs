// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sys;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SBExpressionOptions {
    /// The underlying raw `SBExpressionOptionsRef`.
    pub raw: sys::SBExpressionOptionsRef,
}

impl SBExpressionOptions {
    /// Construct a new `SBExpressionOptions`.
    pub fn new() -> SBExpressionOptions {
        SBExpressionOptions::wrap(unsafe { sys::CreateSBExpressionOptions() })
    }

    /// Construct a new `SBExpressionOptions`.
    pub(crate) fn wrap(raw: sys::SBExpressionOptionsRef) -> SBExpressionOptions {
        SBExpressionOptions { raw }
    }

    /// Whether to unwind the expression stack on error.
    pub fn unwind_on_error(&self) -> bool {
        unsafe { sys::SBExpressionOptionsGetUnwindOnError(self.raw) }
    }

    /// Whether to unwind the expression stack on error.
    pub fn set_unwind_on_error(&self, unwind: bool) {
        unsafe { sys::SBExpressionOptionsSetUnwindOnError(self.raw, unwind) };
    }

    /// Whether to ignore breakpoint hits while running expressions.
    pub fn ignore_breakpoints(&self) -> bool {
        unsafe { sys::SBExpressionOptionsGetIgnoreBreakpoints(self.raw) }
    }

    /// Whether to ignore breakpoint hits while running expressions.
    pub fn set_ignore_breakpoints(&self, ignore: bool) {
        unsafe { sys::SBExpressionOptionsSetIgnoreBreakpoints(self.raw, ignore) };
    }
}

impl Clone for SBExpressionOptions {
    fn clone(&self) -> SBExpressionOptions {
        SBExpressionOptions {
            raw: unsafe { sys::CloneSBExpressionOptions(self.raw) },
        }
    }
}

impl Default for SBExpressionOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SBExpressionOptions {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBExpressionOptions(self.raw) };
    }
}

unsafe impl Send for SBExpressionOptions {}
unsafe impl Sync for SBExpressionOptions {}
