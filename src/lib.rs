//! Mutual exclusion across processes on a file descriptor or path.
//!
//! - On Unix-like systems this is implemented use
//!   [`flock(2)`](https://man7.org/linux/man-pages/man2/flock.2.html).
//! - On Windows this is implemented using
//!   [`LockFileEx`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex).
//!
//! # 🚀 Getting started
//!
//! First `fmutex` to your Cargo manifest.
//!
//! ```sh
//! cargo add fmutex
//! ```
//!
//! Now use one of the provided functions to lock a file descriptor (Unix) or
//! handle (Windows) or a file path.
//!
//! - [`lock()`](#lock) to acquire a lock on a file descriptor or handle.
//! - [`try_lock()`](#try_lock) to attempt to acquire a lock on a file
//!   descriptor or handle.
//! - [`lock_path()`](#lock_path) to acquire a lock on a file path.
//! - [`try_lock_path()`](#try_lock_path) to attempt to acquire a lock on a file
//!   path.
//!
//! # 🤸 Usage
//!
//! ## [`lock()`]
//!
//! ```
//! # use std::fs;
//! # let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! let fd = fs::OpenOptions::new().create(true).write(true).open(&path)?;
//!
//! {
//!     let _guard = fmutex::lock(&fd)?;
//!
//!     // do mutually exclusive stuff here
//!
//! } // <-- `_guard` dropped here and the lock is released
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`try_lock()`]
//!
//! ```
//! # use std::fs;
//! # let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! let fd = fs::OpenOptions::new().create(true).write(true).open(&path)?;
//!
//! match fmutex::try_lock(&fd)? {
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
//! ## [`lock_path()`]
//!
//! ```
//! let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
//!
//! {
//!     let _guard = fmutex::lock_path(path)?;
//!
//!     // do mutually exclusive stuff here
//!
//! } // <-- `_guard` dropped here and the lock is released
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## [`try_lock_path()`]
//!
//! ```
//! let path = "path/to/my/file.txt";
//! # let dir = temp_dir::TempDir::new().unwrap();
//! # let path = dir.child("test");
//! # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
//!
//! match fmutex::try_lock_path(path)? {
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

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

use std::fs;
use std::io;
use std::path::Path;

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
///
/// This structure is created by the [`lock`] and [`try_lock`] functions.
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

    pub(crate) fn unlock(&self) -> io::Result<()> {
        sys::unlock(self.borrow().inner)
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.f.unlock().ok();
    }
}

/// Acquires a lock on the resource, blocking the current thread until it can.
///
/// Upon returning, the thread is the only thread / process with the lock held.
/// A guard is returned to allow scoped unlock of the lock. When the guard goes
/// out of scope, the resource will be unlocked.
///
/// # Errors
///
/// If the resource cannot be read.
pub fn lock<F>(f: &F) -> io::Result<Guard<'_>>
where
    F: AsResource,
{
    let f = File::Borrowed(f.as_resource());
    f.lock_exclusive()?;
    Ok(Guard { f })
}

/// Attempts to acquire a lock on the resource, returning `None` if it is
/// locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the resource will be unlocked.
///
/// # Errors
///
/// If the file descriptor cannot be read.
pub fn try_lock<F>(f: &F) -> io::Result<Option<Guard<'_>>>
where
    F: AsResource,
{
    let f = File::Borrowed(f.as_resource());
    match f.try_lock_exclusive()? {
        true => Ok(Some(Guard { f })),
        false => Ok(None),
    }
}

/// Acquires the lock for the file at the given path, blocking the current
/// thread until it can.
///
/// Upon returning, the thread is the only thread / process with the lock held.
/// A guard is returned to allow scoped unlock of the lock. When the guard goes
/// out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
pub fn lock_path<P>(path: P) -> io::Result<Guard<'static>>
where
    P: AsRef<Path>,
{
    let f = File::with_path(path.as_ref())?;
    f.lock_exclusive()?;
    Ok(Guard { f })
}

/// Attempts to acquire the lock for the file at the given path, returning
/// `None` if it is locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
pub fn try_lock_path<P>(path: P) -> io::Result<Option<Guard<'static>>>
where
    P: AsRef<Path>,
{
    let f = File::with_path(path.as_ref())?;
    match f.try_lock_exclusive()? {
        true => Ok(Some(Guard { f })),
        false => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;
    use std::time::Duration;
    use temp_dir::TempDir;

    #[test]
    fn smoke() {
        // Setup
        let dir = TempDir::new().unwrap();
        let path = dir.child("test");
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)
            .unwrap();

        let file2 = fs::OpenOptions::new().read(true).open(&path).unwrap();

        let handle = thread::spawn(move || {
            let guard = crate::lock(&file).unwrap();
            thread::sleep(Duration::from_millis(500));
            drop(guard);
        });

        thread::sleep(Duration::from_millis(250));
        assert!(crate::try_lock(&file2).unwrap().is_none());

        // Cleanup
        handle.join().unwrap();
    }

    #[test]
    fn smoke_path() {
        // Setup
        let dir = TempDir::new().unwrap();
        let path = dir.child("test");
        fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)
            .unwrap();

        let path2 = path.clone();

        // Test
        let handle = thread::spawn(|| {
            let guard = crate::lock_path(path).unwrap();
            thread::sleep(Duration::from_millis(500));
            drop(guard);
        });
        thread::sleep(Duration::from_millis(250));

        // Check that we are *not* able to acquire the lock while it is held
        // by the thread.
        assert!(crate::try_lock_path(path2).unwrap().is_none());

        // Cleanup
        handle.join().unwrap();
    }
}
