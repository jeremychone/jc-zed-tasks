// region:    --- Modules

mod cli;
mod derive_aliases;
mod error;
mod support;

use derive_aliases::*;
pub use error::{Error, Result};

// endregion: --- Modules

fn main() -> Result<()> {
	cli::execute()?;

	Ok(())
}
