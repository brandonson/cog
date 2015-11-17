use super::constraint::{LayoutConstraint,BlockConstraint, ConnectionConstraint};
use super::display::{Position, ConnectionDisplay, BlockDisplay, ConnectionPart};
use data::{Connection, BlockSpec, ConnectionType};
use std::collections::{VecDeque,HashMap};
use std::cmp::max;

use astar::ReusableSearchProblem;
use astar::astar;

use super::path_conversion::conn_display_with_path;

use super::LayoutManager;


pub struct BacktrackingDownwardLayout {
  pub screen_width: u32,
  pub screen_height: u32
}

impl LayoutManager for BacktrackingDownwardLayout {
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
      block_display.pos.y = last_end_y  + y_spacing;

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
    self.recursive_connection_determination(
      connections,
      blocks,
      constraint,
      vec![]).unwrap_or_else(|| vec![])
  }
}

impl BacktrackingDownwardLayout {
  fn recursive_connection_determination<'a>(
    &self,
    connections:&[Connection],
    blocks: &[(&'a BlockSpec, BlockDisplay)],
    constraint: &LayoutConstraint,
    current_paths: Vec<Position>)
      -> Option<Vec<ConnectionDisplay>> {
    let min_box_distance = constraint.connection.box_distance;

    if connections.len() == 0 {
      return Some(vec![]);
    }

    let conn = &connections[0];

    let start_block =
      match find_block_display(&conn.start, blocks) {
        Some(x) => x,
        None => {
          return
            self.recursive_connection_determination(
              &connections[1..],
              blocks,
              constraint,
              current_paths);
        }
      };
    let end_block =
      match find_block_display(&conn.end, blocks) {
        Some(x) => x,
        None => {
          return
            self.recursive_connection_determination(
              &connections[1..],
              blocks,
              constraint,
              current_paths);
        }
      };

    let filtered_blocks:(Vec<_>,Vec<_>) =
      blocks.iter().partition(|b| {
        let name = b.0.get_name();
        name != conn.start && name != conn.end
      });

    let start_connections = find_connection_points(start_block, &current_paths);
    let end_connections = find_connection_points(end_block, &current_paths);
    let mut iter_run = 0;

    loop {
      let mut removing_start = true;
      let mut rem_indicator = iter_run;
      let mut cur_starts = start_connections.clone();
      let mut cur_ends = end_connections.clone();

      while rem_indicator > 0 {
        let remove_from = if removing_start {&mut cur_starts} else {&mut cur_ends};
        let len = remove_from.len();
        remove_from.remove(rem_indicator % len);
        rem_indicator /= len;
        removing_start = !removing_start;
      }

      iter_run += 1;

      if cur_starts.len() == 0 || cur_ends.len() == 0 {
        return None;
      }

      let result = cur_starts.iter().filter_map(
        |start| {
          cur_ends.iter().filter_map(
            |end| {
              let mut node_finder =
                DisplayNodeFinder{
                  blocks: filtered_blocks.0.iter().map(|x| &x.1).collect(),
                  non_checked_blocks: filtered_blocks.1.iter().map(|x| &x.1).collect(),
                  constraint:constraint,
                  end_point: *end,
                  blocked: current_paths.as_slice()};
              let mut problem = node_finder.search(*start, *end);
              let res = astar(&mut problem);
              res
            }
          ).min_by(|vdeq| vdeq.len())
        }
      ).min_by(|vdeq| vdeq.len());

      match result {
        Some(path) => {
          let mut new_paths = current_paths.clone();
          for p in path.iter() {
            new_paths.push(*p);
          }
          let lower_result =
            self.recursive_connection_determination(
              &connections[1..],
              blocks,
              constraint,
              new_paths);
          match lower_result {
            Some(mut vals) => {
              vals.push(conn_display_with_path(conn, path));
              return Some(vals)
            }
            None => {
              println!("No Lower result.  {:?} connections, {:?} starts, {:?} ends",
                        connections.len(), cur_starts.len(), cur_ends.len());
              rem_indicator += 1;}
          }
        }
        None => return None
      }
    }
  }
}

fn find_connection_points(block: &BlockDisplay, paths:&[Position]) -> Vec<Position> {
  let mut core = core_connectors(block);

  let mut found = true;

  while found {
    found = false;
    core =
      core.into_iter().map(
      |p|
        if paths.contains(&p) {
          found = true;
          new_connection_points(p, block).into_iter().filter(|new_p| !paths.contains(new_p)).collect()
        } else {
          vec![p]
        }
    ).flat_map(|v| v).collect();
  }
  core
}

fn new_connection_points(used:Position, block: &BlockDisplay) -> Vec<Position> {
  let mut conns = vec![];
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
