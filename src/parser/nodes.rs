use nom::{not_line_ending, line_ending, space, alphanumeric};

use collections::borrow::ToOwned;

use data::*;

named! (box_spec <&[u8], BlockSpec>,
  chain!(
    space?                      ~
    tag!("box")                 ~
    space                       ~
    tag!("text")                ~
    space                       ~
    name: ident_str             ~
    coloring: defaulted_color   ~
    space?                      ~
    line_ending                 ~
    text_line: not_line_ending  ,
    ||{BlockSpec::Boxed(
        name,
        coloring,
        (&String::from_utf8(text_line.to_owned()).unwrap()).trim().to_owned())}
  )
);

named! (color_select <&[u8], Coloring>,
    alt!(
        tag!("red") => { |_| Coloring::Red } |
        tag!("cyan") => { |_| Coloring::Cyan } |
        tag!("blue") => { |_| Coloring::Blue } |
        tag!("black") => { |_| Coloring::Black } |
        tag!("green") => { |_| Coloring::Green } |
        tag!("white") => { |_| Coloring::White } |
        tag!("yellow") => { |_| Coloring::Yellow } |
        tag!("magenta") => { |_| Coloring::Magenta } |
        tag!("default") => { |_| Coloring::Default }
    )
);

named! (coloring_spec <&[u8], Coloring>,
  chain!(
    dbg! (tag!("color")) ~
    dbg! (space)         ~
    c: dbg_dmp!(color_select),
    ||c));

named! (defaulted_color <&[u8], Coloring>,
  chain!(
    c: preceded!(space, coloring_spec)?,
    ||c.unwrap_or(Coloring::Default)));

named! (conn_type_spec <&[u8], ConnectionType>,
        alt!(tag!("generic") => { |_|  ConnectionType::Generic } |
             tag!("singular") => { |_| ConnectionType::Singular } |
             tag!("dual") => { |_| ConnectionType::Dual }));

named! (connection_spec<&[u8], ConnectionSpec>,
  chain!(
    space?                                    ~
    ct: terminated!(conn_type_spec,space)?    ~
    tag!("connection")                        ~
    space                                     ~
    first: ident_str                          ~
    space                                     ~
    second: ident_str                         ~
    color: defaulted_color,
    || ConnectionSpec{ty: ct.unwrap_or(ConnectionType::Generic),
                  start: first.to_owned(),
                  end: second.to_owned(),
                  color:color}
  )
);

named! (ident_str <&[u8], String>,
  chain!(
    data: alphanumeric,
    || String::from_utf8(data.to_owned()).unwrap()));

named! (data_spec <&[u8], DataSpec>,
  alt!(connection_spec => { |conn| DataSpec::ConnectionDataSpec(conn) } |
       box_spec => { |data| DataSpec::BlockDataSpec(data) }
  )
);

pub type DataVec = Vec<DataSpec>;

named! (eol_space_spec < &[u8], Vec<()> >,
    many1!(chain!(space? ~ line_ending, || ())));

named! (pub full_graph_spec <&[u8], DataVec>,
  separated_list!(eol_space_spec, data_spec)
);

#[cfg(test)]
mod test{
  use collections::borrow::ToOwned;
  use super::{box_spec, connection_spec};
  use ::data::*;
  use nom::IResult::Done;
  #[test]
  fn black_box_spec() {
    let input = &b"box text tester color black\nThis is the included text"[..];
    assert_eq!(box_spec(input),
               Done(&b""[..],
                    BlockSpec::Boxed("tester".to_owned(),
                                     Coloring::Black,
                                     "This is the included text".to_owned())));
  }

  #[test]
  fn conn_test() {
    let input = &b"generic connection a b\n"[..];
    let in2 = &b"dual connection foo bar color red"[..];
    let no_ty_spec = &b"connection foo bar\n"[..];
    assert_eq!(connection_spec(input),
               Done(&b"\n"[..], ConnectionSpec{
                 ty:ConnectionType::Generic,
                 start:"a".to_owned(),
                 end:"b".to_owned(),
                 color: Coloring::Default}));
    assert_eq!(connection_spec(in2),
               Done(&b""[..], ConnectionSpec{
                 ty:ConnectionType::Dual,
                 start:"foo".to_owned(),
                 end:"bar".to_owned(),
                 color: Coloring::Red}));
    assert_eq!(connection_spec(no_ty_spec),
               Done(&b"\n"[..], ConnectionSpec{
                 ty:ConnectionType::Generic,
                 start:"foo".to_owned(),
                 end:"bar".to_owned(),
                 color: Coloring::Default}));

  }

  #[test]
  fn color_spec() {
    let input = &b" color red"[..];
    let col_in = &b"red"[..];
    assert_eq!(
        super::color_select(col_in),
        Done(&b""[..], Coloring::Red));
    assert_eq!(
        super::defaulted_color(input),
        Done(&b""[..],Coloring::Red));
  }
}
