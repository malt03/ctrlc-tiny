use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    ctrlc_tiny::init_ctrlc_with_print("Ctrl+C pressed\n")?;

    println!("e2e_init_ctrlc_with_print started");
    stdout().flush()?;

    while !ctrlc_tiny::is_ctrlc_received() {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    
    println!("Finished");
    stdout().flush()?;

    Ok(())
}