use layout::{LayoutCreator,Constraint,Layout};
use self::block::DynamicBlockLayout;
use graph::Graph;

mod block;
mod positioning;

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
  fn build_layout(&self, mut g: Graph, constraint: Constraint) -> Layout {
    let mapped_blocks = self.lay_blocks(g.blocks.as_mut_slice(), &constraint);
    //TODO finish
    Layout {
      blocks: vec![],
      connections: vec![],
    }
  }
}
