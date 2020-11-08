//! Split a single value implementing `AsyncRead + AsyncWrite` into separate
//! `AsyncRead` and `AsyncWrite` handles.
//!
//! To restore this read/write object from its `split::ReadHalf` and
//! `split::WriteHalf` use `unsplit`.

use crate::io::{AsyncRead, AsyncWrite, ReadBuf};

use pin_project_lite::pin_project;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

cfg_io_util! {
    /// Joins an `AsyncRead` and an `AsyncWrite` value into a single `AsyncRead + AsyncWrite`
    /// value.
    pub fn join<R, W>(read: R, write: W) -> Join<R, W>
    where
        R: AsyncRead,
        W: AsyncWrite,
    {
        Join { read, write }
    }

    pin_project! {
        /// The joined read and write value returned from [`join`](join()).
        pub struct Join<R: AsyncRead, W: AsyncWrite> {
            #[pin]
            read: R,
            #[pin]
            write: W,
        }
    }
    
    impl<R: AsyncRead, W: AsyncWrite> AsyncRead for Join<R, W> {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            let this = self.project();
            AsyncRead::poll_read(this.read, cx, buf)
        }
    }

    impl<R: AsyncRead, W: AsyncWrite> AsyncWrite for Join<R, W> {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>> {
            let this = self.project();
            AsyncWrite::poll_write(this.write, cx, buf)
        }
    
        fn poll_flush(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), io::Error>> {
            let this = self.project();
            AsyncWrite::poll_flush(this.write, cx)
        }
    
        fn poll_shutdown(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), io::Error>> {
            let this = self.project();
            AsyncWrite::poll_shutdown(this.write, cx)
        }
    }
}