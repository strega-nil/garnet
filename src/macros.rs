macro_rules! log {
  ($msg:expr) => {
    if cfg!(debug_assertions) {
      eprintln!(concat!("Log: ", $msg));
    }
  };
  ($msg:expr, $($val:expr),*) => {
    if cfg!(debug_assertions) {
      eprintln!(concat!("Log: ", $msg), $($val,)*);
    }
  }
}

macro_rules! locformat_to {
  ($out:expr) => {
    <_ as std::io::Write>::write(&mut $out, b"\n")
  };
	($out:expr, $loc:expr => $msg:expr) => {
    locformat_to!($out, $loc => ($msg) { })
  };
	($out:expr, $loc:expr => ($msg:expr) { $( $name:expr => $value:expr ),* }) => {{
    let mut args = ::fluent::FluentArgs::new();
    $(args.insert($name, ::fluent::FluentValue::from($value));)*

		let mut errors = vec![];
    let msg = $loc
      .bundle()
			.get_message($msg)
			.expect("Message doesn't exist.");
		let pattern = msg.value.expect("Message has no value.");
		let value =
      $loc.bundle().format_pattern(&pattern, Some(&args), &mut errors);
    assert!(errors.is_empty());

    <_ as std::io::Write>::write(&mut $out, value.as_bytes())
  }};
}

macro_rules! locprintln {
	() => { locformat_to!(::std::io::stdout().lock()).expect("writing message failed") };
	($out:expr, $loc:expr => $msg:expr) => {
    locprintln!($out, $loc => ($msg) { })
  };
	($loc:expr => ($msg:expr) { $( $name:expr => $value:expr ),* }) => {
    locformat_to!(::std::io::stdout().lock(), $loc => ($msg) { $( $name => $value ),* }).expect("writing message failed");
  };
}

macro_rules! loceprintln {
	() => { locformat_to!(::std::io::stderr().lock()).expect("writing message failed") };
	($out:expr, $loc:expr => $msg:expr) => {
    loceprintln!($out, $loc => ($msg) { })
  };
	($loc:expr => ($msg:expr) { $( $name:expr => $value:expr ),* }) => {
    locformat_to!(std::io::stderr().lock(), $loc => ($msg) { $( $name => $value ),* }).expect("writing message failed");
  };
}
