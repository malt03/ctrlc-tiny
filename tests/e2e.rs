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
    let mut lines = Vec::new();
    let mut started = false;

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).expect("failed to read line") == 0 {
            break;
        }
        let line = line.trim();
        if started {
            lines.push(line.to_string());
        } else if line == "probe started" {
            started = true;
        }
        if started {
            unsafe {
                libc::kill(child_id as i32, libc::SIGINT);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    assert_eq!(
        lines,
        vec!["Ctrl-C detected: 1", "Ctrl-C detected: 2",],
        "Expected Ctrl-C detection in child output"
    );
}
