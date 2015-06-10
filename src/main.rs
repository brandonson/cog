#![feature(collections)]
#![feature(plugin)]
#![feature(core)]
#![feature(convert)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate ncurses;
extern crate collections;
extern crate docopt;

#[macro_use]
extern crate nom;

use parser::driver::ParserDriver;
use std::error::Error;
use layout::constraint::BlockConstraint;
use layout::{LayoutManager, DownwardLayout};
use layout::display::Position;
use data::{DataSpec, BlockSpec};

mod parser;
mod data;
mod layout;
mod render;

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

  let constraint = BlockConstraint {
    min_limited_width: 20,
    max_height_per_width: 1,
    max_width_per_height: 10,
    inter_block_distance: 5
  };

  let blocks:Vec<BlockSpec> = specs.unwrap().into_iter().filter_map(
    |ds| match ds {
      DataSpec::BlockDataSpec(block) => Some(block),
      _ => None
    }).collect();

  let layout_manager = DownwardLayout{screen_width:50, screen_height: 50};

  let layout = layout_manager.determine_block_vector_layout(blocks.as_slice(), &constraint);

  ncurses::setlocale(ncurses::LcCategory::all, "");
  ncurses::initscr();
  ncurses::cbreak();

  for block in layout {
    render::block::draw_block_display(Position{x:0, y:0}, block);
  }
  ncurses::getch();
  ncurses::endwin();
}
