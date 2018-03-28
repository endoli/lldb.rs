// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SBCommandInterpreter {
    /// The underlying raw `SBCommandInterpreterRef`.
    pub raw: sys::SBCommandInterpreterRef,
}

impl SBCommandInterpreter {
    /// Construct a new `SBCommandInterpreter`.
    pub fn wrap(raw: sys::SBCommandInterpreterRef) -> SBCommandInterpreter {
        SBCommandInterpreter { raw }
    }
}

impl Drop for SBCommandInterpreter {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBCommandInterpreter(self.raw) };
    }
}

#[cfg(feature = "graphql")]
graphql_object!(SBCommandInterpreter: super::debugger::SBDebugger | &self | {});
