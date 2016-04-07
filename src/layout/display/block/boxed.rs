use data::*;
use layout::constraint::*;
use layout::display::*;
use std::cmp::{min,max};
use std::mem;
use std::ops::Add;

pub fn create_unpositioned_from_box_spec(
  color:Coloring,
  text: &str,
  constraint: &BlockConstraint)
    -> BlockDisplay {
  // Check if we can just use the string directly
  if let Some(display) = naive_single_line_display(color, text, constraint) {
    return display;
  }

  //This currently ignores the limitation on being too tall,
  //to make implementation easier.
  //TODO make this properly expand horizontally if needed
  let mut lines:Vec<String> = vec![];
  let mut current_line = String::new();
  let mut widest_line:u32 = 0;
  for word in text.split(' ') {
    if current_line.len() + word.len() + 5 > constraint.min_limited_width as usize{
      let old_current = mem::replace(&mut current_line, String::new());
      widest_line = max(old_current.len() as u32, widest_line);
      lines.push(old_current);
    }

    if current_line.len() + 4 > constraint.min_limited_width as usize{
      let old_current = mem::replace(&mut current_line, String::new());
      widest_line = max(old_current.len() as u32, widest_line);
      lines.push(old_current);
    } else {
      if current_line.len() > 0 {
        current_line.push(' ');
      }
      current_line.push_str(word);
    }
  }

  if current_line.len() > 0 {
    widest_line = max(current_line.len() as u32, widest_line);
    lines.push(current_line);
  }

  let h = lines.len() + 2;
  BlockDisplay{
    color: color,
    content_lines: lines,
    pos: Position{x: 0, y: 0},
    size: Size{width: widest_line + 4, height: h as u32}}
}

fn naive_single_line_display(
  color:Coloring,
  text: &str,
  constraint: &BlockConstraint)
    -> Option<BlockDisplay> {

  // + 4 allows for space on either side, plus lines on sides
  if text.len() as u32 + 4 <= constraint.min_limited_width {
    Some(BlockDisplay{
      color: color,
      content_lines: vec![text.to_owned()],
      pos: Position{x: 0, y: 0},
      //height is 3 for line + text + line
      size: Size{width: text.len() as u32 + 4, height: 3}})
  } else {
    None
  }
}
