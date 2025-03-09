use std::io;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct BorrowedFallback<'a> {
    _marker: PhantomData<&'a ()>,
}

impl BorrowedFallback<'_> {
    pub fn new() -> BorrowedFallback<'static> {
        BorrowedFallback {
            _marker: PhantomData,
        }
    }
}

pub fn lock_exclusive(_f: BorrowedFallback<'_>) -> io::Result<()> {
    Err(err())
}

pub fn try_lock_exclusive(_f: BorrowedFallback<'_>) -> io::Result<bool> {
    Err(err())
}

pub fn unlock(_f: BorrowedFallback<'_>) -> io::Result<()> {
    Err(err())
}

fn err() -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        "exclusive lock is not supported on this platform",
    )
}
