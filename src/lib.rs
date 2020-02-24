pub mod node_parser;
pub mod meshformat_parser;
pub mod element_parser;
pub mod physical_name;
pub use node_parser::{Node,NodePaser,NodeParseError};
pub use meshformat_parser::{MeshFormatParser,MeshFormat,MeshFormatParseError};
pub use element_parser::{Element,ElementPaser,ElementPaserError,ElementType};
pub use physical_name::{PhysicalName,PhysicalNameParseError,PhysicalNameParser};

#[derive(Debug)]
pub enum GmshParseError{
    MeshFormatParseError,
    PhysicalNameParseError,
    NodeParseError,
    ElementPaserError
}

pub fn parse(lines:&[&str])->Result<(MeshFormat,Vec<PhysicalName>,Vec<Node>,Vec<Element>),GmshParseError>{
    let (version,rest) = MeshFormatParser::parse_meshformat(&lines).map_err(|_|GmshParseError::MeshFormatParseError)?;
    let (physical_name,rest) = PhysicalNameParser::parse(&rest).unwrap_or((Vec::new(),rest));
    let (nodes,rest) = NodePaser::parse(rest).map_err(|_|GmshParseError::NodeParseError)?;
    let (elements,_) = ElementPaser::parse(rest).map_err(|_|GmshParseError::ElementPaserError)?;
    Ok((version,physical_name,nodes,elements))
}

use std::fmt;
use std::error::Error;
impl fmt::Display for GmshParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse")
    }
}
impl Error for GmshParseError {}

