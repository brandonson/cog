use graph::{Graph,GraphBlock};
use layout::Constraint;
use super::DynamicLayoutCreator;
use super::positioning::{LocalPositionMode,LocalPositionIterator};
use display::{BlockDisplay, Position};
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use graphsort::ConnectionCountSortedBlock;

struct MappedBlockDisplay {
  display: BlockDisplay,
  graph_block: Rc<GraphBlock>,
}

struct BlockPositionMappingBuilder {
  blocks: HashMap<Rc<GraphBlock>, Position>,
  positions: HashMap<Position, Rc<GraphBlock>>,
}

impl BlockPositionMappingBuilder {
  fn add_block_position(&mut self, block:Rc<GraphBlock>, pos:Position) {
    self.blocks.insert(block.clone(), pos);
    self.positions.insert(pos, block);
  }

  fn has_block(&self, block: &Rc<GraphBlock>) -> bool {
    self.blocks.contains_key(block)
  }

  fn has_position(&self, pos: &Position) -> bool {
    self.positions.contains_key(pos)
  }

  fn get_block_position<'s>(&'s self, block: &Rc<GraphBlock>) -> Option<&'s Position> {
    self.blocks.get(block)
  }

  fn into_block_position_map(self) -> HashMap<Rc<GraphBlock>, Position> {
    self.blocks
  }

  fn new(block_count:usize) -> BlockPositionMappingBuilder {
    BlockPositionMappingBuilder {
      blocks: HashMap::with_capacity(block_count),
      positions: HashMap::with_capacity(block_count),
    }
  }
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

    let mut done_blocks = BlockPositionMappingBuilder::new(blocks.len());

    let first = &blocks[0];
    //Positions are unsigned, so just start far enough in that we can't realistically go below zero evenly spacing.
    //TODO rethink this
    done_blocks.add_block_position(first.clone(), Position::new((blocks.len()*2) as u32, (blocks.len()*2) as u32));

    build_block_position_mapping(blocks, &mut done_blocks, first);
    make_mapped_vectors_from_position_mapping(done_blocks.into_block_position_map())
  }
}

fn build_block_position_mapping<'gb>(
  blocks: &'gb [Rc<GraphBlock>],
  done_blocks:&mut BlockPositionMappingBuilder,
  build_from:&'gb Rc<GraphBlock>) {

  let from_position = *done_blocks.get_block_position(build_from).unwrap();

  let sorted_far_ends = collect_sorted_far_ends(blocks, build_from);
  let mut block_pos = Position::new(0,0);

  for graph_block in sorted_far_ends.into_iter() {
    if done_blocks.has_block(&graph_block) {
      continue;
    }

    // if there's only one connection, we want to place nearby
    if graph_block.connection_count() <= 1 {
      //Find a free position in the local positions
      block_pos =
        LocalPositionIterator::new(from_position, LocalPositionMode::Corners, 1)
          .find(|p| !done_blocks.has_position(&p)).unwrap();
      done_blocks.add_block_position(graph_block, block_pos);
      //Don't need to recurse and add this block's connected neighbours, since
      //we got to it along its ownly connection
    } else {
      block_pos =
        LocalPositionIterator::new(from_position, LocalPositionMode::Centered, 1)
          .find(|p| !done_blocks.has_position(&p)).unwrap();
      done_blocks.add_block_position(graph_block.clone(), block_pos);

      for conn in graph_block.connections.borrow().iter() {
        let far = conn.upgrade().unwrap().get_far_end(&graph_block).upgrade().unwrap();
        if !done_blocks.has_block(&far) {
          build_block_position_mapping(
            blocks,
            done_blocks,
            &far);
        }
      }
    }
  }
  let incompleteFind = blocks.iter().find(|blk| !done_blocks.has_block(blk));
  if let Some(incomplete) = incompleteFind {
    block_pos =
      LocalPositionIterator::new(block_pos, LocalPositionMode::Centered, 3)
        .find(|p| !done_blocks.has_position(&p)).unwrap();
    done_blocks.add_block_position(incomplete.clone(), block_pos);
    for conn in incomplete.connections.borrow().iter() {
      let far = conn.upgrade().unwrap().get_far_end(incomplete).upgrade().unwrap();
      if !done_blocks.has_block(&far) {
        build_block_position_mapping(
          blocks,
          done_blocks,
          &far);
      }
    }
  }
}

fn collect_sorted_far_ends<'gb>(blocks: &'gb [Rc<GraphBlock>], build_from: &'gb Rc<GraphBlock>) -> Vec<Rc<GraphBlock>> {
  let mut far_ends:Vec<ConnectionCountSortedBlock<Rc<GraphBlock>>> =
    build_from.connections.borrow().iter()
      .map(|c| c.upgrade().unwrap().get_far_end(build_from).upgrade().unwrap())
      .map(|blk| ConnectionCountSortedBlock(blk))
      .collect();
  far_ends.sort_by(|block_a, block_b| block_b.cmp(block_a));

  far_ends.into_iter().map(|ccsb| ccsb.0).collect()
}

fn make_mapped_vectors_from_position_mapping(mapping: HashMap<Rc<GraphBlock>, Position>) -> Vec<Vec<Option<MappedBlockDisplay>>>{
  vec![]
}


