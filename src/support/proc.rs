use crate::Result;
use std::process::Command;

pub fn run_proc(cmd: &str, args: &[&str]) -> Result<String> {
	let output = Command::new(cmd).args(args).output()?;

	if !output.status.success() {
		let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
		return Err(crate::Error::custom(format!("Command '{cmd}' failed: {err_msg}")));
	}

	Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn is_proc_running(name: &str) -> bool {
	Command::new("pgrep")
		.arg("-x")
		.arg(name)
		.output()
		.map(|o| o.status.success())
		.unwrap_or(false)
}

pub fn run_proc_detach(cmd: &str, args: &[&str]) -> Result<()> {
	use std::os::unix::process::CommandExt;

	run_proc_daemon(|| {
		let mut command = Command::new(cmd);
		command.args(args);

		// exec() replaces the current process image with the new command.
		// If successful, this code is never reached.
		let err = command.exec();

		Err(crate::Error::custom(format!("Failed to exec '{cmd}': {err}")))
	})
}

pub fn run_proc_daemon<F>(f: F) -> Result<()>
where
	F: FnOnce() -> Result<()>,
{
	use daemonize::Daemonize;

	let daemonize = Daemonize::new();

	// daemonize.start() forks the process.
	// The parent process exits here, returning control to the user.
	// The child process continues execution below.
	daemonize.start()?;

	f()
}
