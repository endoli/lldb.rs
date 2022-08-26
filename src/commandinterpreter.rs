// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sys;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SBCommandInterpreter {
    /// The underlying raw `SBCommandInterpreterRef`.
    pub raw: sys::SBCommandInterpreterRef,
}

impl SBCommandInterpreter {
    /// Construct a new `SBCommandInterpreter`.
    pub(crate) fn wrap(raw: sys::SBCommandInterpreterRef) -> SBCommandInterpreter {
        SBCommandInterpreter { raw }
    }
}

impl Clone for SBCommandInterpreter {
    fn clone(&self) -> SBCommandInterpreter {
        SBCommandInterpreter {
            raw: unsafe { sys::CloneSBCommandInterpreter(self.raw) },
        }
    }
}

impl Drop for SBCommandInterpreter {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBCommandInterpreter(self.raw) };
    }
}

unsafe impl Send for SBCommandInterpreter {}
unsafe impl Sync for SBCommandInterpreter {}
