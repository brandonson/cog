use graph::Graph;
use display::{BlockDisplay, ConnectionDisplay};
pub use self::constraint::LayoutConstraint as Constraint;

pub mod constraint;
pub mod dynamic;

pub struct Layout {
  pub blocks: Vec<BlockDisplay>,
  pub connections: Vec<ConnectionDisplay>
}

pub trait LayoutCreator {
  fn build_layout(&self, g: Graph, constraints: Constraint) -> Layout;
}
