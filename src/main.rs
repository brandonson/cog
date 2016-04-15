#![feature(collections)]
#![feature(plugin)]
#![feature(iter_arith)]
#![feature(iter_cmp)]
#![feature(convert)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate ncurses;
extern crate collections;
extern crate docopt;
extern crate astar;

#[macro_use]
extern crate nom;

use parser::driver::ParserDriver;
use std::error::Error;
use layout::constraint::*;
use display::{Position, BlockDisplay, ConnectionDisplay};
use data::{ConnectionSpec, DataSpec, BlockSpec};

mod parser;
mod data;
mod layout;
mod render;
mod graph;
mod graphsort;
mod display;

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

  let spec_ok = match specs {
    Ok(res) => res,
    Err(e) => {println!("{:?}", e); return;}
  };

  let constraint = BlockConstraint {
    min_limited_width: 40,
    max_height_per_width: 1,
    max_width_per_height: 10,
    inter_block_distance: 5
  };

  let conn_constraint =
    ConnectionConstraint{
      min_length: 10,
      max_length: 1000,
      box_distance: 3};

  let full_constraint =
    LayoutConstraint {
      connection: conn_constraint,
      block: constraint,
      max_width: 150,
      max_height: 100
    };

  let graph = graph::Graph::from_dataspec_vec(spec_ok);

  let (layout, connections) = (vec![], vec![]);

  ncurses::setlocale(ncurses::LcCategory::all, "");
  ncurses::initscr();
  ncurses::start_color();
  ncurses::use_default_colors();
  ncurses::cbreak();

  data::Coloring::init_default_color_pairs();

  for block in layout {
    render::block::draw_block_display(Position{x:0, y:0}, block);
  }
  for connection in connections.iter() {
    render::connection::draw_connection(Position{x:0, y:0}, connection);
  }
  ncurses::getch();
  ncurses::endwin();
}
