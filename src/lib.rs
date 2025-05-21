//! Mutual exclusion across processes on a file descriptor or path.
//!
//! - On Unix-like systems this is implemented use
//!   [`flock(2)`](https://man7.org/linux/man-pages/man2/flock.2.html).
//! - On Windows this is implemented using
//!   [`LockFileEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex).
//!
//! # 🚀 Getting started
//!
//! First add `fmutex` to your Cargo manifest.
//!
//! ```sh
//! cargo add fmutex
//! ```
//!
//! Now use one of the provided functions to lock a file descriptor (Unix) or
//! handle (Windows) or a file path.
//!
//! For exclusive locks (only one process can hold the lock):
//! - [`lock_exclusive()`](#lock_exclusive) to acquire an exclusive lock on a
//!   file descriptor or handle.
//! - [`try_lock_exclusive()`](#try_lock_exclusive) to attempt to acquire an
//!   exclusive lock on a file descriptor or handle.
//! - [`lock_exclusive_path()`](#lock_exclusive_path) to acquire an exclusive
//!   lock on a file path.
//! - [`try_lock_exclusive_path()`](#try_lock_exclusive_path) to attempt to
//!   acquire an exclusive lock on a file path.
//!
//! For shared locks (multiple processes can hold the lock simultaneously, but
//! not when an exclusive lock is held):
//! - [`lock_shared()`](#lock_shared) to acquire a shared lock on a file
//!   descriptor or handle.
//! - [`try_lock_shared()`](#try_lock_shared) to attempt to acquire a shared
//!   lock on a file descriptor or handle.
//! - [`lock_shared_path()`](#lock_shared_path) to acquire a shared lock on a
//!   file path.
//! - [`try_lock_shared_path()`](#try_lock_shared_path) to attempt to acquire a
//!   shared lock on a file path.
//!
//! # 🤸 Usage
//!
//! ## [`lock_exclusive()`]
//!
//! ```
//! # use std::fs;
//! # let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! let fd = fs::OpenOptions::new().create(true).write(true).open(&path)?;
//!
//! {
//!     let _guard = fmutex::lock_exclusive(&fd)?;
//!
//!     // do mutually exclusive stuff here
//!
//! } // <-- `_guard` dropped here and the lock is released
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`try_lock_exclusive()`]
//!
//! ```
//! # use std::fs;
//! # let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! let fd = fs::OpenOptions::new().create(true).write(true).open(&path)?;
//!
//! match fmutex::try_lock_exclusive(&fd)? {
//!     Some(_guard) => {
//!
//!         // do mutually exclusive stuff here
//!
//!     } // <-- `_guard` dropped here and the lock is released
//!
//!     None => {
//!         eprintln!("the lock could not be acquired!");
//!     }
//! }
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`lock_exclusive_path()`]
//!
//! ```
//! let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
//!
//! {
//!     let _guard = fmutex::lock_exclusive_path(path)?;
//!
//!     // do mutually exclusive stuff here
//!
//! } // <-- `_guard` dropped here and the lock is released
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`try_lock_exclusive_path()`]
//!
//! ```
//! let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
//!
//! match fmutex::try_lock_exclusive_path(path)? {
//!     Some(_guard) => {
//!
//!         // do mutually exclusive stuff here
//!
//!     } // <-- `_guard` dropped here and the lock is released
//!
//!     None => {
//!         eprintln!("the lock could not be acquired!");
//!     }
//! }
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`lock_shared()`]
//!
//! ```
//! # use std::fs;
//! # let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! let fd = fs::OpenOptions::new().create(true).write(true).open(&path)?;
//!
//! {
//!     let _guard = fmutex::lock_shared(&fd)?;
//!
//!     // do shared read-only operations here
//!     // other processes can also acquire shared locks simultaneously
//!
//! } // <-- `_guard` dropped here and the lock is released
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`try_lock_shared()`]
//!
//! ```
//! # use std::fs;
//! # let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! let fd = fs::OpenOptions::new().create(true).write(true).open(&path)?;
//!
//! match fmutex::try_lock_shared(&fd)? {
//!     Some(_guard) => {
//!
//!         // do shared read-only operations here
//!         // other processes can also acquire shared locks simultaneously
//!
//!     } // <-- `_guard` dropped here and the lock is released
//!
//!     None => {
//!         eprintln!("the shared lock could not be acquired (file is exclusively locked)!");
//!     }
//! }
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`lock_shared_path()`]
//!
//! ```
//! let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
//!
//! {
//!     let _guard = fmutex::lock_shared_path(path)?;
//!
//!     // do shared read-only operations here
//!     // other processes can also acquire shared locks simultaneously
//!
//! } // <-- `_guard` dropped here and the lock is released
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`try_lock_shared_path()`]
//!
//! ```
//! let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
//!
//! match fmutex::try_lock_shared_path(path)? {
//!     Some(_guard) => {
//!
//!         // do shared read-only operations here
//!         // other processes can also acquire shared locks simultaneously
//!
//!     } // <-- `_guard` dropped here and the lock is released
//!
//!     None => {
//!         eprintln!("the shared lock could not be acquired (file is exclusively locked)!");
//!     }
//! }
//! # Ok::<(), std::io::Error>(())
//! ```

#[cfg(all(not(unix), not(windows)))]
mod fallback;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use std::fs;
use std::io;
use std::path::Path;

#[cfg(all(not(unix), not(windows)))]
use crate::fallback as sys;
#[cfg(unix)]
use crate::unix as sys;
#[cfg(windows)]
use crate::windows as sys;

/// Cross-platform version of [`AsFd`] and [`AsHandle`].
///
/// [`AsFd`]: std::os::unix::io::AsFd
/// [`AsHandle`]: std::os::windows::io::AsHandle
pub trait AsResource {
    fn as_resource(&self) -> BorrowedResource<'_>;
}

/// When this structure is dropped, the file will be unlocked.
#[derive(Debug)]
pub struct Guard<'a> {
    f: File<'a>,
}

/// Cross-platform borrowed file.
#[derive(Debug)]
pub(crate) enum File<'a> {
    Borrowed(BorrowedResource<'a>),
    Owned(fs::File),
}

/// Cross-platform version of [`BorrowedFd`] and [`BorrowedHandle`].
///
/// [`BorrowedFd`]: std::os::unix::io::BorrowedFd
/// [`BorrowedHandle`]: std::os::windows::io::BorrowedHandle
#[derive(Debug, Clone, Copy)]
pub struct BorrowedResource<'a> {
    #[cfg(unix)]
    pub(crate) inner: sys::BorrowedFd<'a>,
    #[cfg(windows)]
    pub(crate) inner: sys::BorrowedHandle<'a>,
    #[cfg(all(not(unix), not(windows)))]
    pub(crate) inner: sys::BorrowedFallback<'a>,
}

#[cfg(unix)]
impl<T> AsResource for T
where
    T: sys::AsFd,
{
    fn as_resource(&self) -> BorrowedResource<'_> {
        BorrowedResource {
            inner: sys::AsFd::as_fd(self),
        }
    }
}

#[cfg(windows)]
impl<T> AsResource for T
where
    T: sys::AsHandle,
{
    fn as_resource(&self) -> BorrowedResource<'_> {
        BorrowedResource {
            inner: sys::AsHandle::as_handle(self),
        }
    }
}

#[cfg(all(not(unix), not(windows)))]
impl<T> AsResource for T {
    fn as_resource(&self) -> BorrowedResource<'_> {
        BorrowedResource {
            inner: sys::BorrowedFallback::new(),
        }
    }
}

impl File<'_> {
    pub(crate) fn with_path(path: &Path) -> io::Result<Self> {
        Ok(File::Owned(fs::OpenOptions::new().read(true).open(path)?))
    }

    pub(crate) fn borrow(&self) -> BorrowedResource<'_> {
        match self {
            &File::Borrowed(f) => f,
            File::Owned(f) => f.as_resource(),
        }
    }

    pub(crate) fn lock_exclusive(&self) -> io::Result<()> {
        sys::lock_exclusive(self.borrow().inner)
    }

    pub(crate) fn try_lock_exclusive(&self) -> io::Result<bool> {
        sys::try_lock_exclusive(self.borrow().inner)
    }

    pub(crate) fn lock_shared(&self) -> io::Result<()> {
        sys::lock_shared(self.borrow().inner)
    }

    pub(crate) fn try_lock_shared(&self) -> io::Result<bool> {
        sys::try_lock_shared(self.borrow().inner)
    }

    pub(crate) fn unlock(&self) -> io::Result<()> {
        sys::unlock(self.borrow().inner)
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.f.unlock().ok();
    }
}

/// Acquires an exclusive lock on the resource, blocking the current thread
/// until it can.
///
/// Upon returning, the thread is the only thread / process with the lock held.
/// A guard is returned to allow scoped unlock of the lock. When the guard goes
/// out of scope, the resource will be unlocked.
///
/// # Errors
///
/// If the resource cannot be read.
pub fn lock_exclusive<F>(f: &F) -> io::Result<Guard<'_>>
where
    F: AsResource,
{
    let f = File::Borrowed(f.as_resource());
    f.lock_exclusive()?;
    Ok(Guard { f })
}

/// Attempts to acquire an exclusive lock on the resource, returning `None` if
/// it is locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the resource will be unlocked.
///
/// # Errors
///
/// If the file descriptor cannot be read.
pub fn try_lock_exclusive<F>(f: &F) -> io::Result<Option<Guard<'_>>>
where
    F: AsResource,
{
    let f = File::Borrowed(f.as_resource());
    match f.try_lock_exclusive()? {
        true => Ok(Some(Guard { f })),
        false => Ok(None),
    }
}

/// Acquires an exclusive lock for the file at the given path, blocking the
/// current thread until it can.
///
/// Upon returning, the thread is the only thread / process with the lock held.
/// A guard is returned to allow scoped unlock of the lock. When the guard goes
/// out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
pub fn lock_exclusive_path<P>(path: P) -> io::Result<Guard<'static>>
where
    P: AsRef<Path>,
{
    let f = File::with_path(path.as_ref())?;
    f.lock_exclusive()?;
    Ok(Guard { f })
}

/// Attempts to acquire an exclusive lock for the file at the given path,
/// returning `None` if it is locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
pub fn try_lock_exclusive_path<P>(path: P) -> io::Result<Option<Guard<'static>>>
where
    P: AsRef<Path>,
{
    let f = File::with_path(path.as_ref())?;
    match f.try_lock_exclusive()? {
        true => Ok(Some(Guard { f })),
        false => Ok(None),
    }
}

/// Acquires a shared lock on the resource, blocking the current thread until it
/// can.
///
/// Upon returning, the thread is one of potentially many threads / processes
/// with the lock held. Multiple shared locks can be held simultaneously, but
/// not with any exclusive locks. A guard is returned to allow scoped unlock of
/// the lock. When the guard goes out of scope, the resource will be unlocked.
///
/// # Errors
///
/// If the resource cannot be read.
pub fn lock_shared<F>(f: &F) -> io::Result<Guard<'_>>
where
    F: AsResource,
{
    let f = File::Borrowed(f.as_resource());
    f.lock_shared()?;
    Ok(Guard { f })
}

/// Attempts to acquire a shared lock on the resource, returning `None` if it is
/// exclusively locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the resource will be unlocked.
///
/// # Errors
///
/// If the file descriptor cannot be read.
pub fn try_lock_shared<F>(f: &F) -> io::Result<Option<Guard<'_>>>
where
    F: AsResource,
{
    let f = File::Borrowed(f.as_resource());
    match f.try_lock_shared()? {
        true => Ok(Some(Guard { f })),
        false => Ok(None),
    }
}

/// Acquires a shared lock for the file at the given path, blocking the current
/// thread until it can.
///
/// Upon returning, the thread is one of potentially many threads / processes
/// with the lock held. Multiple shared locks can be held simultaneously, but
/// not with any exclusive locks. A guard is returned to allow scoped unlock of
/// the lock. When the guard goes out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
pub fn lock_shared_path<P>(path: P) -> io::Result<Guard<'static>>
where
    P: AsRef<Path>,
{
    let f = File::with_path(path.as_ref())?;
    f.lock_shared()?;
    Ok(Guard { f })
}

/// Attempts to acquire a shared lock for the file at the given path, returning
/// `None` if it is exclusively locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
pub fn try_lock_shared_path<P>(path: P) -> io::Result<Option<Guard<'static>>>
where
    P: AsRef<Path>,
{
    let f = File::with_path(path.as_ref())?;
    match f.try_lock_shared()? {
        true => Ok(Some(Guard { f })),
        false => Ok(None),
    }
}
