use clap::{Args, Parser, Subcommand, ValueEnum};

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

	/// Toggle AI in Zed settings (~/.config/zed/settings.json)
	ZedToggleAi,

	/// Open a new Alacritty development terminal
	NewDevTerm(NewDevTermArgs),
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

#[derive(Args, Debug)]
pub struct NewDevTermArgs {
	/// Working directory for the new terminal
	#[arg(long)]
	pub cwd: String,

	/// Start tmux in the new terminal
	#[arg(long)]
	pub with_tmux: bool,

	/// Auto position the terminal relative to Zed
	#[arg(long, value_enum)]
	pub auto_pos: Option<AutoPos>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum AutoPos {
	Below,
	Bottom,
}
