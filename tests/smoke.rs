use std::fs;
use std::thread;
use std::time::Duration;
use temp_dir::TempDir;

#[test]
fn smoke_exclusive() {
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
        let guard = fmutex::lock_exclusive(&file).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });

    thread::sleep(Duration::from_millis(250));
    assert!(fmutex::try_lock_exclusive(&file2).unwrap().is_none());

    // Cleanup
    handle.join().unwrap();
}

#[test]
fn smoke_exclusive_path() {
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
        let guard = fmutex::lock_exclusive_path(path).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });
    thread::sleep(Duration::from_millis(250));

    // Check that we are *not* able to acquire the lock while it is held
    // by the thread.
    assert!(fmutex::try_lock_exclusive_path(path2).unwrap().is_none());

    // Cleanup
    handle.join().unwrap();
}

#[test]
fn smoke_shared() {
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

    // Test that multiple shared locks can be acquired simultaneously
    let handle = thread::spawn(move || {
        let guard = fmutex::lock_shared(&file).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });

    thread::sleep(Duration::from_millis(250));

    // We should be able to acquire another shared lock while the first one is held
    let guard2 = fmutex::try_lock_shared(&file2).unwrap();
    assert!(guard2.is_some());
    drop(guard2);

    // Cleanup
    handle.join().unwrap();
}

#[test]
fn smoke_shared_path() {
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

    // Test that multiple shared locks can be acquired simultaneously
    let handle = thread::spawn(|| {
        let guard = fmutex::lock_shared_path(path).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });

    thread::sleep(Duration::from_millis(250));

    // We should be able to acquire another shared lock while the first one is held
    let guard2 = fmutex::try_lock_shared_path(path2).unwrap();
    assert!(guard2.is_some());
    drop(guard2);

    // Cleanup
    handle.join().unwrap();
}

#[test]
fn exclusive_blocks_shared() {
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

    // Test that a shared lock cannot be acquired when an exclusive lock is held
    let handle = thread::spawn(move || {
        let guard = fmutex::lock_exclusive(&file).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });

    thread::sleep(Duration::from_millis(250));

    // We should not be able to acquire a shared lock while an exclusive lock is held
    assert!(fmutex::try_lock_shared(&file2).unwrap().is_none());

    // Cleanup
    handle.join().unwrap();
}

#[test]
fn shared_blocks_exclusive() {
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

    // Test that an exclusive lock cannot be acquired when a shared lock is held
    let handle = thread::spawn(move || {
        let guard = fmutex::lock_shared(&file).unwrap();
        thread::sleep(Duration::from_millis(500));
        drop(guard);
    });

    thread::sleep(Duration::from_millis(250));

    // We should not be able to acquire an exclusive lock while a shared lock is held
    assert!(fmutex::try_lock_exclusive(&file2).unwrap().is_none());

    // Cleanup
    handle.join().unwrap();
}
