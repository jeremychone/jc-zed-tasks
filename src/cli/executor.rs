use crate::Result;
use crate::cli::cmd::{CliCmd, CliSubCmd, MdToHtmlArgs, SaveClipboardImageArgs, TmuxRunAipArgs};
use crate::cli::exec_toggle;
use crate::support::{clipboard, jsons, tmux, zed};
use clap::Parser as _;
use lazy_regex::regex;
use simple_fs::{SPath, list_files, read_to_string};
use std::fs;

pub fn execute() -> Result<()> {
	let cli_cmd = CliCmd::parse();

	match cli_cmd.command {
		CliSubCmd::TmuxRunAip(args) => exec_tmux_run_aip(args)?,
		CliSubCmd::ZedToggleAi => exec_zed_toggle_ai()?,
		CliSubCmd::ToggleProfile(args) => exec_toggle::exec_command(args)?,
		CliSubCmd::SaveClipboardImage(args) => exec_save_clipboard_image(args)?,
		CliSubCmd::MdToHtml(args) => exec_md_to_html(args)?,
	}

	Ok(())
}

// region:    --- Exec Handlers

fn exec_save_clipboard_image(args: SaveClipboardImageArgs) -> Result<()> {
	zed::touch_tasks_json()?;

	let dir = SPath::new(args.dir);
	if !dir.exists() {
		return Err(format!("Directory does not exist: {dir}").into());
	}

	let re = regex!(r"^image-(\d+)\.png$");
	let mut max_idx = 0;

	// -- Find max index
	let files = list_files(&dir, Some(&["image-*.png"]), None)?;
	for file in files {
		if let Some(caps) = re.captures(file.name())
			&& let Ok(idx) = caps[1].parse::<u32>()
			&& idx > max_idx
		{
			max_idx = idx;
		}
	}

	// -- Save image
	let next_idx = max_idx + 1;
	let file_name = format!("image-{:02}.png", next_idx);
	let dest_path = dir.join(&file_name);

	clipboard::save_to_png_image(&dest_path)?;

	if args.copy_md_ref {
		let md_ref = format!("![IMAGE]({file_name})");
		clipboard::set_text(md_ref)?;
		println!("Markdown reference copied to clipboard: {file_name}");
	}

	println!("Image saved to: {dest_path}");

	Ok(())
}

fn exec_md_to_html(args: MdToHtmlArgs) -> Result<()> {
	zed::touch_tasks_json()?;

	let md_path = SPath::new(args.file);
	let content = read_to_string(&md_path)?;

	let mut options = pulldown_cmark::Options::empty();
	options.insert(pulldown_cmark::Options::ENABLE_TABLES);
	options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
	options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
	options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
	options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);

	let parser = pulldown_cmark::Parser::new_ext(&content, options);

	let mut html_output = String::new();
	pulldown_cmark::html::push_html(&mut html_output, parser);

	let html_path = md_path.ensure_extension("html");
	fs::write(html_path.std_path(), html_output)?;

	println!("Converted {md_path} to {html_path}");

	Ok(())
}

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

fn exec_zed_toggle_ai() -> Result<()> {
	let home = home::home_dir().ok_or("Could not find home directory")?;
	let settings_path = SPath::from_std_path(home)?.join(".config/zed/settings.json");

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
