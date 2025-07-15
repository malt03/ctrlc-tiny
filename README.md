# ctrlc-tiny

[![Crates.io Version](https://img.shields.io/crates/v/ctrlc-tiny)](https://crates.io/crates/ctrlc-tiny)
[![docs.rs](https://img.shields.io/docsrs/ctrlc-tiny)](https://docs.rs/ctrlc-tiny)
[![Test](https://github.com/malt03/ctrlc-tiny/actions/workflows/test.yml/badge.svg?event=release)](https://github.com/malt03/ctrlc-tiny/actions/workflows/test.yml)

A tiny crate for checking if Ctrl-C was pressed.

No handlers to set. No threads.  
Just call `init_ctrlc()` once, then check `is_ctrlc_received()` in your loop.

## ✨ Features

- Signal-safe `SIGINT` handler
- No threads, no allocations
- Zero dependencies
- Ideal for polling-based CLI tools

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
ctrlc-tiny = "0.1"
```

Example:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc_tiny::init_ctrlc()?;
    loop {
        if ctrlc_tiny::is_ctrlc_received() {
            println!("Ctrl-C detected");
            break;
        }
        // work...
    }

    Ok(())
}
```

## 🔍 Why not use `ctrlc`?

[`ctrlc`](https://crates.io/crates/ctrlc) provides a flexible way to handle signals using closures and shared state.

It spawns a thread and communicates through channels, which is great for many use cases but may be more than you need if you're just polling for Ctrl-C.

`ctrlc-tiny` is focused on one job: setting a flag when `SIGINT` is received.
No threads, no handlers, no extra logic.

## 🔒 Signal Safety

- Internally uses a `volatile sig_atomic_t` flag — safe in POSIX signal handlers.
- No heap, no threads — fully signal-safe by design.

## 🛠️ Platform Support

- ✅ Linux
- ✅ macOS
- ❌ Windows

## 📦 License

Licensed under either of:

- MIT
- Apache 2.0

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE).
