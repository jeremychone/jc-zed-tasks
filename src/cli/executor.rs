use crate::cli::cmd::{CliCmd, CliSubCmd, CreateGitIgnoreArgs};
use crate::support::tmux;
use crate::Result;
use clap::Parser as _;

pub fn execute() -> Result<()> {
	let cli_cmd = CliCmd::parse();

	match cli_cmd.command {
		CliSubCmd::TmuxRunAip => exec_tmux_run_aip()?,
		CliSubCmd::CreateGitIgnore(args) => exec_create_git_ignore(args)?,
	}

	Ok(())
}

// region:    --- Exec Handlers

fn exec_tmux_run_aip() -> Result<()> {
	let sessions = tmux::list_sessions()?;
	println!("{sessions:#?}");

	Ok(())
}

fn exec_create_git_ignore(args: CreateGitIgnoreArgs) -> Result<()> {
	println!("create-git-ignore: {path}", path = args.path);

	Ok(())
}

// endregion: --- Exec Handlers
