use data::{BlockSpec,ConnectionSpec, DataSpec};
use std::rc::{Rc, Weak};
use std::hash::{Hasher, Hash};
use std::cell::RefCell;

#[derive(Debug)]
pub struct GraphBlock {
  pub spec: BlockSpec,
  pub connections: RefCell<Vec<Weak<GraphConnection>>>
}

impl PartialEq for GraphBlock {
  fn eq(&self, rhs:&GraphBlock) -> bool {
    rhs.spec == self.spec
  }
}

impl Eq for GraphBlock{}

impl Hash for GraphBlock {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.spec.hash(state);
  }
}

#[derive(Debug)]
pub struct GraphConnection {
  pub spec: ConnectionSpec,
  pub start_block: Weak<GraphBlock>,
  pub end_block: Weak<GraphBlock>
}

impl PartialEq for GraphConnection {
  fn eq(&self, rhs:&GraphConnection) -> bool {
    rhs.spec == self.spec
  }
}

impl Eq for GraphConnection{}

impl Hash for GraphConnection {
  fn hash<H:Hasher>(&self, state:&mut H) {
    self.spec.hash(state);
  }
}

#[derive(Debug)]
pub struct Graph {
  pub blocks: Vec<Rc<GraphBlock>>,
  pub connections: Vec<Rc<GraphConnection>>,
}

impl GraphBlock {
  fn new(spec: BlockSpec) -> GraphBlock {
    GraphBlock {
      spec: spec,
      connections: RefCell::new(vec![]),
    }
  }

  pub fn connection_count(&self) -> usize {
    self.connections.borrow().len()
  }
}

impl GraphConnection {
  fn new(spec: ConnectionSpec, blocks: &mut [Rc<GraphBlock>]) -> Result<Rc<GraphConnection>, String> {
    //Get the start and end blocks in a structure that can also handle them being the same
    let mut block_find_res = block_find::find_start_and_end(blocks, &spec.start, &spec.end);

    if block_find_res.start.is_none() {
      Err(format!("Block {} does not exist.", spec.start))
    } else if block_find_res.end.is_none() && !block_find_res.are_same {
      Err(format!("Block {} does not exist", spec.end))
    } else {
      let start = Rc::downgrade(block_find_res.start.as_ref().unwrap());
      let end = if block_find_res.are_same {
        start.clone()
      } else {
        Rc::downgrade(block_find_res.end.as_ref().unwrap())
      };
      let rc_conn = Rc::new(GraphConnection {
        spec: spec,
        start_block: start,
        end_block: end,
      });
      block_find_res.push_connections(&rc_conn);
      Ok(rc_conn)
    }
  }

  pub fn get_far_end(&self, from:&GraphBlock) -> Weak<GraphBlock> {
    if (*self.start_block.upgrade().unwrap()).spec == from.spec {
      self.end_block.clone()
    } else {
      self.start_block.clone()
    }
  }

}

impl Graph {
  pub fn from_dataspec_vec(spec:Vec<DataSpec>) -> Result<Graph, Vec<String>> {
    graphize_data(spec)
  }
}

fn graphize_data(spec:Vec<DataSpec>) -> Result<Graph, Vec<String>>{
  let (blocks, connections) = divide_into_block_and_connection(spec);

  let mut graph = Graph{
    blocks: vec![],
    connections: vec![],
  };

  graph.blocks = blocks.into_iter().map(|b| Rc::new(GraphBlock::new(b))).collect();

  let mut temp_conns:Vec<Result<Rc<GraphConnection>, String>> =
    connections.into_iter().map(|c| GraphConnection::new(c, &mut graph.blocks)).collect();

  if temp_conns.iter().find(|c| c.is_err()).is_some() {
    return Err(
      temp_conns.into_iter()
        .filter(|c| c.is_err())
        .map(|err_res| err_res.unwrap_err())
        .collect());
  }

  graph.connections =
    temp_conns.into_iter()
      .map(|ok_res| ok_res.unwrap())
      .collect();
  Ok(graph)
}

fn divide_into_block_and_connection(specdata:Vec<DataSpec>) -> (Vec<BlockSpec>,Vec<ConnectionSpec>){
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
  return (blocks, connections);
}

mod block_find {
  use graph::{GraphBlock, GraphConnection};
  use std::rc::Rc;

  pub struct BlockSearchResult<'blks> {
    pub start: Option<&'blks mut Rc<GraphBlock>>,
    pub end: Option<&'blks mut Rc<GraphBlock>>,
    pub are_same: bool
  }

  impl<'blks> BlockSearchResult<'blks> {
    fn single_block_result(
      block: &'blks mut Rc<GraphBlock>,
      start_name: &str,
      end_name: &str)
        -> BlockSearchResult<'blks> {
      if block.spec.get_name() == start_name {
        let same = block.spec.get_name() == end_name;
        BlockSearchResult {
          start: Some(block),
          end: None,
          are_same: same,
        }
      } else if block.spec.get_name() == end_name {
        BlockSearchResult{
          start: None,
          end: Some(block),
          are_same: false,
        }
      } else {
        BlockSearchResult{
          start: None,
          end: None,
          are_same: false,
        }
      }
    }

    fn merge(res_1: BlockSearchResult<'blks>, res_2:BlockSearchResult<'blks>)
        -> BlockSearchResult<'blks>{
      BlockSearchResult{
        start: res_1.start.or(res_2.start),
        end: res_1.end.or(res_2.end),
        are_same: res_1.are_same || res_2.are_same,
      }
    }

    pub fn push_connections(&mut self, graph_conn: &Rc<GraphConnection>) {
      self.start.as_mut().unwrap().connections.borrow_mut().push(Rc::downgrade(graph_conn));
      if self.are_same {
        self.start.as_mut().unwrap().connections.borrow_mut().push(Rc::downgrade(graph_conn));
      } else {
        self.end.as_mut().unwrap().connections.borrow_mut().push(Rc::downgrade(graph_conn));
      }
    }
  }

  pub fn find_start_and_end<'blks>(
    blocks: &'blks mut [Rc<GraphBlock>],
    start_name: &str,
    end_name: &str)
      -> BlockSearchResult<'blks> {
    if blocks.len() == 1 {
      BlockSearchResult::single_block_result(&mut blocks[0], start_name, end_name)
    } else {
      let mid = blocks.len()/2;
      let (first_half, second_half) = blocks.split_at_mut(mid);
      BlockSearchResult::merge(
        find_start_and_end(first_half, start_name, end_name),
        find_start_and_end(second_half, start_name, end_name))
    }
  }
}
