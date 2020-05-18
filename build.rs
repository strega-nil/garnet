use anyhow::Result;
use std::path::{Path, PathBuf};

fn rerun_if_changed(path: &Path) -> Result<()> {
	if path.is_dir() {
		let mut dirs_to_track = vec![path.to_owned()];
		while let Some(dir) = dirs_to_track.pop() {
			println!("cargo:rerun-if-changed={}", dir.display());

			for item in dir.read_dir()? {
				let item = item?;
				if item.file_type()?.is_dir() {
					dirs_to_track.push(item.path());
				} else {
					// we choose not to deal with non-utf8 paths in locale/
					println!("cargo:rerun-if-changed={}", item.path().display());
				}
			}
		}
	} else {
		println!("cargo:rerun-if-changed={}", path.display());
	}

	Ok(())
}

// overrides the destination
fn fs_copy_recursive(src: &Path, dst: &Path) -> Result<()> {
	let mut dirs = vec![];

	/*
		all of these file accesses are racey;
		we assume that someone who is building this won't make edits to
		`target/` at the same time
	*/
	if dst.exists() {
		std::fs::remove_dir_all(dst)?;
	}

	if src.symlink_metadata()?.is_dir() {
		dirs.push((src.to_owned(), dst.to_owned()));
	} else {
		let _ = std::fs::copy(src, dst)?;
	}

	while let Some((src, dst)) = dirs.pop() {
		std::fs::create_dir(&dst)?;
		for item in src.read_dir()? {
			let item = item?;

			if item.file_type()?.is_dir() {
				dirs.push((item.path(), dst.join(item.file_name())))
			} else {
				let _ = std::fs::copy(item.path(), dst.join(item.file_name()))?;
			}
		}
	}

	Ok(())
}

fn main() -> Result<()> {
	let mut locale_src = std::env::current_dir()?;
	locale_src.push("locale");
	rerun_if_changed(&locale_src)?;

	let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
	let locale_dst = out_dir.join("locale");

	fs_copy_recursive(&locale_src, &locale_dst)?;

	Ok(())
}
