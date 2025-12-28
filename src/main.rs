// region:    --- Modules

mod cli;
mod derive_aliases;
mod error;
mod support;

use derive_aliases::*;
pub use error::{Error, Result};
use std::process;

// endregion: --- Modules

fn main() {
	match cli::execute() {
		Ok(_) => (),
		Err(err) => {
			eprintln!("{err}");
			process::exit(1);
		}
	}
}
