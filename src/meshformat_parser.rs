use std::error::Error;
pub struct MeshFormatParser;
#[derive(Debug)]
pub struct MeshFormat{
    version:(usize,usize),
    filetype:usize,
    data_size:usize
}

impl MeshFormatParser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$MeshFormat"
    }
    pub fn parse_meshformat<'a>(lines:&'a [&'a str]) -> Result<(MeshFormat,&'a [&'a str]),MeshFormatParseError>{
        if !Self::is_start(lines[0]){
            return Err(MeshFormatParseError)
        }
        let meshformat = MeshFormat::from_line(lines[1])?;
        if !Self::is_end(lines[2]){
            return Err(MeshFormatParseError)
        }
        Ok((meshformat,&lines[3..]))
    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndMeshFormat"
    }
}

impl MeshFormat{
    pub fn from_line(nextline:&str) -> Result<Self,MeshFormatParseError>{
        let mut iter = nextline.split_whitespace();
        Ok(Self{version:(2,2),filetype:0,data_size:8})
    }
}
use std::fmt;
#[derive(Debug)]
pub struct MeshFormatParseError;
impl fmt::Display for MeshFormatParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse mesh format")
    }
}
impl Error for MeshFormatParseError {}
