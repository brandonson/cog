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

  for i in 1..(block.size.height-1) {
    top_left.add_y(i).place_char('|');
    top_left.add_y(i).add_x(block.size.width-1).place_char('|');
  }
  for i in 1..(block.size.width-1) {
    top_left.add_x(i).place_char('-');
    top_left.add_x(i).add_y(block.size.height-1).place_char('-');
  }

  for (i, content) in (0u32..).zip(block.content_lines.iter()) {
    let start_pos = top_left.add_y(i + 1).add_x(2);
    mvprintw(start_pos.y as i32, start_pos.x as i32, content);
  }
}
