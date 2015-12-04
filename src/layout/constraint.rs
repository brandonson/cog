use data::*;

pub struct ConnectionConstraint {
  pub min_length: u32,
  pub max_length: u32,
  pub box_distance: u32,
}

pub struct BlockConstraint {
  pub min_limited_width: u32,
  pub max_height_per_width: u32,
  pub max_width_per_height: u32,
  pub inter_block_distance: u32,
}

pub struct LayoutConstraint {
  pub connection: ConnectionConstraint,
  pub block: BlockConstraint,
  pub max_width: u32,
  pub max_height: u32,
}

impl BlockConstraint {
  pub fn max_width_for_height(&self, height:u32) -> u32 {
    ::std::cmp::max(height * self.max_width_per_height, self.min_limited_width)
  }

  pub fn max_height_for_width(&self, width:u32) -> u32 {
    width * self.max_height_per_width
  }
}
