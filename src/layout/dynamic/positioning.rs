use display::Position;

pub struct LocalPositionIterator {
  //Base position to iterate out from
  base_position: Position,
  //Current list of positions (four corners from base)
  //Looks something like this
  /*
   * 1 2
   *  B
   * 3 4
   */
  //Where B is the base position
  current_positions: [Position; 4],
  //Index into the list of current positions
  current_position_idx: usize,
  //How distant we are from the base position
  //Each set is one diagonal step farther than the
  //previous set
  position_set: u32,
}

impl LocalPositionIterator {
  fn gen_position<F1,F2>(&self, adj_x: F1, adj_y: F2) -> Position
      where F1: Fn(u32,u32)->u32,
            F2: Fn(u32,u32)->u32{
    let mut b = self.base_position.clone();
    b.x = adj_x(b.x, self.position_set);
    b.y = adj_y(b.y, self.position_set);
    b
  }
  fn go_to_set(&mut self, new_position_set: u32) {
    self.current_position_idx = 0;
    self.position_set = new_position_set;

    self.current_positions[0] = self.gen_position(|bx, set| bx - set, |by, set| by - set);
    self.current_positions[1] = self.gen_position(|bx, set| bx + set, |by, set| by - set);
    self.current_positions[2] = self.gen_position(|bx, set| bx - set, |by, set| by + set);
    self.current_positions[3] = self.gen_position(|bx, set| bx + set, |by, set| by + set);
  }

  pub fn new(base: Position) -> LocalPositionIterator {
    let mut iter = LocalPositionIterator {
      base_position: base,
      current_positions: [Position{x:0,y:0},Position{x:0,y:0},Position{x:0,y:0},Position{x:0,y:0}],
      current_position_idx: 0,
      position_set: 0,
    };

    iter.go_to_set(1);
    iter
  }
}

impl Iterator for LocalPositionIterator {
  type Item = Position;

  fn next(&mut self) -> Option<Position> {
    //Recreate position list if we're out
    if self.current_position_idx >= self.current_positions.len() {
      let next_set = self.position_set + 1;
      self.go_to_set(next_set);
    }

    //Get next position and advance index
    let res_idx = self.current_position_idx;
    self.current_position_idx += 1;
    Some(self.current_positions[res_idx])
  }
}
