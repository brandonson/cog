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
use layout::{LayoutManager};
use layout::downward_cross::CrossingDownwardLayout;
use layout::display::Position;
use data::{Connection, DataSpec, BlockSpec};

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
    min_limited_width: 40,
    max_height_per_width: 1,
    max_width_per_height: 10,
    inter_block_distance: 5
  };
  let spec_ok = match specs {
    Ok(res) => res,
    Err(e) => {println!("{:?}", e); return;}
  };

  let conn_constraint =
    ConnectionConstraint{
      min_length: 10,
      max_length: 1000,
      box_distance: 5};

  let full_constraint = 
    LayoutConstraint {
      connection: conn_constraint,
      block: constraint,
      max_width: 100,
      max_height: 100
    };

  let split_spec: (Vec<_>, Vec<_>) = spec_ok.into_iter().partition(
    |ds| if let &DataSpec::BlockDataSpec(_) = ds {
      true
    } else {
      false 
    }
  );


  let blocks:Vec<BlockSpec> = split_spec.0.into_iter().filter_map(
    |ds| match ds {
      DataSpec::BlockDataSpec(block) => Some(block),
      _ => None
    }).collect();

  let connections:Vec<Connection> =
    split_spec.1.into_iter().filter_map(
      |ds| match ds {
        DataSpec::ConnectionDataSpec(conn) => Some(conn),
        _   => None
      }).collect();

  let layout_manager = CrossingDownwardLayout{screen_width:50, screen_height: 50};

  let layout = layout_manager.determine_block_vector_layout(blocks.as_slice(), &full_constraint.block);

  let connections =
    layout_manager.determine_connection_layout(
      connections.as_slice(),
      layout.as_slice(),
      &full_constraint);

  ncurses::setlocale(ncurses::LcCategory::all, "");
  ncurses::initscr();
  ncurses::start_color();
  ncurses::use_default_colors();
  ncurses::cbreak();

  data::Coloring::init_default_color_pairs();

  for block in layout {
    render::block::draw_block_display(Position{x:0, y:0}, block.1);
  }
  for connection in connections.iter() {
    render::connection::draw_connection(Position{x:0, y:0}, connection);
  }
  ncurses::getch();
  ncurses::endwin();
}
