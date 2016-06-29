// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// A container that stores various debugger related info.
#[derive(Debug)]
pub struct SBSymbolContext {
    /// The underlying raw `SBSymbolContextRef`.
    pub raw: sys::SBSymbolContextRef,
}

impl SBSymbolContext {
    /// Construct a new `SBSymbolContext`.
    pub fn new(raw: sys::SBSymbolContextRef) -> SBSymbolContext {
        SBSymbolContext { raw: raw }
    }

    /// Construct a new `Some(SBSymbolContext)` or `None`.
    pub fn maybe(raw: sys::SBSymbolContextRef) -> Option<SBSymbolContext> {
        if unsafe { sys::SBSymbolContextIsValid(raw) != 0 } {
            Some(SBSymbolContext { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBSymbolContext` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBSymbolContextIsValid(self.raw) != 0 }
    }
}
