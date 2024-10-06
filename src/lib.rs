//! This crate provides a convenient interface for calculating hash digests on the fly while writing data to a writer. It supports various hash algorithms, and the library is designed to be easy to use.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum-writer = "0.0.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum-writer
//! ```     
//!
//! # Features
//!
//! ## Asynchronous Runtime
//!
//! * `async-runtime-tokio`: Enables async interface for Tokio runtime.
//!
//! By default, neither of these features is enabled.
//!
//! # Usage
//!
//! ```rust,ignore
//! use std::io::{self, Write};
//!
//! use chksum_md5::MD5;
//! use chksum_writer::Writer;
//!
//! fn main() -> io::Result<()> {
//!     // Create a new writer with the MD5 hash algorithm
//!     let mut writer = Writer::<_, MD5>::new(io::stdout());
//!
//!     // Write data to the writer
//!     writer.write_all(b"example data")?;
//!
//!     // Get the calculated digest
//!     let digest = writer.digest();
//!
//!     // Print the digest (hex representation)
//!     println!("Digest: {}", digest);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Implementations
//!
//! This crate should be used along with a hash implementation crate.
//!  
//! Various crates implement their own [`Writer`], which can be enabled with the `writer` Cargo feature.
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

use std::io::{self, Write};
#[cfg(feature = "async-runtime-tokio")]
use std::pin::{pin, Pin};
#[cfg(feature = "async-runtime-tokio")]
use std::task::{Context, Poll};

use chksum_core::Hash;
#[cfg(feature = "async-runtime-tokio")]
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Creates new [`Writer`].
pub fn new<H>(inner: impl Write) -> Writer<impl Write, H>
where
    H: Hash,
{
    Writer::new(inner)
}

/// Creates new [`Writer`] with provided hash.
pub fn with_hash<H>(inner: impl Write, hash: H) -> Writer<impl Write, H>
where
    H: Hash,
{
    Writer::with_hash(inner, hash)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`].
pub fn async_new<H>(inner: impl AsyncWrite) -> AsyncWriter<impl AsyncWrite, H>
where
    H: Hash,
{
    AsyncWriter::new(inner)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`] with provided hash.
pub fn async_with_hash<H>(inner: impl AsyncWrite, hash: H) -> AsyncWriter<impl AsyncWrite, H>
where
    H: Hash,
{
    AsyncWriter::with_hash(inner, hash)
}

/// Wraps a writer and calculates the hash digest on the fly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Writer<W, H>
where
    W: Write,
    H: Hash,
{
    inner: W,
    hash: H,
}

impl<W, H> Writer<W, H>
where
    W: Write,
    H: Hash,
{
    /// Creates new [`Writer`].
    pub fn new(inner: W) -> Self {
        let hash = H::default();
        Self::with_hash(inner, hash)
    }

    /// Creates new [`Writer`] with provided hash.
    #[must_use]
    pub const fn with_hash(inner: W, hash: H) -> Self {
        Self { inner, hash }
    }

    /// Unwraps this [`Writer`], returning the underlying writer.
    #[must_use]
    pub fn into_inner(self) -> W {
        let Self { inner, .. } = self;
        inner
    }

    /// Returns calculated hash digest.
    #[must_use]
    pub fn digest(&self) -> H::Digest {
        self.hash.digest()
    }
}

impl<W, H> Write for Writer<W, H>
where
    W: Write,
    H: Hash,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.hash.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Wraps a reader and calculates the hash digest on the fly.
#[cfg(feature = "async-runtime-tokio")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AsyncWriter<W, H>
where
    W: AsyncWriteExt,
    H: Hash,
{
    inner: W,
    hash: H,
}

#[cfg(feature = "async-runtime-tokio")]
impl<W, H> AsyncWriter<W, H>
where
    W: AsyncWriteExt,
    H: Hash,
{
    /// Creates new [`AsyncWriter`].
    pub fn new(inner: W) -> Self {
        let hash = H::default();
        Self::with_hash(inner, hash)
    }

    /// Creates new [`AsyncWriter`] with provided hash.
    #[must_use]
    pub const fn with_hash(inner: W, hash: H) -> Self {
        Self { inner, hash }
    }

    /// Unwraps this [`AsyncWriter`], returning the underlying writer.
    #[must_use]
    pub fn into_inner(self) -> W {
        let Self { inner, .. } = self;
        inner
    }

    /// Returns calculated hash digest.
    #[must_use]
    pub fn digest(&self) -> H::Digest {
        self.hash.digest()
    }
}

#[cfg(feature = "async-runtime-tokio")]
impl<W, H> AsyncWrite for AsyncWriter<W, H>
where
    W: AsyncWrite + Unpin,
    H: Hash + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, io::Error>> {
        let Self { inner, hash } = self.get_mut();
        match pin!(inner).poll_write(cx, buf) {
            Poll::Ready(Ok(n)) => {
                hash.update(&buf[..n]);
                Poll::Ready(Ok(n))
            },
            poll => poll,
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let Self { inner, .. } = self.get_mut();
        pin!(inner).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let Self { inner, .. } = self.get_mut();
        pin!(inner).poll_shutdown(cx)
    }
}
