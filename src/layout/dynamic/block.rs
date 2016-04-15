use graph::{Graph,GraphBlock};
use layout::Constraint;
use super::DynamicLayoutCreator;
use display::{BlockDisplay, Position};
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use graphsort::ConnectionCountSortedBlock;

struct MappedBlockDisplay {
  display: BlockDisplay,
  graph_block: Rc<GraphBlock>,
}

pub trait DynamicBlockLayout {
  fn lay_blocks(&self, blocks: &mut [Rc<GraphBlock>], constraints: &Constraint) -> Vec<Vec<Option<MappedBlockDisplay>>>;
}


fn sort_blocks_most_conns_first(blocks:&mut [Rc<GraphBlock>]) {
  blocks.sort_by(|a,b| {
    a.connections.borrow().len().cmp(&b.connections.borrow().len())
  });
}

impl DynamicBlockLayout for DynamicLayoutCreator {
  fn lay_blocks<'gb>(&self, blocks: &'gb mut [Rc<GraphBlock>], constraints: &Constraint) -> Vec<Vec<Option<MappedBlockDisplay>>> {
    sort_blocks_most_conns_first(blocks);

    let blocks:&'gb [Rc<GraphBlock>] = blocks;

    let mut done_blocks:HashMap<&'gb Rc<GraphBlock>, Position> = HashMap::with_capacity(blocks.len());

    let first = &blocks[0];
    //Positions are unsigned, so just start far enough in that we can't realistically go below zero evenly spacing.
    //TODO rethink this
    done_blocks.insert(first, Position::new((blocks.len()*2) as u32, (blocks.len()*2) as u32));

    build_block_position_mapping(blocks, &mut done_blocks, first);
    make_mapped_vectors_from_position_mapping(done_blocks)
  }
}

fn build_block_position_mapping<'gb>(
  blocks: &'gb [Rc<GraphBlock>],
  done_blocks:&mut HashMap<&'gb Rc<GraphBlock>, Position>,
  build_from:&'gb Rc<GraphBlock>) {

  let mut connected_heap:BinaryHeap<ConnectionCountSortedBlock<Rc<GraphBlock>>> = BinaryHeap::new();

  for conn in build_from.connections.borrow().iter() {
    let far = conn.upgrade().unwrap().get_far_end(build_from);
    connected_heap.push(ConnectionCountSortedBlock(far.upgrade().unwrap()));
  }

  while !connected_heap.is_empty() {
    let next_block = connected_heap.pop().unwrap();
  }
}

fn make_mapped_vectors_from_position_mapping<'gb>(mapping: HashMap<&'gb Rc<GraphBlock>, Position>) -> Vec<Vec<Option<MappedBlockDisplay>>>{
  vec![]
}


