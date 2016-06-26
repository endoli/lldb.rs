# lldb

[![](http://meritbadge.herokuapp.com/lldb)](https://crates.io/crates/lldb)

Dual licensed under the MIT and Apache 2 licenses.

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/lldb).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
lldb = "0.0.1"
```

On Mac OS X, the `LLDB.framework` requires that an `@rpath`
be configured for your application so that the `LLDB.framework`
can be found. This isn't directly supported by Cargo today, but
for local work and development, you can do this:

```shell
export DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/SharedFrameworks
```

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
