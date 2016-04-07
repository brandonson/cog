use layout::{LayoutCreator,Constraint,Layout};
use self::block::DynamicBlockLayout;
use graph::Graph;

mod block;

struct DynamicLayoutCreator {
  max_crossed_grids: u32,
  empty_space_grids: u32
}

impl Default for DynamicLayoutCreator {
  fn default() -> DynamicLayoutCreator {
    DynamicLayoutCreator{
      max_crossed_grids: 5,
      empty_space_grids: 2,
    }
  }
}


impl LayoutCreator for DynamicLayoutCreator {
  fn build_layout(&self, g: Graph, constraints: Constraint) -> Layout {
    self.lay_blocks(g.blocks.as_mut_slice(), constraints);
  }
}
