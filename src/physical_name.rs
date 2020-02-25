use std::error::Error;
pub struct PhysicalNameParser;

impl PhysicalNameParser {
    pub fn is_start(nextline: &str) -> bool {
        nextline == "$PhysicalNames"
    }
    pub fn parse<'a>(
        lines: &'a [&'a str],
    ) -> Result<(Vec<PhysicalName>, &'a [&'a str]), PhysicalNameParseError> {
        if !Self::is_start(&lines[0]) {
            return Err(PhysicalNameParseError);
        }
        let num_elements: usize = lines[1].parse().map_err(|_| PhysicalNameParseError)?;
        let elements = lines[2..(num_elements + 2)]
            .iter()
            .map(|line| PhysicalName::from_line(line))
            .collect::<Result<Vec<PhysicalName>, PhysicalNameParseError>>()?;
        if !Self::is_end(&lines[num_elements + 2]) {
            return Err(PhysicalNameParseError);
        }
        Ok((elements, &lines[num_elements + 3..]))
    }
    pub fn is_end(nextline: &str) -> bool {
        nextline == "$EndPhysicalNames"
    }
}
#[derive(Debug)]
pub struct PhysicalName {
    pub dimension: usize,
    pub tag: usize,
    pub name: String,
}
impl PhysicalName {
    pub fn from_line(line: &str) -> Result<Self, PhysicalNameParseError> {
        let mut parsed_str_iter = line.split_whitespace();
        let dimemsion: usize = parsed_str_iter
            .next()
            .ok_or(PhysicalNameParseError)
            .map(|dim_str| dim_str.parse::<usize>().map_err(|_| PhysicalNameParseError))??;
        let tag: usize = parsed_str_iter
            .next()
            .ok_or(PhysicalNameParseError)
            .map(|tag_str| tag_str.parse::<usize>().map_err(|_| PhysicalNameParseError))??;
        let name = parsed_str_iter.next().ok_or(PhysicalNameParseError)?;
        Ok(Self {
            dimension: dimemsion,
            tag: tag,
            name: quoted_string::strip_dquotes(name).ok_or(PhysicalNameParseError)?.to_owned()
        })
    }
}

use std::fmt;
#[derive(Debug)]
pub struct PhysicalNameParseError;
impl fmt::Display for PhysicalNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Physical Name")
    }
}
impl Error for PhysicalNameParseError {}
