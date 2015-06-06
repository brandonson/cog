#![feature(collections)]
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate ncurses;
extern crate collections;
extern crate docopt;

#[macro_use]
extern crate nom;

use parser::driver::ParserDriver;
use std::error::Error;

mod parser;
mod data;
mod layout;

docopt!(Args derive Debug, "
Usage: cog [options] <infile>

Options:
");

fn main() {
  let cli_args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

  let spec_reader = parser::driver::FileDriver::new(&cli_args.arg_infile);

  let specs =
    match spec_reader {
      Ok(fdriver) => fdriver.read_to_specification(),
      Err(e) => {
        println!("Could not read {}: {}", cli_args.arg_infile, e.description());
        return;
      }
    };
  println!("{:?}", specs);
}
