use std::io::{stdout, Write};
use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {
    println!("probe started");
    stdout().flush()?;

    // Initialize the Ctrl-C handler
    ctrlc_tiny::init_ctrlc()?;

    // Poll until SIGINT is received
    loop {
        if ctrlc_tiny::is_ctrlc_received() {
            println!("Ctrl-C detected");
            stdout().flush()?;
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
