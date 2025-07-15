# ctrlc-tiny

A tiny, signal-safe `Ctrl-C` flag for polling-based programs.  
Designed to be minimal, fast, and portable — with no background threads or allocations.

[![crates.io](https://img.shields.io/crates/v/ctrlc-tiny.svg)](https://crates.io/crates/ctrlc-tiny)
[![docs.rs](https://docs.rs/ctrlc-tiny/badge.svg)](https://docs.rs/ctrlc-tiny)

---

## ✨ Features

- ✅ Signal-safe handler for `SIGINT` (Ctrl-C)
- ✅ No threads, no allocations
- ✅ Zero dependencies (except `libc`)
- ✅ Ideal for polling-style CLI programs

---

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
ctrlc-tiny = "0.1"
```

In your code:

```rust
fn main() -> Result<(), std::io::Error> {
ctrlc_tiny::init_ctrlc()?;

    loop {
        if ctrlc_tiny::is_ctrlc_received() {
            println!("Ctrl-C received!");
            break;
        }

        // do some work...
    }

    Ok(())

}
```

---

## 🔍 Why not use the `ctrlc` crate?

[`ctrlc`](https://crates.io/crates/ctrlc) is great, but:

- It uses background threads and message channels
- It isn't signal-safe in the strictest sense
- This crate is simpler, smaller, and purely flag-based

---

## 🔒 Safety Notes

- The internal flag is backed by a `sig_atomic_t`
- It is set exactly once per process lifetime and never reset

---

## 🛠️ Platform Support

- ✅ Linux
- ✅ macOS
- ✅ Unix-likes with `sigaction`
- ❌ Windows (not yet supported)

---

## 📦 License

Licensed under either of:

- MIT License
- Apache License 2.0

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE) for details.
