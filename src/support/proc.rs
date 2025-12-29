use crate::Result;
use std::process::Command;

pub fn run_proc(cmd: &str, args: &[&str]) -> Result<String> {
	let output = Command::new(cmd)
		.args(args)
		.output()?;

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
