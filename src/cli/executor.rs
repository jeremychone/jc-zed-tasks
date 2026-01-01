use crate::Result;
use crate::cli::cmd::{AutoPos, CliCmd, CliSubCmd, NewDevTermArgs, TmuxRunAipArgs};
use crate::support::mac::{self, APP_NAME_ALACRITTY, APP_NAME_ZED, WindowBounds};
use crate::support::{jsons, os, proc, tmux};
use clap::Parser as _;
use simple_fs::{SPath, read_to_string};
use std::time::Duration;
use std::{env, fs, process, thread};

pub fn execute() -> Result<()> {
	let cli_cmd = CliCmd::parse();

	match cli_cmd.command {
		CliSubCmd::TmuxRunAip(args) => exec_tmux_run_aip(args)?,
		CliSubCmd::ZedToggleAi => exec_zed_toggle_ai()?,
		CliSubCmd::NewDevTerm(args) => exec_new_dev_term(args)?,
	}

	Ok(())
}

const ALACRITTY_BIN: &str = "/Applications/Alacritty.app/Contents/MacOS/alacritty";

// region:    --- Exec Handlers

fn exec_tmux_run_aip(args: TmuxRunAipArgs) -> Result<()> {
	let dir_str = args.dir.as_deref().ok_or("tmux_run_aip must have a --dir")?;
	let dir = SPath::new(dir_str);

	let pane_id = if let Some(pane_name) = args.pane.as_deref() {
		let pane = tmux::find_first_pane(Some(&dir), Some(pane_name))?;
		let pane = pane.ok_or(format!("no pane '{pane_name}' found running at '{dir}'"))?;
		pane.id
	} else {
		let sessions = tmux::list_sessions()?;

		// -- Find the best window
		let window = sessions
			.into_iter()
			.filter(|s| s.attached)
			.flat_map(|s| s.windows)
			.find(|w| w.active && w.panes.iter().any(|p| p.path == dir))
			.or_else(|| {
				// Fallback to any window in any session that has a pane in this dir
				tmux::list_sessions()
					.ok()?
					.into_iter()
					.flat_map(|s| s.windows)
					.find(|w| w.panes.iter().any(|p| p.path == dir))
			})
			.ok_or(format!("No window found for directory '{dir}'"))?;

		// -- Find aip pane in this window
		let aip_pane = window
			.panes
			.iter()
			.find(|p| p.active && p.command == "aip")
			.or_else(|| window.panes.iter().find(|p| p.command == "aip"))
			.ok_or(format!("No pane running 'aip' found in the active window for '{dir}'"))?;

		aip_pane.id.clone()
	};

	tmux::send_keys(&pane_id, "r")?;

	Ok(())
}

fn exec_new_dev_term(args: NewDevTermArgs) -> Result<()> {
	let cwd = SPath::new(args.cwd);
	let cwd = if cwd.is_relative() {
		SPath::from_std_path(env::current_dir()?)?.join(cwd)
	} else {
		cwd
	};

	let mut proc_args: Vec<String> = if crate::support::proc::is_proc_running("alacritty") {
		vec![
			"msg".to_string(),
			"create-window".to_string(),
			"--working-directory".to_string(),
			cwd.to_string(),
		]
	} else {
		vec!["--working-directory".to_string(), cwd.to_string()]
	};

	if args.with_tmux {
		proc_args.extend(["-e".to_string(), "tmux".to_string(), "new-session".to_string()]);
	}

	// -- Get Zed bounds (only if pos is requested)
	let bound_and_pos = if let Some(auto_pos) = args.pos {
		if os::is_mac() {
			let bounds = mac::get_front_window_bounds(APP_NAME_ZED);
			if let Err(ref err) = bounds {
				eprintln!("Warning: Could not get Zed bounds: {err}");
			}
			bounds.map(|zb| (zb, auto_pos)).ok()
		} else {
			eprintln!("Warning: Window positioning is only supported on macOS.");
			None
		}
	} else {
		None
	};

	// -- Detach and run
	let proc_args: Vec<&str> = proc_args.iter().map(|s| s.as_str()).collect();
	proc::run_proc_detach_spawn(ALACRITTY_BIN, &proc_args)?;

	// NOTE: Because the way proc_detach use deamon, here it won't print in the zed window

	if let Some((zb, auto_pos)) = bound_and_pos {
		// Wait for window to be created/focused
		thread::sleep(Duration::from_millis(50));

		// Get Alacritty bounds to calculate relative position
		let ab = mac::get_front_window_bounds(APP_NAME_ALACRITTY)?;

		// Calculate relative position (centered horizontally)
		let ax = zb.x + (zb.width - ab.width) / 2;
		let ay = match auto_pos {
			AutoPos::Below => zb.y + zb.height + 4,
			AutoPos::Bottom => zb.y + zb.height - ab.height,
		};

		mac::set_front_window_xy(APP_NAME_ALACRITTY, ax, ay)?;
	} else {
		if args.pos.is_some() {
			println!("Zed bounds not found, running detached.");
		}
	}

	Ok(())
}

fn exec_zed_toggle_ai() -> Result<()> {
	let home = env::var("HOME").map_err(|_| "HOME environment variable not set")?;
	let settings_path = SPath::new(home).join(".config/zed/settings.json");

	if !settings_path.exists() {
		return Err(crate::Error::custom(format!(
			"Zed settings file not found at: {settings_path}"
		)));
	}

	let content = simple_fs::read_to_string(&settings_path)?;
	let new_content = jsons::toggle_bool_text_mode(&content, &["disable_ai"])?;

	fs::write(settings_path.std_path(), new_content)?;

	println!("Zed AI toggled.");

	Ok(())
}

// endregion: --- Exec Handlers
