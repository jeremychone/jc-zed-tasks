use crate::Result;
use simple_fs::SPath;
use std::fs::{OpenOptions, metadata};

/// Touches the Zed tasks.json file to trigger a reload if it exists.
/// NOTE: Needed because of a Zed bug (2026-01-09) that it does not refresh the current file in the environment variable.
///       The work around is to touch the zed tasks.json, and then, the current file
///       Now, since this binary is called after, we just touch_tasks_json for helping the next call. Not bullet proof, but should help.
pub fn touch_tasks_json() -> Result<()> {
	let home = home::home_dir().ok_or("Could not find home directory")?;
	let tasks_path = SPath::from_std_path(home)?.join(".config/zed/tasks.json");

	// with `filetime` crate (cleanest)
	// if tasks_path.exists() {
	// 	let now = FileTime::now();
	// 	set_file_times(&tasks_path, now, now)?;
	// }

	// A little hacky, but no filetime dep (might change later)
	if tasks_path.exists() {
		let len = metadata(&tasks_path)?.len();
		OpenOptions::new().write(true).open(tasks_path)?.set_len(len)?;
	}

	// With touch command
	// if tasks_path.exists() {
	// 	std::process::Command::new("touch").arg(tasks_path.as_str()).status()?;
	// }

	Ok(())
}
