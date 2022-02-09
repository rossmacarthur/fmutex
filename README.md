# fmutex

[![Crates.io Version](https://img.shields.io/crates/v/fmutex.svg)](https://crates.io/crates/fmutex)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/fmutex)

Provides mutual exclusion on a file using
[`flock(2)`](https://man7.org/linux/man-pages/man2/flock.2.html).

## Usage

### `lock()`

```rust
{
    let _guard = fmutex::lock(path)?;

    // do mutually exclusive stuff here

} // <-- `_guard` dropped here and the lock is released
```

### `try_lock()`

```rust
match fmutex::try_lock(path)? {
    Some(_guard) => {

        // do mutually exclusive stuff here

    } // <-- `_guard` dropped here and the lock is released

    None => {
        eprintln!("warn: the lock could not be acquired!");
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
