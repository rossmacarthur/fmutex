//! Provides mutual exclusion on a file using
//! [`flock(2)`](https://man7.org/linux/man-pages/man2/flock.2.html).

use std::fs;
use std::fs::File;
use std::io;
use std::os::unix::prelude::AsRawFd;
use std::path::Path;

/// When this structure is dropped, the file will be unlocked.
///
/// This structure is created by the [`lock`] and [`try_lock`] functions.
#[derive(Debug)]
pub struct Guard(File);

/// Acquires the file lock, blocking the current thread until it can.
///
/// Upon returning, the thread is the only thread / process with the lock held.
/// A guard is returned to allow scoped unlock of the lock. When the guard goes
/// out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
///
/// # Examples
///
/// ```
/// let path = "path/to/my/file.txt";
/// # let dir = temp_dir::TempDir::new().unwrap();
/// # let path = dir.child("test");
/// # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
///
/// {
///     let _guard = fmutex::lock(path)?;
///
///     // do mutually exclusive stuff here
///
/// } // <-- `_guard` dropped here and the lock is released
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn lock<P>(path: P) -> io::Result<Guard>
where
    P: AsRef<Path>,
{
    let guard = Guard::new(path.as_ref())?;
    lock_exclusive(&guard.0)?;
    Ok(guard)
}

/// Attempts to acquire the file lock, returning `None` if it is locked.
///
/// If the lock could not be acquired at this time, then `None` is returned.
/// Otherwise, a guard is returned to allow scoped unlock of the lock. When the
/// guard goes out of scope, the file will be unlocked.
///
/// # Errors
///
/// If the file cannot be read.
///
/// # Examples
///
/// ```
/// let path = "path/to/my/file.txt";
/// # let dir = temp_dir::TempDir::new().unwrap();
/// # let path = dir.child("test");
/// # std::fs::OpenOptions::new().create(true).write(true).open(&path).unwrap();
///
/// match fmutex::try_lock(path)? {
///     Some(_guard) => {
///
///         // do mutually exclusive stuff here
///
///     } // <-- `_guard` dropped here and the lock is released
///
///     None => {
///         eprintln!("the lock could not be acquired!");
///     }
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn try_lock<P>(path: P) -> io::Result<Option<Guard>>
where
    P: AsRef<Path>,
{
    let guard = Guard::new(path.as_ref())?;
    match try_lock_exclusive(&guard.0) {
        Ok(()) => Ok(Some(guard)),
        Err(err) if err.kind() == io::ErrorKind::WouldBlock => Ok(None),
        Err(err) => Err(err),
    }
}

impl Guard {
    fn new(path: &Path) -> io::Result<Self> {
        let file = fs::OpenOptions::new().read(true).open(path)?;
        Ok(Self(file))
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        unlock(&self.0).ok();
    }
}

fn lock_exclusive(file: &File) -> io::Result<()> {
    flock(file, libc::LOCK_EX)
}

fn try_lock_exclusive(file: &File) -> io::Result<()> {
    flock(file, libc::LOCK_EX | libc::LOCK_NB)
}

fn unlock(file: &File) -> io::Result<()> {
    flock(file, libc::LOCK_UN)
}

fn flock(file: &File, flag: libc::c_int) -> io::Result<()> {
    let r = unsafe { libc::flock(file.as_raw_fd(), flag) };
    match r {
        r if r < 0 => Err(io::Error::last_os_error()),
        _ => Ok(()),
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
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .unwrap();

        let path2 = path.clone();

        // Test
        let handle = thread::spawn(|| {
            let guard = lock(path).unwrap();
            thread::sleep(Duration::from_millis(200));
            drop(guard);
        });
        thread::sleep(Duration::from_millis(100));

        // Check that we are *not* able to acquire the lock while it is held
        // by the thread.
        assert!(try_lock(path2).unwrap().is_none());

        // Cleanup
        handle.join().unwrap();
    }
}
