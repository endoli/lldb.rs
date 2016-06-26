// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// Represents a thread of execution.
#[derive(Debug)]
pub struct SBThread {
    /// The underlying raw `SBThreadRef`.
    pub raw_thread: sys::SBThreadRef,
}
