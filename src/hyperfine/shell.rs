use std;
use std::io;
use std::process::{Command, ExitStatus, Stdio};

use hyperfine::timer::get_cpu_timer;

/// Used to indicate the result of running a command
#[derive(Debug, Copy, Clone)]
pub struct ExecuteResult {
    /// The amount of user time the process used
    pub user_time: f64,

    /// The amount of cpu time the process used
    pub system_time: f64,

    /// The exit status of the process
    pub status: ExitStatus,
}

/// Execute the given command and return a timing summary
#[cfg(windows)]
pub fn execute_and_time(stdout: Stdio, stderr: Stdio, command: &str) -> io::Result<ExecuteResult> {
    let mut child = run_shell_command(stdout, stderr, command)?;
    let cpu_timer = get_cpu_timer(&child);
    let status = child.wait()?;

    let (user_time, system_time) = cpu_timer.stop();
    Ok(ExecuteResult {
        user_time,
        system_time,
        status,
    })
}

/// Execute the given command and return a timing summary
#[cfg(not(windows))]
pub fn execute_and_time(stdout: Stdio, stderr: Stdio, command: &str) -> io::Result<ExecuteResult> {
    let cpu_timer = get_cpu_timer();

    let status = run_shell_command(stdout, stderr, command)?;

    let (user_time, system_time) = cpu_timer.stop();

    Ok(ExecuteResult {
        user_time,
        system_time,
        status,
    })
}

/// Run a standard shell command
#[cfg(not(windows))]
fn run_shell_command(
    stdout: Stdio,
    stderr: Stdio,
    command: &str,
) -> io::Result<std::process::ExitStatus> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(stderr)
        .status()
}

/// Run a Windows shell command using cmd.exe
#[cfg(windows)]
fn run_shell_command(
    stdout: Stdio,
    stderr: Stdio,
    command: &str,
) -> io::Result<std::process::Child> {
    Command::new("cmd.exe")
        .arg("/C")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
}
