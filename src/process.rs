// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// Represents the process associated with the target program.
#[derive(Debug)]
pub struct SBProcess {
    /// The underlying raw `SBProcessRef`.
    pub raw_process: sys::SBProcessRef,
}
