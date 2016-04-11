use data::{BlockSpec,ConnectionSpec, DataSpec};
use std::rc::Rc;

pub struct GraphBlock {
  pub spec: Rc<BlockSpec>,
  pub connection_count: u32,
}

pub struct GraphConnection {
  pub spec: Rc<ConnectionSpec>,
  pub start_block: Rc<GraphBlock>,
  pub end_block: Rc<GraphBlock>
}

pub struct Graph {
  pub blocks: Vec<Rc<GraphBlock>>,
  pub connections: Vec<Rc<GraphConnection>>,
}

impl GraphBlock {
  fn new(spec: Rc<BlockSpec>) -> GraphBlock {
    GraphBlock {
      spec: spec,
      connection_count: 0,
    }
  }
}

impl GraphConnection {
  fn new(spec: Rc<ConnectionSpec>, blocks: &mut [Rc<GraphBlock>]) -> Result<GraphConnection, String> {
    let start = blocks.iter().find(|b| b.spec.get_name() == spec.start);
    let end = blocks.iter().find(|b| b.spec.get_name() == spec.end);

    if start.is_none()  {
      Err(format!("Block {} does not exist.", spec.start))
    } else if end.is_none() {
      Err(format!("Block {} does not exist", spec.end))
    } else {
      start.unwrap().connection_count += 1;
      end.unwrap().connection_count += 1;
      Ok(GraphConnection {
        spec: spec,
        start_block: start.unwrap().clone(),
        end_block: end.unwrap().clone(),
      })
    }
  }
}

impl Graph {
  pub fn from_dataspec_vec(spec:Vec<DataSpec>) -> Result<Graph, String> {
    graphize_data(spec)
  }
}

fn graphize_data(spec:Vec<DataSpec>) -> Result<Graph, String>{
  let (blocks, connections) = divide_into_block_and_connection(spec);

  let mut graph = Graph{
    blocks: vec![],
    connections: vec![],
  };

  graph.blocks = blocks.into_iter().map(|b| Rc::new(GraphBlock::new(b))).collect();
  let mut temp_conns = Vec::with_capacity(connections.len());
  for conn in connections.into_iter().map(|c| GraphConnection::new(c, &mut graph.blocks)) {
    let actual_conn = try!(conn);
    temp_conns.push(Rc::new(actual_conn));
  }

  graph.connections = temp_conns;

  Ok(graph)
}

fn wrap_rc<D>(data:Vec<D>) -> Vec<Rc<D>> {
  data.into_iter().map(|d| Rc::new(d)).collect()
}

fn divide_into_block_and_connection(specdata:Vec<DataSpec>) -> (Vec<Rc<BlockSpec>>,Vec<Rc<ConnectionSpec>>){
  let split_spec: (Vec<_>, Vec<_>) = specdata.into_iter().partition(
    |ds| if let &DataSpec::BlockDataSpec(_) = ds {
      true
    } else {
      false
    }
  );


  let blocks:Vec<BlockSpec> = split_spec.0.into_iter().filter_map(
    |ds| match ds {
      DataSpec::BlockDataSpec(block) => Some(block),
      _ => None
    }).collect();

  let connections:Vec<ConnectionSpec> =
    split_spec.1.into_iter().filter_map(
      |ds| match ds {
        DataSpec::ConnectionDataSpec(conn) => Some(conn),
        _   => None
      }).collect();
  return (wrap_rc(blocks), wrap_rc(connections));
}
