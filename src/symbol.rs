// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sys;

/// the symbol possibly associated with a stack frame.
#[derive(Debug)]
pub struct SBSymbol {
    /// The underlying raw `SBSymbolRef`.
    pub raw_symbol: sys::SBSymbolRef,
}
