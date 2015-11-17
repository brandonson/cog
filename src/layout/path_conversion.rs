use super::display::{ConnectionDisplay, Position, ConnectionPart};
use data::{Connection, ConnectionType};
use std::collections::VecDeque;

/// Takes a connection and a path, and converts them to a
/// ConnectionDisplay.
///
/// This breaks the path down into it's constituent parts (straight lines)
/// and uses information from the connection to figure out what colors and
/// characters to use when rendering it.
pub fn conn_display_with_path(conn: &Connection, mut path: VecDeque<Position>) -> ConnectionDisplay {
  let mut last_change:Option<(i8, i8)> = None;
  let mut first_change:Option<(i8, i8)> = None;
  let mut part_vec:Vec<ConnectionPart> = vec![];

  let mut part_start:Option<Position> = None;
  let mut last_point:Option<Position> = None;

  for slice in path.into_iter().collect::<Vec<_>>().windows(2) {
    let (a,b) = (slice[0], slice[1]);
    if (a.x != b.x) && (a.y != b.y) {
      panic!("How'd we get a path that moves diagonally!");
    }

    // If we haven't started a path part (or just finished one)
    // start one now
    if part_start.is_none() {
      part_start = Some(a);
    }
    //Figure out what the change in x and y values is.
    //Also asserts to make sure that we never have a step
    //of more than 1, as if we do, things are broken
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
      // If the change before the current one was different,
      // we need to create a new connection part
      if new_change != old_change {
        let part_char = if old_change.0 == 0 {'|'} else {'-'};
        part_vec.push(
          ConnectionPart{
            start: part_start.unwrap(),
            end: a,
            internal_character: part_char});
        //The start of the next part is whatever change we just saw
        //that was different from the last part
        part_start = Some(a);
      }
    }

    // Set the last change to be the one we just dealt with
    last_change = Some(new_change);

    // No changes yet - so set up the first one
    if first_change.is_none() {
      first_change = Some(new_change);
    }

    //Record the last point so we can use it for the next iteration
    last_point = Some(b);
  }

  //If we have a part in progress, finish it
  if part_start.is_some() && last_point.is_some() && last_change.is_some(){
    let part_char = if last_change.unwrap().0 == 0 {'|'} else {'-'};
    part_vec.push(
      ConnectionPart{
        start: part_start.unwrap(),
        end: last_point.unwrap(),
        internal_character: part_char});
  }

  // Figure out the characters for the end points
  let (total_start,total_end) =
    total_chars(
      first_change.map(|fc| (-fc.0, -fc.1)),
      last_change,
      conn.ty);

  ConnectionDisplay{
    parts:part_vec,
    color:conn.color,
    part_end_char: '+',
    total_start_char: total_start,
    total_end_char: total_end
  }
}

/// Figure out what characters to place on the end of a connection.
///
/// * `first_change` - The change in x and y values for the first step of the path
/// * `last_change` - The change in x and y values for the last step of the path
/// * `ty` - the type of connection that is being used
fn total_chars(first_change:Option<(i8, i8)>, last_change:Option<(i8,i8)>, ty:ConnectionType) -> (char,char) {
  if ty == ConnectionType::Generic {
    ('#', '#')
  } else if ty == ConnectionType::Singular {
    ('#', incoming_for_change(last_change))
  } else {
    (incoming_for_change(first_change), incoming_for_change(last_change))
  }
}

/// Determines what character to use for a position change
/// when entering a block
fn incoming_for_change(change:Option<(i8, i8)>) -> char {
  if let Some((x,y)) = change {
    if x == -1 {
      '<'
    } else if x == 1 {
      '>'
    } else if y == -1 {
      '^'
    } else {
      'v'
    }
  } else {
    '#'
  }
}
