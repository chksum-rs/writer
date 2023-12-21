# chksum-writer

[![GitHub](https://img.shields.io/badge/github-chksum--rs%2Fwriter-24292e?style=flat-square&logo=github "GitHub")](https://github.com/chksum-rs/writer)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/writer/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/writer/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/chksum-writer?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum-writer/)
[![MSRV](https://img.shields.io/badge/MSRV-1.70.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/writer/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-writer/0.0.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-writer/0.0.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/writer?style=flat-square "LICENSE")](https://github.com/chksum-rs/writer/blob/master/LICENSE)

A convenient interface for calculating hash digests on the fly while writing data to a writer.

## Setup

To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:

```toml
[dependencies]
chksum-writer = "0.0.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```sh
cargo add chksum-writer
```

## Examples

For implementation-specific examples, refer to the documentation of the following crates:

* [`chksum-md5`](https://crates.io/crates/chksum-md5)
* [`chksum-sha1`](https://crates.io/crates/chksum-sha1)
* [`chksum-sha2`](https://crates.io/crates/chksum-sha2)

## License

This crate is licensed under the MIT License.
