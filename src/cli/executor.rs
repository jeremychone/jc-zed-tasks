use crate::Result;
use crate::cli::cmd::{CliCmd, CliSubCmd, CreateGitIgnoreArgs, TmuxRunAipArgs};
use crate::support::tmux;
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
	let pane_name = args.dir.as_deref().ok_or("tmux_run_aip must have a --pane")?;
	let dir = args.dir.as_deref().ok_or("tmux_run_aip must have a --dir")?;

	let pane = tmux::find_first_pane(Some(dir), Some(pane_name))?;

	let pane = pane.ok_or(format!("no pane '{pane_name}' found running at '{dir}'"))?;

	tmux::send_keys(&pane.id, "r")?;

	Ok(())
}

fn exec_create_git_ignore(args: CreateGitIgnoreArgs) -> Result<()> {
	println!("create-git-ignore: {path}", path = args.path);

	Ok(())
}

// endregion: --- Exec Handlers
