use std::{
    io::{BufRead, Read},
    process::Command,
    thread,
    time::Duration,
};

#[test]
fn e2e_init_ctrlc_test() {
    let child = Command::new("cargo")
        .args(&["run", "--example", "e2e_init_ctrlc"])
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
        } else if line == "e2e_init_ctrlc started" {
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

#[test]
fn e2e_init_ctrlc_with_print_test() {
    let child = Command::new("cargo")
        .args(&["run", "--example", "e2e_init_ctrlc_with_print"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to start example");

    let child_id = child.id();
    let stdout = child.stdout.expect("failed to capture stdout");
    let stderr = child.stderr.expect("failed to capture stderr");
    let mut stdout_reader = std::io::BufReader::new(stdout);
    let mut stderr_reader = std::io::BufReader::new(stderr);
    let mut stdout_lines = Vec::new();
    let mut started = false;

    loop {
        let mut line = String::new();
        if stdout_reader
            .read_line(&mut line)
            .expect("failed to read line")
            == 0
        {
            break;
        }
        let line = line.trim();
        if started {
            stdout_lines.push(line.to_string());
        } else if line == "e2e_init_ctrlc_with_print started" {
            started = true;
            unsafe {
                libc::kill(child_id as i32, libc::SIGINT);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }

    let mut stderr_content = String::new();
    stderr_reader
        .read_to_string(&mut stderr_content)
        .expect("failed to read stderr");

    assert_eq!(
        stdout_lines,
        vec!["Finished"],
        "Expected finish message in stdout"
    );
    assert!(
        stderr_content.contains("Ctrl+C pressed"),
        "Expected Ctrl+C message in stderr, got: {}",
        stderr_content
    );
}
