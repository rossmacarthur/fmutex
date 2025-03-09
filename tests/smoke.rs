use std::fs;
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
        let guard = fmutex::lock(&file).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });

    thread::sleep(Duration::from_millis(250));
    assert!(fmutex::try_lock(&file2).unwrap().is_none());

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
        let guard = fmutex::lock_path(path).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });
    thread::sleep(Duration::from_millis(250));

    // Check that we are *not* able to acquire the lock while it is held
    // by the thread.
    assert!(fmutex::try_lock_path(path2).unwrap().is_none());

    // Cleanup
    handle.join().unwrap();
}
