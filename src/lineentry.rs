// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// Specifies an association with a contiguous range of
/// instructions and a source file location.
#[derive(Debug)]
pub struct SBLineEntry {
    /// The underlying raw `SBLineEntryRef`.
    pub raw_line_entry: sys::SBLineEntryRef,
}
