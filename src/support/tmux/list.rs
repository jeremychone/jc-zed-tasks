use crate::Result;
use crate::support::proc::run_proc;
use crate::support::tmux::types::*;

const TMUX_LIST_FORMAT: &str = "#{?session_attached,ATTACHED,DETACHED} #S:#I.#P #{window_name} [#{pane_title}] #{pane_current_path} #{pane_current_command} #{session_id} #{window_id} #{pane_id}";

pub fn list_sessions() -> Result<TmuxSessions> {
	let output = match run_proc("tmux", &["list-panes", "-a", "-F", TMUX_LIST_FORMAT]) {
		Ok(out) => out,
		Err(e) => {
			let err_str = e.to_string();
			if err_str.contains("no server running") || err_str.contains("failed to connect to server") {
				return Ok(TmuxSessions(vec![]));
			}
			return Err(e);
		}
	};

	let mut sessions: Vec<TmuxSession> = Vec::new();

	for line in output.lines() {
		let Some(parts) = parse_line(line) else {
			continue;
		};

		// 1. Get or create session
		let session = if let Some(s) = sessions.iter_mut().find(|s| s.id == parts.s_id) {
			s
		} else {
			sessions.push(TmuxSession {
				id: parts.s_id.clone(),
				name: parts.s_name.clone(),
				attached: parts.attached,
				windows: Vec::new(),
			});
			sessions.last_mut().unwrap()
		};

		// 2. Get or create window
		let window = if let Some(w) = session.windows.iter_mut().find(|w| w.id == parts.w_id) {
			w
		} else {
			session.windows.push(TmuxWindow {
				id: parts.w_id.clone(),
				index: parts.w_idx,
				name: parts.w_name.clone(),
				panes: Vec::new(),
			});
			session.windows.last_mut().unwrap()
		};

		// 3. Add pane
		window.panes.push(TmuxPane {
			id: parts.p_id,
			session_id: parts.s_id,
			window_id: parts.w_id,
			index: parts.p_idx,
			title: parts.p_title,
			path: parts.path,
			command: parts.cmd,
		});
	}

	Ok(TmuxSessions(sessions))
}

pub fn list_panes(folder: Option<&str>, pane_name: Option<&str>) -> Result<Vec<TmuxPane>> {
	let output = match run_proc("tmux", &["list-panes", "-a", "-F", TMUX_LIST_FORMAT]) {
		Ok(out) => out,
		Err(e) => {
			let err_str = e.to_string();
			if err_str.contains("no server running") || err_str.contains("failed to connect to server") {
				return Ok(vec![]);
			}
			return Err(e);
		}
	};

	let mut panes = Vec::new();

	for line in output.lines() {
		if let Some(parts) = parse_line(line) {
			let folder_match = folder.map(|f| parts.path == f).unwrap_or(true);
			let title_match = pane_name.map(|name| parts.p_title == name).unwrap_or(true);

			if folder_match && title_match {
				panes.push(TmuxPane {
					id: parts.p_id,
					session_id: parts.s_id,
					window_id: parts.w_id,
					index: parts.p_idx,
					title: parts.p_title,
					path: parts.path,
					command: parts.cmd,
				});
			}
		}
	}

	Ok(panes)
}

// region:    --- Support

struct LineParts {
	s_name: String,
	attached: bool,
	s_id: SessionId,
	w_idx: usize,
	w_id: WindowId,
	w_name: String,
	p_idx: usize,
	p_id: PaneId,
	p_title: String,
	path: String,
	cmd: String,
}

fn parse_line(line: &str) -> Option<LineParts> {
	// Format: ATTACHED/DETACHED session_name:win_idx.pane_idx win_name [pane_title] path command session_id window_id pane_id
	let mut parts = line.splitn(3, ' ');
	let attached_str = parts.next()?;
	let session_full = parts.next()?;
	let rest = parts.next()?;

	let attached = attached_str == "ATTACHED";

	let (s_name, win_pane) = session_full.split_once(':')?;
	let (w_idx_str, p_idx_str) = win_pane.split_once('.')?;
	let w_idx = w_idx_str.parse().ok()?;
	let p_idx = p_idx_str.parse().ok()?;

	// rest: win_name [pane_title] path command session_id window_id pane_id
	let open_bracket = rest.rfind(" [")?;
	let close_bracket = rest.rfind(']')?;

	let w_name = rest[..open_bracket].trim().to_string();
	let p_title = rest[open_bracket + 2..close_bracket].to_string();

	let tail = rest[close_bracket + 1..].trim();
	// tail: path command session_id window_id pane_id
	let (tail_path_cmd_s_w, p_id_str) = tail.rsplit_once(' ')?;
	let (tail_path_cmd_s, w_id_str) = tail_path_cmd_s_w.rsplit_once(' ')?;
	let (path_cmd, s_id_str) = tail_path_cmd_s.rsplit_once(' ')?;
	let (path, cmd) = path_cmd.rsplit_once(' ').unwrap_or((path_cmd, ""));

	Some(LineParts {
		s_name: s_name.to_string(),
		attached,
		s_id: SessionId::from(std::sync::Arc::from(s_id_str)),
		w_idx,
		w_id: WindowId::from(std::sync::Arc::from(w_id_str)),
		w_name,
		p_idx,
		p_id: PaneId::from(std::sync::Arc::from(p_id_str)),
		p_title,
		path: path.to_string(),
		cmd: cmd.to_string(),
	})
}

// endregion: --- Support
