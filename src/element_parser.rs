use std::error::Error;
pub struct ElementPaser;

impl ElementPaser{
    pub fn is_start(nextline:&str) -> bool{
        nextline == "$Elements"
    }
    pub fn parse<'a>(lines:&'a [&'a str]) -> Result<(Vec<Element>,&'a [&'a str]),ElementPaserError>{
        if !Self::is_start(&lines[0]){
            return Err(ElementPaserError)
        }
        let num_elements:usize = lines[1].parse().map_err(|_|ElementPaserError)?;
        let elements = lines[2..(num_elements+2)].iter().map(|line| Element::from_line(line)).collect::<Result<Vec<Element>,ElementPaserError>>()?;
        if !Self::is_end(&lines[num_elements+2]){
            return Err(ElementPaserError)
        }
        Ok((elements,&lines[num_elements+3..]))
    }
    pub fn is_end(nextline:&str) -> bool{
        nextline == "$EndElements"
    }
}
#[derive(Debug)]
pub struct Element {
    pub id: usize,
    pub element: ElementType,
    pub tags: [usize; 3],
}
#[derive(Debug)]
pub enum ElementType {
    Node([usize; 1]),
    Line([usize; 2]),
    LineSecondOrder([usize; 3]),
    Triangle([usize; 3]),
    TriangleSecondOrder([usize; 6]),
}
use std::convert::TryFrom;
impl ElementType {
    pub fn new(element_type: usize, node_number_list: &[usize]) -> Result<Self, ElementPaserError> {
        match element_type {
            15 => {
                let nodes: [usize; 1] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Node(nodes))
            },
            1 => {
                let nodes: [usize; 2] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Line(nodes))
            },
            8 => {
                let nodes: [usize; 3] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::LineSecondOrder(nodes))
            }
            9 => {
                let nodes: [usize; 6] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::TriangleSecondOrder(nodes))
            }
            2 => {
                let nodes: [usize; 3] = Self::to_fixed_array(node_number_list)?;
                Ok(ElementType::Triangle(nodes))
            }
            _ => Err(ElementPaserError),
        }
    }
    fn to_fixed_array<'a, A: TryFrom<&'a [usize]>>(
        list: &'a [usize],
    ) -> Result<A, ElementPaserError> {
        TryFrom::try_from(list).map_err(|_| ElementPaserError)
    }
}

impl Element {
    pub fn from_line(line: &str) -> Result<Self, ElementPaserError> {
        let parsed_nums = line
            .split_whitespace()
            .map(|num_str| num_str.parse().map_err(|_| ElementPaserError))
            .collect::<Result<Vec<usize>, ElementPaserError>>()?;
        let mut parsed_nums_iter = parsed_nums.into_iter();
        let id = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        let element_type = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        let number_of_tags = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        let mut tags = [0; 3];
        for (_, tag) in (0..number_of_tags).zip(tags.iter_mut()) {
            *tag = parsed_nums_iter.next().ok_or(ElementPaserError)?;
        }
        let element = ElementType::new(element_type, &parsed_nums_iter.collect::<Vec<usize>>())?;
        Ok(Self { id, element, tags })
    }
}

use std::fmt;
#[derive(Debug)]
pub struct ElementPaserError;
impl fmt::Display for ElementPaserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Element")
    }
}
impl Error for ElementPaserError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_element_parse_oneline() {
        let line = "1 15 2 0 1 1";
        Element::from_line(line).unwrap();
    }
    fn test_element_parser() {
        let line = "1 15 2 0 1 1";
        println!("{:?}", Element::from_line(line));
        panic!("");
    }
}
