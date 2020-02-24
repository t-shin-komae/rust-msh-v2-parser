use std::error::Error;
pub struct NodePaser;

impl NodePaser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$Nodes"
    }
    pub fn start_parse_all_nodes<'a>(lines:&[&'a str]) -> Result<(Vec<Node>,Vec<&'a str>),Box<dyn Error>>{
        let num_nodes:usize = lines[0].parse()?;
        let nodes = lines[1..(num_nodes+1)].iter().map(|line| Node::from_line(line).unwrap()).collect();
        Ok((nodes,lines[num_nodes+1..].into()))
    }
    pub fn parse<'a>(lines:&'a [&'a str]) -> Result<(Vec<Node>,&'a [&'a str]),NodeParseError>{
        if !NodePaser::is_start(&lines[0]){
            return Err(NodeParseError)
        }
        let num_nodes:usize = lines[1].parse().map_err(|_|NodeParseError)?;
        let nodes = lines[2..(num_nodes+2)].iter().map(|line| Node::from_line(line).unwrap()).collect();
        if !NodePaser::is_end(&lines[num_nodes+2]){
            return Err(NodeParseError)
        }
        Ok((nodes,&lines[num_nodes+3..]))
    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndNodes"
    }
}
#[derive(Debug)]
pub struct Node {
    id: usize,
    coord: [f64; 3],
}
impl Node {
    pub fn from_line(line: &str) -> Result<Self, NodeParseError> {
        let mut iter = line.split_whitespace();
        let id_str: &str = iter.next().ok_or(NodeParseError)?;
        let id: usize = id_str.parse().map_err(|_|NodeParseError)?;
        let mut coord = [0.0f64; 3];
        for (coord_str, coord) in iter.zip(coord.iter_mut()) {
            *coord = coord_str.parse().map_err(|_|NodeParseError)?;
        }
        Ok(Self { id, coord })
    }
}

use std::fmt;
#[derive(Debug)]
pub struct NodeParseError;
impl fmt::Display for NodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Node")
    }
}
impl Error for NodeParseError {}
