use std::io::{stdout, Write};
use std::{thread, time::Duration};

fn main() -> std::io::Result<()> {
    ctrlc_tiny::init_ctrlc(None)?;

    println!("probe started");
    stdout().flush()?;

    let mut count = 0;
    loop {
        if ctrlc_tiny::is_ctrlc_received() {
            count += 1;
            println!("Ctrl-C detected: {}", count);
            stdout().flush()?;
            if count == 2 {
                break;
            }
            ctrlc_tiny::reset_ctrlc_received();
        }
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
