use layout::display::BlockDisplay;
use layout::display::Position;
use ncurses::mvprintw;
use super::CharPositioner;

pub fn draw_block_display(
  offset: Position,
  block: BlockDisplay) {
  
  let top_left = offset + block.pos;
  
  for corner in block.corners() {
    corner.place_char('+');
  }

  for i in 1..block.size.height {
    top_left.add_y(i).place_char('|');
    top_left.add_y(i).add_x(block.size.width).place_char('|');
  }
  for i in 1..block.size.width {
    top_left.add_x(i).place_char('-');
    top_left.add_x(i).add_y(block.size.height).place_char('-');
  }
}
