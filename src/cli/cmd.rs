use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliCmd {
	#[command(subcommand)]
	pub command: CliSubCmd,
}

#[derive(Subcommand, Debug)]
pub enum CliSubCmd {
	/// Run AIP in a tmux session
	TmuxRunAip(TmuxRunAipArgs),

	/// Create a .gitignore file at the specified path
	CreateGitIgnore(CreateGitIgnoreArgs),
}

#[derive(Args, Debug)]
pub struct TmuxRunAipArgs {
	/// Filter by pane directory
	#[arg(long)]
	pub dir: Option<String>,

	/// Filter by pane name (title)
	#[arg(long)]
	pub pane: Option<String>,
}

#[derive(Args, Debug)]
pub struct CreateGitIgnoreArgs {
	/// The path where the .gitignore should be created
	pub path: String,
}
