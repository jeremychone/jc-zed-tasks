use crate::cli::cmd::{CliCmd, CliSubCmd, CreateGitIgnoreArgs, TmuxRunAipArgs};
use crate::support::tmux;
use crate::Result;
use clap::Parser as _;

pub fn execute() -> Result<()> {
	let cli_cmd = CliCmd::parse();

	match cli_cmd.command {
		CliSubCmd::TmuxRunAip(args) => exec_tmux_run_aip(args)?,
		CliSubCmd::CreateGitIgnore(args) => exec_create_git_ignore(args)?,
	}

	Ok(())
}

// region:    --- Exec Handlers

fn exec_tmux_run_aip(args: TmuxRunAipArgs) -> Result<()> {
	let pane = tmux::find_first_pane(args.dir.as_deref(), args.pane_name.as_deref())?;

	if let Some(pane) = pane {
		tmux::send_keys(&pane.id, "r")?;
	} else {
		let name = args.pane_name.as_deref().unwrap_or("");
		println!("no pane '{name}' found");
	}

	Ok(())
}

fn exec_create_git_ignore(args: CreateGitIgnoreArgs) -> Result<()> {
	println!("create-git-ignore: {path}", path = args.path);

	Ok(())
}

// endregion: --- Exec Handlers
