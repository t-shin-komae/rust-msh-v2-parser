use gmsh_v2_parser::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
fn main() -> Result<(),Box<dyn Error>>{
    let file = File::open("mesh.msh")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut lines:Vec<&str> = contents.split('\n').collect();
    // let (version,rest) = MeshFormatParser::parse_meshformat(&lines)?;
    // let (physical_name,rest) = PhysicalNameParser::parse(&lines)?;
    // let (nodes,rest) = NodePaser::parse(rest)?;
    // let (elements,rest) = ElementPaser::parse(rest)?;
    let (meshformat,physical_name,nodes,elements) = parse(&lines)?;
    println!("{:?}",meshformat);
    println!("{:?}",physical_name);
    println!("{:?}",nodes);
    println!("{:?}",elements);
    Ok(())
}
