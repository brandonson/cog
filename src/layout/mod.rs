pub mod constraint;
pub mod display;

use self::constraint::BlockConstraint;
use self::display::{BlockDisplay};
use data::BlockSpec;

pub trait LayoutManager{
  fn determine_block_vector_layout(
    &self,
    blocks: &[BlockSpec],
    constraint: &BlockConstraint)
      -> Vec<BlockDisplay>;
}

pub struct DownwardLayout {
  pub screen_width: u32,
  pub screen_height: u32
}

impl LayoutManager for DownwardLayout {
  fn determine_block_vector_layout(
    &self,
    blocks:&[BlockSpec],
    constraint:&BlockConstraint)
      -> Vec<BlockDisplay> {
    let mut displays:Vec<BlockDisplay> =
      blocks.iter().map(
        |bspec| {
          BlockDisplay::create_unpositioned_from_spec(bspec, constraint)
        }).collect();
    
    let total_box_y:u32 = displays.iter().map(|b_disp| b_disp.size.height).sum();

    // Try for spacing after each block except the last one
    let y_spacing = constraint.inter_block_distance * (displays.len() - 1) as u32;
    let total_y = total_box_y + y_spacing;

    let mut last_end_y = 0;

    for block_display in displays.iter_mut() {
      block_display.pos.x = self.screen_width/2 - block_display.size.width/2;
      block_display.pos.y = last_end_y + block_display.size.height + y_spacing;
      
      last_end_y = block_display.pos.y;
    }

    displays
  }
}

fn calculate_required_vertical_space(
  block_count: usize,
  spacing_per_block: u32,
  columns: u32)
    -> u32 {
  (block_count as u32 - columns) * spacing_per_block
}
