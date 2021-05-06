# lldb

[![](http://meritbadge.herokuapp.com/lldb)](https://crates.io/crates/lldb)

This crate provides a safe binding to the public API for [LLDB](https://lldb.llvm.org/) the
debugger provided by the [LLVM project](https://llvm.org/). LLDB provides a modern, high
performance debugger framework and is the default debugger for macOS
and iOS.

This builds upon a lower level crate [`lldb-sys`](https://github.com/endoli/lldb-sys.rs/).

Dual licensed under the MIT and Apache 2 licenses.

## Documentation

The API is fully documented with examples:
[https://endoli.github.io/lldb.rs/](https://endoli.github.io/lldb.rs/)

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/lldb).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
lldb = "0.0.8"
```

### Linux

Install the lldb and liblldb-dev packages for your platform so that you have both LLDB itself installed as well as the headers and other support files required. For example, on Ubuntu you can run `sudo apt install lldb liblldb-dev`.

### macOS

You will need to have 2 environment variables set to do the build:

    LLVM_ROOT - This points to the root of the LLVM source tree.
    LLVM_BUILD_ROOT - This points to the root of an LLVM build directory. This may be the same as the LLVM source tree, especially if you're working from a binary install.

For now, you will have to set an @rpath manually on your executables so that they can find the LLDB.framework at runtime. This can be done with install_name_tool:

install_name_tool -add_rpath /Applications/Xcode.app/Contents/SharedFrameworks target/debug/examples/barebones

Alternatively, for testing and local work, you can set the DYLD_FRAMEWORK_PATH:

    export DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/SharedFrameworks

## Development Guidelines

The official LLDB bindings for C++ and Python maintain very
strict backwards compatibility. This has resulted in them
having methods that are considered to be deprecated in favor
of newer methods with more comprehensive arguments. The Rust
bindings don't have that problem (yet), so we should try to
only write bindings for the recommended usages and not the
deprecated methods.

The documentation for the official bindings is also somewhat
spotty. While it would be great to contribute upstream to
improve that, we should seek to have these bindings be well
documented and with more examples.

People should be jealous of the quality of our bindings.
Sic itur ad astra.

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Support and Maintenance

I am developing this library largely on my own so far. I am able
to offer support and maintenance, but would very much appreciate
donations via [Patreon](https://patreon.com/endoli). I can also
provide commercial support, so feel free to
[contact me](mailto:bruce.mitchener@gmail.com).

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
