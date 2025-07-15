# ctrlc-tiny

[![Crates.io Version](https://img.shields.io/crates/v/ctrlc-tiny)](https://crates.io/crates/ctrlc-tiny)
[![docs.rs](https://img.shields.io/docsrs/ctrlc-tiny)](https://docs.rs/ctrlc-tiny)
[![Test](https://github.com/malt03/ctrlc-tiny/actions/workflows/test.yml/badge.svg?event=release)](https://github.com/malt03/ctrlc-tiny/actions/workflows/test.yml)

A tiny crate for checking if Ctrl-C was pressed.

No handlers to set. No threads. No `AtomicBool`.  
Just call `init_ctrlc()` once, then check `is_ctrlc_received()` in your loop.

## ✨ Features

- Signal-safe `SIGINT` handler
- No threads, no allocations
- No runtime Rust dependencies
- Ideal for polling-based CLI tools

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
ctrlc-tiny = "0.1"
```

Example:

```rust
fn main() -> std::io::Result<()> {
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

Need to detect Ctrl-C more than once? See [`examples/multi_ctrlc.rs`](https://github.com/malt03/ctrlc-tiny/blob/main/examples/multi_ctrlc.rs).

## 🔍 Why not use `ctrlc`?

[`ctrlc`](https://crates.io/crates/ctrlc) is great when you want to run custom logic when Ctrl-C is pressed.

But if you just want to check whether Ctrl-C was pressed, it can feel more involved than necessary.

`ctrlc-tiny` keeps things simple: a single flag you can poll.

## 🔒 Signal Safety

- Internally uses a `volatile sig_atomic_t` flag — safe in POSIX signal handlers.
- No heap, no threads — fully signal-safe by design.
- The flag can be reset via `reset_ctrlc_received()`, but may race with the signal handler if SIGINT is received at the same time.

## 🛠️ Platform Support

- ✅ Linux
- ✅ macOS
- ❌ Windows (no plans to add support)

## 📦 License

Licensed under either of:

- MIT
- Apache 2.0

See [LICENSE-MIT](LICENSE-MIT) or [LICENSE-APACHE](LICENSE-APACHE).
