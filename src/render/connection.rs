use display::{ConnectionDisplay, Position};
use ncurses::mvprintw;
use super::CharPositioner;

pub fn draw_connection(
  offset: Position,
  connection: &ConnectionDisplay) {

  connection.color.color_on();
  
  let mut first_drawn = false;
  let mut end:Option<Position> = None;

  for part in connection.parts.iter() {
    if part.start.x == part.end.x {
      let (upper, lower) = 
        if part.end.y > part.start.y {
          (part.start, part.end)
        } else {
          (part.end, part.start)
        };
      for i in 0..(lower.y - upper.y) {
        (upper + offset).add_y(i).place_char(part.internal_character);
      }
    } else {
      let (left, right) =
        if part.end.x > part.start.x {
          (part.start, part.end)
        } else {
          (part.end, part.start)
        };
      for i in 0..(right.x - left.x) {
        (left + offset).add_x(i).place_char(part.internal_character);
      }
    }
    if first_drawn {
      (part.start + offset).place_char(connection.part_end_char);
    } else {
      (part.start + offset).place_char(connection.total_start_char);
      first_drawn = true;
    };
    (offset + part.end).place_char(connection.part_end_char);
    end = Some(part.end);
  }

  for pos in end {
    (pos + offset).place_char(connection.total_end_char);
  }

  connection.color.color_off();
}
