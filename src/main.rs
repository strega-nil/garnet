#![allow(unused)]

pub use anyhow::Result;

#[macro_use]
mod macros;

mod cli;
mod gemini;
mod localize;

use localize::Localize;

fn main() -> Result<()> {
	let loc = localize::Localize::new()?;

	if loc.args().is_present("cli") {
		cli::main(&loc)
	} else {
		todo!()
	}
}
