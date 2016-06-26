// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// Represents a compilation unit or compiled source file.
#[derive(Debug)]
pub struct SBCompileUnit {
    /// The underlying raw `SBCompileUnitRef`.
    pub raw_compile_unit: sys::SBCompileUnitRef,
}
