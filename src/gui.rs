#![windows_subsystem = "windows"]

#[cfg(target_os="windows")]
extern crate garnet_win as garnet_gui;
#[cfg(target_os="linux")]
extern crate garnet_gtk as garnet_gui;
#[cfg(target_os="macos")]
extern crate garnet_mac as garnet_gui;

use garnet_core::prelude::*;

fn main() -> Result<()> {
  garnet_gui::main()
}
