use crate::Result;
use crate::support::proc::run_proc;
use super::types::PaneId;

pub fn send_keys(pane_id: &PaneId, keys: &str) -> Result<()> {
	run_proc("tmux", &["send-keys", "-t", &pane_id.to_string(), keys])?;
	Ok(())
}
