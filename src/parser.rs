use crate::map::Map;
use nom::bytes::complete::take_while1;
use nom::character::complete;
use nom::character::complete::multispace0;
use nom::multi::{separated_list0, separated_list1};
use nom::IResult;

fn consume_line(line: &str) -> IResult<&str, Vec<u16>> {
    let (remaining, data) = separated_list1(take_while1(|c| c == ' '), complete::u16)(line)?;
    Ok((remaining, data))
}

pub fn parse_map(map: &str) -> IResult<&str, (u16, Vec<Vec<u16>>)> {
    //TODO: Comments
    let (remaining, size) = complete::u16(map)?;
    let (remaining, _whitespace) = multispace0(remaining)?; // Remove the trailing newline after the map size
    let (remaining, board) = separated_list0(take_while1(|c| c == '\n'), consume_line)(remaining)?;

    Ok((remaining, (size, board)))
}

pub fn validate_map(size: u16, board: Vec<Vec<u16>>) -> Result<Map, &'static str> {
    //TODO: Check discrete
    if board.len() != size as usize {
        return Err("Board has invalid size");
    }
    let mut map = Map {
        size,
        board: Vec::with_capacity(size as usize),
    };
    for v in board {
        if v.len() != size as usize {
            return Err("Board has invalid size");
        }
        map.board.extend(v);
    }
    Ok(map)
}

//TODO: Test
