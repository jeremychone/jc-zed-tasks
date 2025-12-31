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

pub fn run_proc_detach_spawn(cmd: &str, args: &[&str]) -> Result<()> {
	use std::os::unix::process::CommandExt;
	run_proc_daemon(|| {
		// spawn and do not wait
		let err = Command::new(cmd).args(args).spawn();

		match err {
			Ok(_) => Ok(()),
			Err(err) => {
				let msg = format!("Failed to spawn '{cmd}'. Cause: {err}");
				println!("{msg}");
				return Err(msg.into());
			}
		}
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
