use graph::GraphBlock;
use std::cmp::Ordering;
use std::borrow::Borrow;

pub struct ConnectionCountSortedBlock<GB:Borrow<GraphBlock>>(pub GB);

impl<GB:Borrow<GraphBlock>> ConnectionCountSortedBlock<GB> {
  fn connection_count(&self) -> usize {
    self.connection_count()
  }
}

impl<GB:Borrow<GraphBlock>> PartialEq for ConnectionCountSortedBlock<GB> {
  fn eq(&self, other: &ConnectionCountSortedBlock<GB>) -> bool {
    self.connection_count() == other.connection_count()
  }
}

impl<GB:Borrow<GraphBlock>> Eq for ConnectionCountSortedBlock<GB> {}

impl<GB:Borrow<GraphBlock>> PartialOrd for ConnectionCountSortedBlock<GB> {
  fn partial_cmp(&self, other: &ConnectionCountSortedBlock<GB>) -> Option<Ordering> {
    self.connection_count().partial_cmp(&other.connection_count())
  }
}

impl<GB:Borrow<GraphBlock>> Ord for ConnectionCountSortedBlock<GB> {
  fn cmp(&self, other: &ConnectionCountSortedBlock<GB>) -> Ordering {
    self.connection_count().cmp(&other.connection_count())
  }
}
