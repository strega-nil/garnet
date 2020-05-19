use crate::{
  localize::Localize,
  Result,
};

enum Input {
  Help,
  Quit,
  Goto(String),
  Unknown(String),
}

fn get_input(loc: &Localize) -> Result<Input> {
  use std::io::{Write, BufRead};

  let mut input = String::new();

  let mut stdin = std::io::stdin();
  let mut stdout = std::io::stdout();

  loop {
    stdout.write(b"> ");
    stdout.flush();

    stdin.read_line(&mut input)?;

    let split = input.split_whitespace().collect::<Vec<_>>();

    if split.len() == 0 {
      // continue
    } else if (["h", "?"].contains(&split[0])) {
      return Ok(Input::Help);
    } else if split[0] == "q" {
      return Ok(Input::Quit);
    } else if split[0] == "g" {
      return Ok(Input::Goto(split[1].to_owned()));
    } else {
      return Ok(Input::Unknown(split[0].to_owned()));
    }
  }
}

pub fn main(loc: &Localize) -> Result<()> {
	loop {
    let input = get_input(loc)?;

    match input {
      Input::Help => locprintln!(loc => "cli-help"),
      Input::Quit => return Ok(()),
      Input::Goto(url) => {
        match crate::gemini::get_page(&url) {
          Ok(page) => println!("{}", page),
          Err(e) => eprintln!("Error: {}", e),
        }
      },
      Input::Unknown(s) => {
        locprintln!(loc => ("cli-unknown-input") { "command" => s });
        locprintln!(loc => "cli-help");
      }
    }
	}

	Ok(())
}
