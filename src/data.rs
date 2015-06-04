#[derive(Debug, PartialEq, Eq)]
pub enum BlockSpec{
  Boxed(String, Coloring, String)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Connection {
  pub ty: ConnectionType,
  pub start: String,
  pub end: String,
  pub color: Coloring
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConnectionType {
  Singular,
  Dual,
  Generic
}

#[derive(Debug, PartialEq, Eq)]
pub enum Coloring {
  Default,
  Black,
  White,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataSpec {
  BlockDataSpec(BlockSpec),
  ConnectionDataSpec(Connection)
}
