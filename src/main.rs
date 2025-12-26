// region:    --- Modules

mod cli;
mod error;
mod support;

pub use error::{Error, Result};

// endregion: --- Modules

fn main() -> Result<()> {
	cli::execute()?;

	Ok(())
}
