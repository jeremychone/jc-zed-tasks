use crate::support::proc::run_proc;
use crate::Result;

// region:    --- Data Structures

#[derive(Debug, Clone)]
pub struct TmuxPane {
	pub index: usize,
	pub title: String,
	pub path: String,
	pub command: String,
}

#[derive(Debug, Clone)]
pub struct TmuxWindow {
	pub index: usize,
	pub name: String,
	pub panes: Vec<TmuxPane>,
}

#[derive(Debug, Clone)]
pub struct TmuxSession {
	pub name: String,
	pub attached: bool,
	pub windows: Vec<TmuxWindow>,
}

#[derive(Debug, Clone)]
pub struct TmuxSessions(pub Vec<TmuxSession>);

impl TmuxSessions {
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl IntoIterator for TmuxSessions {
	type Item = TmuxSession;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// endregion: --- Data Structures

pub fn list_sessions() -> Result<TmuxSessions> {
	let format = "#{?session_attached,ATTACHED,DETACHED} #S:#I.#P #{window_name} [#{pane_title}] #{pane_current_path} #{pane_current_command}";

	let output = match run_proc("tmux", &["list-panes", "-a", "-F", format]) {
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
		let Some((s_name, attached, w_idx, p_idx, w_name, p_title, path, cmd)) = parse_line(line) else {
			continue;
		};

		// 1. Get or create session
		let session = if let Some(s) = sessions.iter_mut().find(|s| s.name == s_name) {
			s
		} else {
			sessions.push(TmuxSession {
				name: s_name.to_string(),
				attached,
				windows: Vec::new(),
			});
			sessions.last_mut().unwrap()
		};

		// 2. Get or create window
		let window = if let Some(w) = session.windows.iter_mut().find(|w| w.index == w_idx) {
			w
		} else {
			session.windows.push(TmuxWindow {
				index: w_idx,
				name: w_name.to_string(),
				panes: Vec::new(),
			});
			session.windows.last_mut().unwrap()
		};

		// 3. Add pane
		window.panes.push(TmuxPane {
			index: p_idx,
			title: p_title.to_string(),
			path: path.to_string(),
			command: cmd.to_string(),
		});
	}

	Ok(TmuxSessions(sessions))
}

// region:    --- Support

fn parse_line(line: &str) -> Option<(String, bool, usize, usize, String, String, String, String)> {
	// Format: ATTACHED session:win_idx.pane_idx win_name [pane_title] path command
	let mut parts = line.splitn(3, ' ');
	let attached_str = parts.next()?;
	let session_full = parts.next()?;
	let rest = parts.next()?;

	let attached = attached_str == "ATTACHED";

	let (s_name, win_pane) = session_full.split_once(':')?;
	let (w_idx_str, p_idx_str) = win_pane.split_once('.')?;
	let w_idx = w_idx_str.parse().ok()?;
	let p_idx = p_idx_str.parse().ok()?;

	// rest: win_name [pane_title] path cmd
	let open_bracket = rest.rfind(" [")?;
	let close_bracket = rest.rfind(']')?;

	let w_name = rest[..open_bracket].trim().to_string();
	let p_title = rest[open_bracket + 2..close_bracket].to_string();
	let path_cmd = rest[close_bracket + 1..].trim();

	let (path, cmd) = path_cmd.rsplit_once(' ').unwrap_or((path_cmd, ""));

	Some((
		s_name.to_string(),
		attached,
		w_idx,
		p_idx,
		w_name,
		p_title,
		path.to_string(),
		cmd.to_string(),
	))
}

// endregion: --- Support
