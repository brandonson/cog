pub mod block;
pub mod connection;
use ncurses::{chtype, mvaddch};
use layout::display::Position;

pub trait CharPositioner{
  fn place_char(&self, character: char);
}

impl CharPositioner for Position {
  fn place_char(&self, character: char) {
    mvaddch(self.y as i32, self.x as i32, character as chtype);
  }
}
