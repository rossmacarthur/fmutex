use std::io;
use std::mem;
use std::os::windows::io::AsRawHandle;
use windows_sys::Win32::Foundation::{ERROR_IO_PENDING, ERROR_LOCK_VIOLATION, HANDLE};
use windows_sys::Win32::Storage::FileSystem::{
    LockFileEx, UnlockFile, LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY,
};

pub use std::os::windows::io::AsHandle;
pub use std::os::windows::io::BorrowedHandle;

pub fn lock_exclusive(h: BorrowedHandle<'_>) -> io::Result<()> {
    lock_file(h, LOCKFILE_EXCLUSIVE_LOCK)
}

pub fn try_lock_exclusive(h: BorrowedHandle<'_>) -> io::Result<bool> {
    match lock_file(h, LOCKFILE_EXCLUSIVE_LOCK | LOCKFILE_FAIL_IMMEDIATELY) {
        Ok(()) => Ok(true),
        Err(err)
            if err.raw_os_error() == Some(ERROR_IO_PENDING as i32)
                || err.raw_os_error() == Some(ERROR_LOCK_VIOLATION as i32) =>
        {
            Ok(false)
        }
        Err(err) => Err(err),
    }
}

pub fn unlock(h: BorrowedHandle<'_>) -> io::Result<()> {
    unsafe {
        let r = UnlockFile(h.as_raw_handle() as HANDLE, 0, 0, !0, !0);
        match r {
            0 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }
}

fn lock_file(h: BorrowedHandle<'_>, flags: u32) -> io::Result<()> {
    unsafe {
        let mut overlapped = mem::zeroed();
        let r = LockFileEx(
            h.as_raw_handle() as HANDLE,
            flags,
            0,
            !0,
            !0,
            &mut overlapped,
        );
        match r {
            0 => Err(io::Error::last_os_error()),
            _ => Ok(()),
        }
    }
}
