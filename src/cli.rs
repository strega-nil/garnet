use garnet_core::prelude::*;

fn main() -> Result<()> {
  let locale_dir = garnet::get_locale_directory()?;
  let loc = garnet_core::localize::Localize::new(locale_dir.as_deref())?;
  garnet_cli::main(&loc)
}
