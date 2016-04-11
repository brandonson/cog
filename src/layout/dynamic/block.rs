use graph::{Graph,GraphBlock};
use layout::Constraint;
use super::DynamicLayoutCreator;
use display::BlockDisplay;

struct MappedBlockDisplay<&'g> {
  display: BlockDisplay,
  graph_block_index: usize,
}

pub trait DynamicBlockLayout {
  fn lay_blocks(&self, blocks: &mut [GraphBlock], constraints: &Constraint) -> Option<Vec<Vec<MappedBlockDisplay>>>;
}


fn sort_blocks_most_conns_first(blocks:&mut [GraphBlock]) {
  blocks.sort_by(|a,b| {
    a.connection_count.cmp(&b.connection_count)
  });
}

impl DynamicBlockLayout for DynamicLayoutCreator {
  fn lay_blocks(&self, blocks: &mut [GraphBlock], constraints: &Constraint) -> Option<Vec<Vec<MappedBlockDisplay>>> {
    sort_blocks_most_conns_first(blocks);
    //Need some way to see what blocks we're connected to
  }
}


