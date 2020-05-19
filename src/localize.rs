use crate::Result;

use fluent::{FluentBundle, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use unic_langid::{langid, LanguageIdentifier};

use std::path::{Path, PathBuf};

static FLUENT_RESOURCES: &[&str] = &["cli.ftl"];
static CLI_LOCATION: &str = "clap.json";

pub struct Localize {
	bundle: FluentBundle<FluentResource>,
	arguments: clap::ArgMatches<'static>,
}

fn read_file(path: &Path) -> Result<String> {
	use std::io::Read;

	let mut result = String::new();
	std::fs::File::open(path)?.read_to_string(&mut result)?;
	Ok(result)
}

impl Localize {
	pub fn new() -> Result<Localize> {
		let default_locale = langid!("en-US");
		let available_locales = get_supported_locales()?;

		let system_locale = get_current_locale()?;

		let resolved_locales = negotiate_languages(
			system_locale
				.as_ref()
				.map(std::slice::from_ref)
				.unwrap_or(&[]),
			&available_locales,
			Some(&default_locale),
			NegotiationStrategy::Filtering,
		);

		let current_locale = *resolved_locales.get(0).unwrap();

		let locale_dir = get_directory_for_locale(current_locale)?;

		let bundle = {
			let mut bundle = FluentBundle::new(resolved_locales.clone());

			for path in FLUENT_RESOURCES.iter() {
				let path = locale_dir.join(*path);
				let source = read_file(&path)?;
				let resource = FluentResource::try_new(source)
					.expect("Failed to parse ftl file");
				bundle
					.add_resource(resource)
					.expect("Adding resource failed");
			}

			bundle
		};

		let arguments = {
			let yaml_file = read_file(&locale_dir.join(CLI_LOCATION))?;
			let mut yaml = clap::YamlLoader::load_from_str(&yaml_file)?;
			if yaml.len() != 1 {
				Err(anyhow::Error::msg("Invalid yaml from CLI file; should be exactly one document"))?;
			}
			// I wish we didn't have to leak this...
			let yaml = Box::leak(Box::new(yaml.pop().unwrap()));
			clap::App::from_yaml(yaml).get_matches()
		};

		Ok(Localize {
			bundle,
			arguments,
		})
	}

	pub fn bundle(&self) -> &FluentBundle<FluentResource> {
		&self.bundle
	}

	pub fn args(&self) -> &clap::ArgMatches<'_> {
		&self.arguments
	}
}

fn get_directory_for_locale(l: &LanguageIdentifier) -> Result<PathBuf> {
	let mut p = get_locale_directory()?;
	p.push(l.to_string());
	p.push("garnet");
	Ok(p)
}

#[cfg(not(system_install))]
fn get_locale_directory() -> Result<PathBuf> {
	let mut path = PathBuf::from(env!("OUT_DIR"));
	path.push("locale");
	Ok(path)
}

// TODO: this should be an Option<LanguageIdentifier>, but laaaazy
#[cfg(windows)]
fn get_current_locale() -> Result<Option<LanguageIdentifier>> {
	use winapi::um::{
		stringapiset::WideCharToMultiByte,
		winnls::{GetUserDefaultLocaleName, CP_UTF8, WC_ERR_INVALID_CHARS},
		winnt::LOCALE_NAME_MAX_LENGTH,
  };

  if let Ok(id) = std::env::var("LC_MESSAGES") {
    if !id.is_empty() {
      return id.parse().map(Some).map_err(From::from);
    }
  }

	unsafe {
		let mut buffer = [0u16; LOCALE_NAME_MAX_LENGTH];
		let len = GetUserDefaultLocaleName(
			buffer.as_mut_ptr(),
			LOCALE_NAME_MAX_LENGTH as i32,
		);

		if len == 0 {
			return Ok(None);
		}

		// * 3 is the maximum amount of extra code units you could need
		let mut u8buffer = [0u8; LOCALE_NAME_MAX_LENGTH * 3];
		let u8len = WideCharToMultiByte(
			CP_UTF8,
			WC_ERR_INVALID_CHARS,
			buffer.as_ptr(),
			len,
			u8buffer.as_mut_ptr() as *mut i8,
			u8buffer.len() as i32,
			std::ptr::null(),
			std::ptr::null_mut(),
		);

		assert!(u8len > 0);

		let name =
			std::str::from_utf8_unchecked(std::slice::from_raw_parts(
				u8buffer.as_ptr(),
				(u8len - 1) as usize,
			));

		Ok(Some(name.parse()?))
	}
}

fn get_supported_locales() -> Result<Vec<LanguageIdentifier>> {
	use std::{fs::DirEntry, io};

	let mut result = Vec::new();

	fn locale_of_path(
		p: io::Result<DirEntry>,
	) -> Option<LanguageIdentifier> {
		let lang_path = p.ok()?;
		if lang_path.file_type().ok()?.is_dir() {
			let langid: LanguageIdentifier =
				lang_path.file_name().to_str()?.parse().ok()?;
			Some(langid)
		} else {
			None
		}
	}

	let dir = get_locale_directory()?;

	for lang_path in dir.read_dir()? {
		if let Some(l) = locale_of_path(lang_path) {
			result.push(l);
		}
	}

	Ok(result)
}
