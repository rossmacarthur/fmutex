use std::io;
use std::os::unix::io::AsRawFd;

pub use std::os::unix::io::AsFd;
pub use std::os::unix::io::BorrowedFd;

pub fn lock_exclusive(fd: BorrowedFd<'_>) -> io::Result<()> {
    flock(fd, libc::LOCK_EX)
}

pub fn try_lock_exclusive(fd: BorrowedFd<'_>) -> io::Result<bool> {
    match flock(fd, libc::LOCK_EX | libc::LOCK_NB) {
        Ok(()) => Ok(true),
        Err(err) if err.kind() == io::ErrorKind::WouldBlock => Ok(false),
        Err(err) => Err(err),
    }
}

pub fn unlock(fd: BorrowedFd<'_>) -> io::Result<()> {
    flock(fd, libc::LOCK_UN)
}

fn flock(fd: BorrowedFd<'_>, flag: libc::c_int) -> io::Result<()> {
    let r = unsafe { libc::flock(fd.as_raw_fd(), flag) };
    match r {
        r if r < 0 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}
