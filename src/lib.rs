use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};

pub struct ProgramMonitor {
    program: String,
    args: Vec<String>,
}

impl ProgramMonitor {
    pub fn new(program: String, args: Vec<String>) -> Self {
        Self { program, args }
    }

    pub fn monitor_program(&self) -> Result<(), String> {
        loop {
            let mut command = Command::new(&self.program);
            command
                .args(&self.args)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            // Start the program
            let mut child = match command.spawn() {
                Ok(child) => child,
                Err(e) => {
                    return Err(format!("Failed to start {}: {}", &self.program, e));
                }
            };

            let exit_status = match child.wait() {
                Ok(status) => status,
                Err(e) => {
                    return Err(format!("Failed to wait for process: {}", e));
                }
            };

            if exit_status.success() {
                // Normal exit with status 0
                return Ok(());
            } else {
                // Handle non-zero exit status
                if let Some(code) = exit_status.code() {
                    eprintln!("{} exited with status {}", &self.program, code);
                } else if let Some(signal) = exit_status.signal() {
                    eprintln!("{} terminated by signal {}", &self.program, signal);
                    // Note: Rust does not provide direct core dump detection as in C++
                    // but you can infer it from the signal if necessary.
                }

                println!("Restarting {}", &self.program);
            }
        }
    }
}
