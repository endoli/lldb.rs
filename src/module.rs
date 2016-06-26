// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// Represents an executable image and its associated object and symbol files.
#[derive(Debug)]
pub struct SBModule {
    /// The underlying raw `SBModuleRef`.
    pub raw_module: sys::SBModuleRef,
}
