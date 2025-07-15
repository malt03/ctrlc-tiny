//! Example: detecting multiple Ctrl-C presses
//!
//! This example demonstrates how to detect Ctrl-C multiple times
//! by resetting the internal flag after each detection.

fn main() -> std::io::Result<()> {
    ctrlc_tiny::init_ctrlc()?;

    let mut count = 0;
    loop {
        if ctrlc_tiny::is_ctrlc_received() {
            ctrlc_tiny::reset_ctrlc_received();

            count += 1;
            println!("SIGINT received {} time(s)", count);

            if count == 10 {
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
