use std::{io::BufRead, process::Command, thread, time::Duration};

#[test]
fn e2e_test() {
    let child = Command::new("cargo")
        .args(&["run", "--example", "ctrlc_probe"])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to start example");

    let child_id = child.id();
    let stdout = child.stdout.expect("failed to capture stdout");
    let mut reader = std::io::BufReader::new(stdout);
    let mut last_line: String = String::new();

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).expect("failed to read line") == 0 {
            break;
        }
        let line = line.trim();
        last_line = line.to_string();
        if line == "probe started" {
            thread::sleep(Duration::from_millis(50));
            unsafe {
                libc::kill(child_id as i32, libc::SIGINT);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    assert_eq!(
        last_line.trim(),
        "Ctrl-C detected",
        "Expected Ctrl-C detection in child output"
    );
}
