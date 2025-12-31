use crate::Result;
use crate::support::mac::support::run_applescript;
use crate::support::mac::types::*;
use std::sync::Arc;

pub const APP_NAME_ZED: &str = "Zed";

/// Get the bounds of the frontmost (active) window for the given application.
pub fn get_front_window_bounds(app_name: &str) -> Result<WindowBounds> {
	let _win = get_front_window(app_name)?.ok_or_else(|| format!("No window found for application: {app_name}"))?;

	let script = format!(
		r#"tell application "System Events"
			tell process "{app_name}"
				get {{position, size}} of window 1
			end tell
		end tell"#
	);

	let output = run_applescript(&script)?;
	// Output format: "x, y, width, height"
	let parts: Vec<i32> = output.split(',').filter_map(|s| s.trim().parse().ok()).collect();

	if parts.len() != 4 {
		return Err(format!("Unexpected bounds format from AppleScript: {output}").into());
	}

	Ok(WindowBounds {
		x: parts[0],
		y: parts[1],
		width: parts[2],
		height: parts[3],
	})
}

/// Set the bounds of the frontmost (active) window for the given application.
pub fn set_front_window_bounds(app_name: &str, bounds: WindowBounds) -> Result<()> {
	let _win = get_front_window(app_name)?.ok_or_else(|| format!("No window found for application: {app_name}"))?;

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

/// Get all window information for a given application using System Events.
pub fn get_app_windows(app_name: &str) -> Result<Vec<AppWindow>> {
	let script = format!(
		r#"tell application "System Events"
			tell process "{app_name}"
				get name of windows
			end tell
		end tell"#
	);

	let output = run_applescript(&script)?;
	if output.is_empty() {
		return Ok(vec![]);
	}

	let windows = output
		.split(',')
		.enumerate()
		.map(|(i, name)| AppWindow {
			app: Arc::from(app_name),
			win_idx: (i + 1) as i32,
			win_name: name.trim().to_string(),
		})
		.collect();

	Ok(windows)
}

/// Get the frontmost (active) window for the given application.
pub fn get_front_window(app_name: &str) -> Result<Option<AppWindow>> {
	let windows = get_app_windows(app_name)?;
	Ok(windows.into_iter().next())
}

/// Get the names of all currently running application processes.
/// Filters for applications that are not background-only to help identify valid UI targets.
pub fn get_all_app_names() -> Result<Vec<String>> {
	let script = r#"tell application "System Events"
		get name of every application process whose background only is false
	end tell"#;

	let output = run_applescript(script)?;
	if output.is_empty() {
		return Ok(vec![]);
	}

	let names: Vec<String> = output.split(',').map(|s| s.trim().to_string()).collect();

	Ok(names)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

	use super::*;

	#[test]
	fn test_support_mac_common_get_front_window_zed() -> Result<()> {
		// -- Exec
		let res = get_front_window(APP_NAME_ZED);

		// -- Check
		match res {
			Ok(Some(window)) => {
				println!("Zed front window: {window:?}");
			}
			Ok(None) => {
				println!("No Zed window found.");
			}
			Err(err) => {
				let msg = err.to_string();
				if msg.contains("Application isn") || msg.contains("not found") {
					println!("Skipping check because Zed is not running: {msg}");
				} else {
					return Err(err.into());
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_support_mac_common_get_app_windows_zed() -> Result<()> {
		// -- Exec
		let res = get_app_windows(APP_NAME_ZED);

		// -- Check
		match res {
			Ok(windows) => {
				println!("Zed windows: {windows:?}");
			}
			Err(err) => {
				let msg = err.to_string();
				if msg.contains("Application isn")
					|| msg.contains("not found")
					|| msg.contains("invalid connection")
					|| msg.contains("Can’t get name of windows")
				{
					println!("Skipping check because Zed is not accessible or has no windows: {msg}");
				} else {
					return Err(err.into());
				}
			}
		}

		Ok(())
	}

	#[test]
	fn test_support_mac_common_get_all_app_names_simple() -> Result<()> {
		// -- Exec
		let names = get_all_app_names()?;

		// -- Nice Print
		let names = names.join("\n");
		println!("Running application names: \n{names}");

		// -- Check
		assert!(!names.is_empty(), "Should have at least some apps running");

		Ok(())
	}

	#[test]
	fn test_support_mac_common_get_front_window_bounds_zed() -> Result<()> {
		// -- Exec
		let res = get_front_window_bounds(APP_NAME_ZED);

		// -- Check
		match res {
			Ok(bounds) => {
				println!("Zed front window bounds: {bounds:?}");
			}
			Err(err) => {
				let msg = err.to_string();
				if msg.contains("Application isn")
					|| msg.contains("not found")
					|| msg.contains("invalid connection")
					|| msg.contains("Can’t get bounds of window 1")
				{
					println!("Skipping check because Zed is not accessible or has no windows: {msg}");
				} else {
					return Err(err.into());
				}
			}
		}

		Ok(())
	}
}

// endregion: --- Tests
