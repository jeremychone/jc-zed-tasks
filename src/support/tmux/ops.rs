use super::types::PaneId;
use crate::Result;
use crate::support::proc::run_proc;

pub fn send_keys(pane_id: &PaneId, keys: &str) -> Result<()> {
	run_proc("tmux", &["send-keys", "-t", &pane_id.to_string(), keys])?;
	Ok(())
}
