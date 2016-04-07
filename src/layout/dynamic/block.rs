use graph::{Graph,GraphBlock};
use layout::Constraint;

pub trait DynamicBlockLayout {
  fn lay_blocks(&self, blocks: &mut [GraphBlock], constraints: &Constraint) -> Option<Vec<Vec<GraphBlock>>>;
}


fn sort_blocks_most_conns_first(blocks:&mut [GraphBlock]) {
  blocks.sort_by(|a,b| {
    a.connection_count.cmp(&b.connection_count)
  });
}




