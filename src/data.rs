use ncurses::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BlockSpec{
  Boxed(String, Coloring, String)
}

impl BlockSpec{
  pub fn get_name(&self) -> &str {
    match *self {
      BlockSpec::Boxed(ref s, _, _) => &s
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Connection {
  pub ty: ConnectionType,
  pub start: String,
  pub end: String,
  pub color: Coloring
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ConnectionType {
  Singular,
  Dual,
  Generic
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Coloring {
  Default,
  Black,
  White,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataSpec {
  BlockDataSpec(BlockSpec),
  ConnectionDataSpec(Connection)
}

impl Coloring {

  pub fn ncurses_color(&self) -> Option<i16> {
    use self::Coloring::*;
    match *self {
      Default => None,
      Black => Some(COLOR_BLACK),
      White => Some(COLOR_WHITE),
      Red => Some(COLOR_RED),
      Green => Some(COLOR_GREEN),
      Yellow => Some(COLOR_YELLOW),
      Blue => Some(COLOR_BLUE),
      Magenta => Some(COLOR_MAGENTA),
      Cyan => Some(COLOR_CYAN)
    }
  }

  pub fn color_on(&self) {
    for c in self.ncurses_color() {
      println!("Color {:?} on", c);
      attron(COLOR_PAIR(c));
    }
  }

  pub fn color_off(&self) {
    for c in self.ncurses_color() {
      println!("Color {:?} off", c);
      attroff(COLOR_PAIR(c));
    }
  }

  pub fn init_default_color_pairs() {
    use self::Coloring::*;
    init_color_pair(White, Black);
    init_color_pair(Red, Black);
    init_color_pair(Green, Black);
    init_color_pair(Yellow, Black);
    init_color_pair(Blue, Black);
    init_color_pair(Magenta, Black);
    init_color_pair(Cyan, Black);
  }
}

fn init_color_pair(color_one:Coloring, color_two:Coloring) {
  let c_one = color_one.ncurses_color().unwrap();
  let c_two = color_two.ncurses_color().unwrap();
  init_pair(c_one, c_one, c_two);
}
