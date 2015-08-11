use layout::display::Position;
use std::collections::{VecDeque, HashMap};

pub trait PathCreator{
  fn calculate_new_path(&mut self, start: Position, end: Position) -> Option<VecDeque<Position>>;
  fn is_valid_path(&self, path: &VecDeque<Position>) -> bool;
}

pub struct PathMemoizer{
  pub paths: HashMap<(Position, Position), Vec<VecDeque<Position>>>,
}

impl PathMemoizer{
  pub fn new() -> PathMemoizer{
    PathMemoizer{paths: HashMap::new()}
  }
  pub fn get_shortest_option<'a, PC>(
    &'a mut self,
    positions: Vec<(Position, Position)>,
    creator: &mut PC)
      -> Option<&'a VecDeque<Position>>
      where PC: PathCreator {
    let mut shortest_path_pos: Option<(Position, Position)> = None;
    let mut short_path_len = 0;

    for pos in positions {
      let path = self.get_path(pos.0, pos.1, creator);
      if let Some(new_path) = path {
        if let Some(_) = shortest_path_pos {
      
          if short_path_len > new_path.len() {
            shortest_path_pos = Some(pos);
            short_path_len = new_path.len();
          }
        } else {
          shortest_path_pos = Some(pos);
          short_path_len = new_path.len();
        }
      }
    }

    match shortest_path_pos {
      Some((start, end)) => self.get_path(start, end, creator),
      None => None
    }
  }

  pub fn get_path<'a, PC>(
    &'a mut self,
    start: Position,
    end: Position,
    creator: &mut PC)
      -> Option<&'a VecDeque<Position>>
      where PC: PathCreator{
    //Look for the shortest valid path we have
    let path_vec = self.paths.entry((start, end)).or_insert(vec![]);
    let mut short_index = 0;
    let mut short_len = 0;
    for (i, path) in path_vec.iter().enumerate() {
      if short_len == 0 {
        if creator.is_valid_path(path) {
          short_index = i;
          short_len = path.len();
        }
      } else {
        if path.len() < short_len && creator.is_valid_path(path) {
          short_index = i;
          short_len = path.len();
        }
      }
    }
    if short_len != 0 {
      println!("Memoized");
      path_vec.get(short_index)
    } else {
      println!("No Memo");
      let path = creator.calculate_new_path(start, end);
      if let Some(p) = path {
        path_vec.push(p);
        path_vec.last()
      } else {
        None
      }
    }
  }

  fn get_new_path<'a, PC>(
    &'a mut self,
    start: Position,
    end: Position,
    creator: &mut PC)
      -> Option<&'a VecDeque<Position>>
      where PC: PathCreator {
    let new_path = creator.calculate_new_path(start, end);
    match new_path {
      Some(path) => {
        //Make sure we have a vector, then push the new path
        self.paths.entry((start, end)).or_insert(vec![]).push(path);
        //This should never fail. We just need to get the pointer version
        //of our path, so we go back through the map/vec
        self.paths.get(&(start, end)).and_then(|v| v.last())
      }
      None => None
    }
  }
}

fn is_new_short_path(
  path: &VecDeque<Position>,
  old_shortest: Option<&VecDeque<Position>>)
    -> bool {
  /* 
   * We have a new short path if:
   *   A: Old short path has greater length than the one we're checking
   *   B: There is no old short path
   */
  old_shortest.map(|short| short.len() > path.len()).unwrap_or(true)
}
