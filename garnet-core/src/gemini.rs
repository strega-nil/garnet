use crate::prelude::*;

use std::io::{Read, Write};
use std::net::TcpStream;

use native_tls::TlsConnector;

pub fn get_page(mut url: &str) -> Result<String> {
	let mut builder = TlsConnector::builder();
	builder.danger_accept_invalid_hostnames(true);
	builder.danger_accept_invalid_certs(true);
	let connector = builder.build().unwrap();

	if url.starts_with("gemini://") {
		url = &url[9..];
	}

	let slash_index = url.find('/');

	let (url, location) = if let Some(slash_index) = slash_index {
		(&url[..slash_index], &url[slash_index..])
	} else {
		(url, "/")
	};

	let stream = TcpStream::connect(format!("{}:1965", url))?;
	let mut stream = connector.connect(url, stream)?;

	stream.write_fmt(format_args!("gemini://{}{}\r\n", url, location))?;
	let mut res = vec![];
	stream.read_to_end(&mut res)?;
	Ok(String::from_utf8_lossy(&res).to_string())
}
