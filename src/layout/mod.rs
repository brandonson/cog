pub mod constraint;
pub mod display;
pub mod backtracking;
pub mod memoizer;
pub mod downward_optimized;
pub mod downward_cross;
pub mod path_conversion;

use self::constraint::{LayoutConstraint,BlockConstraint, ConnectionConstraint};
use self::display::{Position, ConnectionDisplay, BlockDisplay, ConnectionPart};
use data::{Connection, BlockSpec};
use std::collections::{VecDeque,HashMap};
use std::cmp::max;

use astar::ReusableSearchProblem;
use astar::astar;

pub trait LayoutManager{
  fn determine_block_vector_layout<'a>(
    &self,
    blocks: &'a [BlockSpec],
    constraint: &BlockConstraint)
      -> Vec<(&'a BlockSpec, BlockDisplay)>;

  fn determine_connection_layout(
    &self,
    connections:&[Connection],
    blocks: &[(&BlockSpec, BlockDisplay)],
    constraint:&LayoutConstraint)
      -> Vec<ConnectionDisplay> ;
}

pub struct DownwardLayout {
  pub screen_width: u32,
  pub screen_height: u32
}

impl LayoutManager for DownwardLayout {
  fn determine_block_vector_layout<'a>(
    &self,
    blocks:&'a [BlockSpec],
    constraint:&BlockConstraint)
      -> Vec<(&'a BlockSpec, BlockDisplay)> {
    let mut displays:Vec<(&'a BlockSpec, BlockDisplay)> =
      blocks.iter().map(
        |bspec| {
          (bspec, BlockDisplay::create_unpositioned_from_spec(bspec, constraint))
        }).collect();

    let total_box_y:u32 = displays.iter().map(|b_disp| b_disp.1.size.height).sum();

    // Try for spacing after each block except the last one
    let y_spacing = constraint.inter_block_distance * (displays.len() - 1) as u32;
    let total_y = total_box_y + y_spacing;

    let mut last_end_y = 0;

    for block_display_tup in displays.iter_mut() {
      let block_display = &mut block_display_tup.1;
      block_display.pos.x = self.screen_width/2 - block_display.size.width/2;
      block_display.pos.y = last_end_y + block_display.size.height + y_spacing;

      last_end_y = block_display.pos.y;
    }

    displays
  }

  fn determine_connection_layout<'a>(
    &self,
    connections:&[Connection],
    blocks: &[(&'a BlockSpec, BlockDisplay)],
    constraint:&LayoutConstraint)
      -> Vec<ConnectionDisplay> {
    let min_distance_from_box = constraint.connection.box_distance;

    let mut open_connectors: HashMap<&'a str, Vec<Position>> = HashMap::new();

    for &(block, ref block_disp) in blocks.iter() {
      open_connectors.insert(block.get_name(), core_connectors(block_disp));
    }

    let mut blocked_positions = vec![];

    let mut paths:Vec<ConnectionDisplay> = Vec::with_capacity(connections.len());

    for conn in connections.iter() {
      let (result,new_starts, new_ends) = {
        let start_conns_opt = open_connectors.get(&conn.start[..]);
        let end_conns_opt = open_connectors.get(&conn.end[..]);

        if start_conns_opt.is_none() || end_conns_opt.is_none() {
          break;
        }

        let start_conns = start_conns_opt.unwrap();
        let end_conns = end_conns_opt.unwrap();
        let filtered_blocks:(Vec<_>,Vec<_>) =
          blocks.iter().partition(|b| {
            let name = b.0.get_name();
            name != conn.start && name != conn.end
          });

        let result = start_conns.iter().filter_map(
          |start| {
            end_conns.iter().filter_map(
              |end| {
                let mut node_finder =
                  DisplayNodeFinder{
                    blocks: filtered_blocks.0.iter().map(|x| &x.1).collect(),
                    non_checked_blocks: filtered_blocks.1.iter().map(|x| &x.1).collect(),
                    constraint:constraint,
                    end_point: *end,
                    blocked: blocked_positions.as_slice()};
                let mut problem = node_finder.search(*start, *end);
                let res = astar(&mut problem);
                res
              }
            ).min_by_key(|vdeq| vdeq.len())
          }
        ).min_by_key(|vdeq| vdeq.len()).unwrap();
        for point in result.iter() {
          blocked_positions.push(*point);
        }
        let new_starts =
          new_connection_points(
            start_conns.to_owned(),
            *result.front().unwrap(),
            find_block_display(&conn.start[..], blocks).unwrap());
        let new_ends =
          new_connection_points(
            end_conns.to_owned(),
            *result.back().unwrap(),
            find_block_display(&conn.end[..], blocks).unwrap());
        (result, new_starts, new_ends)
      };

      {
        *open_connectors.get_mut(&conn.start[..]).unwrap() = new_starts;
        *open_connectors.get_mut(&conn.end[..]).unwrap() = new_ends;
      }

      paths.push(conn_display_with_path(conn, result));
    }

    paths
  }
}

fn new_connection_points(mut conns:Vec<Position>, used:Position, block: &BlockDisplay) -> Vec<Position> {
  if used.x == block.pos.x || used.x == block.pos.x + block.size.width - 1 {
    let change = max(block.size.height/4, 2);
    if used.y - change > block.pos.y {
      conns.push(Position{x:used.x, y: used.y - change})
    }
    if used.y + change < block.pos.y + block.size.height - 1 {
      conns.push(Position{x:used.x, y: used.y + change});
    }
  } else {
    let change = max(block.size.width/4, 2);
    if used.x - change > block.pos.x {
      conns.push(Position{x:used.x - change, y: used.y})
    }
    if used.x + change < block.pos.x + block.size.width - 1 {
      conns.push(Position{x:used.x + change, y: used.y});
    }
  }

  conns
}

fn find_block_display<'a>(
  name: &str,
  blocks: &'a [(&BlockSpec, BlockDisplay)]) -> Option<&'a BlockDisplay> {
  for &(spec, ref display) in blocks.iter() {
    if spec.get_name() == name {
      return Some(display);
    }
  }
  None
}

fn conn_display_with_path(conn: &Connection, mut path: VecDeque<Position>) -> ConnectionDisplay {
  let mut last_change:Option<(i8, i8)> = None;
  let mut part_vec:Vec<ConnectionPart> = vec![];

  let mut part_start:Option<Position> = None;
  let mut last_point:Option<Position> = None;

  for slice in path.into_iter().collect::<Vec<_>>().windows(2) {
    let (a,b) = (slice[0], slice[1]);
    if (a.x != b.x) && (a.y != b.y) {
      panic!("How'd we get a path that moves diagonally!");
    }

    if part_start.is_none() {
      part_start = Some(a);
    }
    let new_change =
      if a.x == b.x {
        if a.y > b.y {
          assert!(a.y - b.y == 1);
          (0, -1)
        } else {
          assert!(b.y - a.y == 1);
          (0, 1)
        }
      } else {
        if a.x > b.x {
          assert!(a.x - b.x == 1);
          (-1, 0)
        } else {
          assert!(b.x - a.x == 1);
          (1, 0)
        }
      };

    if let Some(old_change) = last_change {
      if new_change != old_change {
        let part_char = if old_change.0 == 0 {'|'} else {'-'};
        part_vec.push(
          ConnectionPart{
            start: part_start.unwrap(),
            end: a,
            internal_character: part_char});
        part_start = Some(a);
      }
    }

    last_change = Some(new_change);
    last_point = Some(b);
  }

  if part_start.is_some() && last_point.is_some() && last_change.is_some(){
    let part_char = if last_change.unwrap().0 == 0 {'|'} else {'-'};
    part_vec.push(
      ConnectionPart{
        start: part_start.unwrap(),
        end: last_point.unwrap(),
        internal_character: part_char});
  }

  ConnectionDisplay{
    parts:part_vec,
    color:conn.color,
    part_end_char: '+',
    total_start_char: '#',
    total_end_char: '#'
  }
}

struct DisplayNodeFinder<'a, 'b, 'c> {
  blocks: Vec<&'a BlockDisplay>,
  non_checked_blocks: Vec<&'a BlockDisplay>,
  constraint: &'b LayoutConstraint,
  end_point: Position,
  blocked: &'c [Position]
}

impl<'a, 'b, 'c> ReusableSearchProblem for DisplayNodeFinder<'a, 'b, 'c> {
  type Node = Position;
  type Cost = u32;
  type Iter = ::std::vec::IntoIter<(Position, u32)>;

  fn heuristic(&self, node_a: &Position, node_b: &Position) -> u32 {
    (*node_a).manhattan_distance(*node_b)
  }

  fn neighbors(&mut self, node:&Position) -> ::std::vec::IntoIter<(Position, u32)> {
    let mut positions = vec![];

    if node.x > 0 {
      positions.push(Position{x:node.x-1, y: node.y});
    }
    if node.y > 0 {
      positions.push(Position{x:node.x, y: node.y-1});
    }
    if node.x < self.constraint.max_width {
      positions.push(Position{x:node.x+1, y: node.y});
    }
    if node.y < self.constraint.max_height {
      positions.push(Position{x:node.x, y: node.y + 1});
    }

    positions.into_iter()
      .filter(|p| self.is_valid_pos(*p))
      .map(|p| (p, 1)).collect::<Vec<_>>().into_iter()
  }
}

impl<'a, 'b, 'c> DisplayNodeFinder<'a, 'b, 'c> {
  fn is_valid_pos(&self, pos:Position) -> bool {
    if pos == self.end_point {
      return true;
    }
    if self.blocked.contains(&pos) {
      return false;
    }
    let is_outside_distance =
      self.blocks.iter().all(
        |b| b.distance_to_position(pos) >= self.constraint.connection.box_distance
      );
    let is_not_inside =
      self.non_checked_blocks.iter().all(
        |b| b.distance_to_position(pos) > 0
      );

    is_outside_distance && is_not_inside
  }
}

fn calculate_required_vertical_space(
  block_count: usize,
  spacing_per_block: u32,
  columns: u32)
    -> u32 {
  (block_count as u32 - columns) * spacing_per_block
}

fn core_connectors(display:&BlockDisplay) -> Vec<Position>{
  vec![
    Position{x: display.pos.x, y: display.pos.y + (display.size.height/2)},
    Position{x: display.pos.x + (display.size.width/2), y: display.pos.y},
    Position{x: display.pos.x + display.size.width-1, y: display.pos.y + (display.size.height/2)},
    Position{x: display.pos.x + (display.size.width/2), y: display.pos.y + display.size.height-1}
  ]
}
