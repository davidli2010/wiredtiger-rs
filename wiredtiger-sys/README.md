# wiredtiger-sys

Rust bindings for [WiredTiger](https://github.com/wiredtiger/wiredtiger) API.

Released under the Apache License 2.0.

Notes that `WiredTiger` is released under *GPL*.

## Dependencies

By default, this crate will attempt to link `libwiredtiger` statically.

## Environment Variables

The following environment variables are used by this crate to find the required libraries and executables:

- `WIREDTIGER_HOME` - provides a path to `WiredTiger`'s install directory.
- `LLVM_CONFIG_PATH` - provides a full path to an `llvm-config` executable(including the executable itself).
