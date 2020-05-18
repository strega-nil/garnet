#![allow(unused)]

pub use anyhow::Result;

#[macro_use]
mod macros;

mod localize;

fn main() -> Result<()> {
	let loc = localize::Localize::new()?;

	if loc.args().is_present("cli") {
		locprintln!();
		locprintln!(loc => ("hello-person") { "name" => "Nicole" });
	} else {
		todo!()
	}

	Ok(())
}
