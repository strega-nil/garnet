use garnet_core::prelude::*;

use std::path::PathBuf;

#[cfg(not(system_install))]
pub fn get_locale_directory() -> Result<Option<PathBuf>> {
  let mut path = PathBuf::from(env!("OUT_DIR"));
  path.push("locale");

  match std::fs::metadata(&path) {
    Ok(m) => if m.is_dir() {
      Ok(Some(path))
    } else {
      Ok(None) // not a directory
    }
    Err(_) => Ok(None), // this file doesn't exist, or we lack perms
  }
}
