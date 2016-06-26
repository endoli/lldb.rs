// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # LLDB
//!
//! ## Installation
//!
//! This crate works with Cargo and is on [crates.io].
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! lldb = "0.0.1"
//! ```
//!
//! ### Mac OS X Installation Notes
//!
//! On Mac OS X, this library relies upon being able to find
//! the `LLDB.framework` that is provided by Xcode.app.
//!
//! In your own application, you will need to configure the
//! `@rpath` for your execuable to point to the location
//! of the `LLDB.framework` that you are using. Doing this
//! automatically is not currently supported by Cargo.
//!
//! For testing, you can use the version provided by
//! Xcode.app by setting an environment variable:
//!
//! ```shell
//! export DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/SharedFrameworks
//! ```
//!
//! [crates.io]: https://crates.io/crates/lldb

#![warn(missing_docs)]
#![deny(trivial_numeric_casts,
        unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate lldb_sys as sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
