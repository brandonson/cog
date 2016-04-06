use data::*;
use super::constraint::*;
use std::cmp::{min,max};
use std::mem;
use std::ops::Add;

pub mod boxed;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Position{
  pub x: u32,
  pub y: u32,
}

impl Add for Position {
  type Output = Position;

  fn add(self, rhs: Position) -> Position {
    Position{x: self.x + rhs.x, y: self.y + rhs.y}
  }
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BlockCorner {
  TopLeft,
  TopRight,
  BottomLeft,
  BottomRight
}

///Iterator through the corners of a block.
pub struct BlockCornerIter {
  top_left: Position,
  block_size: Size,
  current_corner: Option<BlockCorner>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionPart {
  pub start: Position,
  pub end: Position,

  pub internal_character: char,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionDisplay {
  pub parts: Vec<ConnectionPart>,
  pub color: Coloring,
  pub part_end_char: char,
  pub total_start_char: char,
  pub total_end_char: char
}


impl Position {
  pub fn new(x: u32, y: u32) -> Position {
    Position{x:x, y:y}
  }

  pub fn add_x(self, addx: u32) -> Position {
    Position{x: self.x + addx, y: self.y}
  }

  pub fn add_y(self, addy: u32) -> Position {
    Position{x: self.x, y: self.y + addy}
  }

  pub fn add_size(self, size: Size) -> Position {
    Position{x: self.x + size.width, y: self.y + size.height}
  }

  pub fn manhattan_distance(self, other: Position) -> u32 {
    let xdiff = if self.x > other.x {self.x - other.x} else {other.x - self.x};
    let ydiff = if self.y > other.y {self.y - other.y} else {other.y - self.y};

    xdiff + ydiff
  }
}

// Initialization functions
impl BlockDisplay{
  pub fn create_unpositioned_from_spec(
    spec: &BlockSpec,
    constraint:&BlockConstraint)
      -> BlockDisplay {
    match *spec {
      BlockSpec::Boxed(_, color, ref text) =>
        self::block::boxed::create_unpositioned_from_box_spec(color, text, constraint)
    }
  }

  pub fn center(&self) -> Position {
    Position{
      x: self.pos.x + self.size.width/2,
      y: self.pos.y + self.size.height/2}
  }

  pub fn corners(&self) -> BlockCornerIter {
    BlockCornerIter{
      top_left: self.pos,
      block_size: self.size,
      current_corner: Some(BlockCorner::first_for_iter())}
  }

}


//Collision functions
impl BlockDisplay {
  pub fn distance_to_position(&self, pos:Position) -> u32 {
    let clamped = self.clamp_position(pos);
    clamped.manhattan_distance(pos)
  }

  fn clamp_position(&self, pos:Position) -> Position {
    let clamped_x = max(min(pos.x, self.pos.x + self.size.width-1), self.pos.x);
    let clamped_y = max(min(pos.y, self.pos.y + self.size.height-1), self.pos.y);

    Position{x:clamped_x, y:clamped_y}
  }
}

impl BlockCorner {
  fn first_for_iter() -> BlockCorner {
    BlockCorner::TopLeft
  }

  fn next(self) -> Option<BlockCorner> {
    use self::BlockCorner::*;
    match self {
      TopLeft => Some(TopRight),
      TopRight => Some(BottomLeft),
      BottomLeft => Some(BottomRight),
      BottomRight => None
    }
  }
}

impl Iterator for BlockCornerIter {
  type Item = Position;

  fn next(&mut self) -> Option<Position> {
    match self.current_corner {
      None => None,
      Some(corner) => {
        let result_pos = self.position_for_corner(corner);
        self.current_corner = corner.next();
        Some(result_pos)
      }
    }
  }
}

impl BlockCornerIter {
  fn position_for_corner(&self, corner:BlockCorner) -> Position {
    use self::BlockCorner::*;
    match corner {
      TopLeft => self.top_left,
      TopRight => self.top_left.add_x(self.block_size.width - 1),
      BottomLeft => self.top_left.add_y(self.block_size.height - 1),
      BottomRight =>
        self.top_left
          .add_x(self.block_size.width - 1)
          .add_y(self.block_size.height - 1)
    }
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
