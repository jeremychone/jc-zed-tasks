use crate::Result;
use std::process::Command;

// -- AppleScript runner helper
pub fn run_applescript(script: &str) -> Result<String> {
	let output = Command::new("osascript")
		.arg("-e")
		.arg(script)
		.output()?;

	if !output.status.success() {
		let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
		return Err(crate::Error::custom(format!("AppleScript failed: {err_msg}")));
	}

	Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
