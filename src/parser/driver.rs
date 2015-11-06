use data::DataSpec;
use std::io;
use std::io::Read;
use std::fs::File;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use nom::IResult;
use nom::Err as NomErr;

use super::nodes;

#[derive(Debug)]
pub enum ParserError{
  NomIncomplete,
  NomErr(::nom::ErrorKind, String),
  NotAllParsed,
  IoErr(io::Error)
}

fn construct_nom_err(ek: ::nom::ErrorKind) -> ParserError {
  let msg = format!("nom parsing failed due to error {:?}", ek);
  ParserError::NomErr(ek, msg)
}

impl<'a> From<NomErr<&'a [u8]>> for ParserError {
  fn from(e: NomErr<&'a [u8]>) -> ParserError {
    use nom::Err::*;

    match e {
      Code(uval) => construct_nom_err(uval),
      Node(uval, _) => construct_nom_err(uval),
      Position(uval, _) => construct_nom_err(uval),
      NodePosition(uval, _, _) => construct_nom_err(uval),
    }
  }
}

impl From<io::Error> for ParserError {
  fn from(e: io::Error) -> ParserError {
    ParserError::IoErr(e)
  }
}

impl Display for ParserError {
  fn fmt(&self, f:&mut Formatter) -> Result<(), fmt::Error> {
    self.description().fmt(f)
  }
}

impl Error for ParserError {
  fn description(&self) -> &str {
    use self::ParserError::*;
    match *self  {
      NomIncomplete => "nom parsing failed as it was incomplete",
      NomErr(_, ref string) => string,
      NotAllParsed => "parsing failed as not all input was parsed",
      IoErr(_) => "parsing failed due to an I/O error"
    }
  }

  fn cause(&self) -> Option<&Error> {
    use self::ParserError::*;
    match *self {
      IoErr(ref e) => Some(e),
      _ => None
    }
  }
}

pub trait ParserDriver {
  fn read_to_specification(self) -> Result<Vec<DataSpec>, ParserError>;
}

pub struct FileDriver {
  file: File
}

impl FileDriver{
  pub fn new(filename: &str) -> io::Result<FileDriver> {
    match File::open(filename) {
      Ok(f) => Ok(FileDriver{file: f}),
      Err(e) => Err(e)
    }
  }
}

impl ParserDriver for FileDriver {
  fn read_to_specification(mut self) -> Result<Vec<DataSpec>, ParserError> {
    use self::ParserError::*;

    let mut buf = vec!();
    try!(self.file.read_to_end(&mut buf));

    let nom_result = nodes::full_graph_spec(&buf);
    match nom_result {
      IResult::Done(rem, _) if rem.len() > 0 => {println!("{:?}", String::from_utf8(rem.to_owned())); Err(NotAllParsed)}
      IResult::Done(_, out) => Ok(out),
      IResult::Incomplete(_) => Err(NomIncomplete),
      IResult::Error(err) => Err(ParserError::from(err)),
    }
  }
}
