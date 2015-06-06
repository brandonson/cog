use data::*;
use super::constraint::*;
use std::cmp::max;
use std::mem;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Position{
  pub x: u32,
  pub y: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Size{
  pub width: u32,
  pub height: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockDisplay {
  pub color: Coloring,
  pub content_lines: Vec<String>,

  pub pos: Position,
  pub size: Size,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionPart {
  pub start: Position,
  pub end: Position,
  pub adjustment: Position,

  pub internal_character: char,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionDisplay {
  pub parts: Vec<ConnectionPart>,
  pub color: Coloring,
  pub part_end_char: char,
}

impl BlockDisplay{
  pub fn create_unpositioned_from_spec(
    spec: &BlockSpec,
    constraint:&BlockConstraint)
      -> BlockDisplay {
    match *spec {
      BlockSpec::Boxed(_, color, ref text) =>
        BlockDisplay::create_unpositioned_from_block_spec(color, text, constraint)
    }
  }

  fn create_unpositioned_from_block_spec(
    color:Coloring,
    test: &str,
    constraint: &BlockConstraint)
      -> BlockDisplay {
    // Check if we can just use the string directly
    // + 4 allows for space on either side, plus lines on sides
    if test.len() < constraint.min_limited_width as usize + 4 {
      return
        BlockDisplay{
          color: color,
          content_lines: vec![test.to_owned()],
          pos: Position{x: 0, y: 0},
          //height is 3 for line + text line
          size: Size{width: test.len() as u32 + 4, height: 3}};
    }

    //This currently ignores the limitation on being too tall,
    //to make implementation easier.  It also just sticks
    //TODO make this properly expand horizontally if needed
    let mut lines:Vec<String> = vec![];
    let mut current_line = String::new();
    let mut widest_line:u32 = 0;
    for word in test.split(' ') {
      if current_line.len() + word.len() + 5 > constraint.min_limited_width as usize{
        let old_current = mem::replace(&mut current_line, word.to_owned());
        widest_line = max(old_current.len() as u32, widest_line);
        lines.push(old_current);
      }

      if current_line.len() + 4 > constraint.min_limited_width as usize{
        let old_current = mem::replace(&mut current_line, String::new());
        widest_line = max(old_current.len() as u32, widest_line);
        lines.push(old_current);
      } else {
        current_line.push(' ');
        current_line.push_str(word);
      }
    }
    if current_line.len() > 0 {
      lines.push(current_line);
    }

    let h = lines.len() + 2;
    BlockDisplay{
      color: color,
      content_lines: lines,
      pos: Position{x: 0, y: 0},
      size: Size{width: widest_line + 4, height: h as u32}}
  }

  pub fn center(&self) -> Position {
    Position{
      x: self.pos.x + self.size.width/2,
      y: self.pos.y + self.size.height/2}
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use data::Coloring;

  #[test]
  fn test_block_display_center(){
    let display =
      BlockDisplay{
        color:Coloring::Default,
        content_lines:vec!(),
        pos: Position{x:20, y:20},
        size: Size{width:30, height: 30}
      };
    assert_eq!(display.center(), Position{x:35, y:35});
  }
}
