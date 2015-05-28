#![feature(collections)]
extern crate ncurses;
extern crate collections;
#[macro_use]
extern crate nom;

mod parser;
mod data;

fn main() {
  ncurses::initscr();
  ncurses::raw();
  ncurses::mvprintw(10, 20, "Hello.");
  ncurses::getch();
  ncurses::endwin();
}
