use crate::Result;
use crate::support::mac::support::run_applescript;

#[derive(Debug, Clone, Copy)]
pub struct WindowBounds {
	pub x: i32,
	pub y: i32,
	pub width: i32,
	pub height: i32,
}

/// Get the bounds of the frontmost (active) window for the given application.
pub fn get_front_window_bounds(app_name: &str) -> Result<WindowBounds> {
	let script = format!(
		r#"tell application "{app_name}"
			get bounds of window 1
		end tell"#
	);

	let output = run_applescript(&script)?;
	// Output format: "x1, y1, x2, y2"
	let parts: Vec<i32> = output
		.split(',')
		.filter_map(|s| s.trim().parse().ok())
		.collect();

	if parts.len() != 4 {
		return Err(crate::Error::custom(format!(
			"Unexpected bounds format from AppleScript: {output}"
		)));
	}

	Ok(WindowBounds {
		x: parts[0],
		y: parts[1],
		width: parts[2] - parts[0],
		height: parts[3] - parts[1],
	})
}

/// Set the bounds of the frontmost (active) window for the given application.
pub fn set_front_window_bounds(app_name: &str, bounds: WindowBounds) -> Result<()> {
	let x2 = bounds.x + bounds.width;
	let y2 = bounds.y + bounds.height;
	let script = format!(
		r#"tell application "{app_name}"
			set bounds of window 1 to {{{}, {}, {}, {}}}
		end tell"#,
		bounds.x, bounds.y, x2, y2
	);

	run_applescript(&script)?;

	Ok(())
}

/// Get the names of all windows for the Zed application as a single string.
pub fn get_zed_windows_names() -> Result<String> {
	let script = r#"tell application "Zed"
			get name of every window
		end tell"#;

	run_applescript(script)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

	use super::*;

	#[test]
	fn test_support_mac_common_get_zed_windows_names_simple() -> Result<()> {
		// -- Exec
		let res = get_zed_windows_names();

		// -- Check
		match res {
			Ok(names) => {
				println!("Zed windows names: {names}");
			}
			Err(err) => {
				let msg = err.to_string();
				// Note: Since this requires Zed and macOS, we handle common errors gracefully in tests.
				if msg.contains("Application isn") || msg.contains("not found") || msg.contains("invalid connection") {
					println!("Skipping check because Zed is not accessible: {msg}");
				} else {
					return Err(err.into());
				}
			}
		}

		Ok(())
	}
}

// endregion: --- Tests
