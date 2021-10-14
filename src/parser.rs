use crate::map::Map;
use nom::branch::alt;
use nom::bytes::complete::{is_not, take_while1};
use nom::character::complete;
use nom::character::complete::{char, multispace0};
use nom::combinator::value;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::pair;
use nom::IResult;

fn consume_line(line: &str) -> IResult<&str, Vec<u16>> {
	separated_list1(take_while1(|c| c == ' '), complete::u16)(line)
}

fn parse_comments(input: &str) -> IResult<&str, ()> {
	let mut remaining = input;
	loop {
		if let Ok((rem, ())) = parse_comment(remaining) {
			if rem.len() != remaining.len() {
				// Continue only if something was consumed
				remaining = rem;
				continue;
			}
		}
		break;
	}
	Ok((remaining, ()))
}
fn parse_comment(input: &str) -> IResult<&str, ()> {
	alt((
		value((), pair(char('#'), is_not("\n\r"))),
		value((), multispace0),
	))(input)
}

pub fn parse_map(map: &str) -> IResult<&str, (u16, Vec<Vec<u16>>)> {
	let (remaining, _) = parse_comments(map)?;
	let (remaining, size) = complete::u16(remaining)?;
	let (remaining, _) = parse_comments(remaining)?; // Remove the trailing newline after the map size
	let (remaining, board) = separated_list0(parse_comments, consume_line)(remaining)?;

	Ok((remaining, (size, board)))
}

pub fn validate_map(size: u16, board: Vec<Vec<u16>>) -> Result<Map, &'static str> {
	if board.len() != size as usize {
		return Err("Board has invalid size");
	}
	let mut map = Map {
		size,
		board: Vec::with_capacity(size as usize),
	};
	let mut validator = vec![false; (size * size) as usize];
	for v in board {
		if v.len() != size as usize {
			return Err("Board has invalid size");
		}
		for item in v.iter() {
			if (*item as usize) < validator.len() {
				validator[*item as usize] = true;
			} else {
				return Err("Invalid value in board");
			}
		}
		map.board.extend(v);
	}
	for item in validator.iter() {
		if !(*item) {
			return Err("Invalid Board");
		}
	}
	Ok(map)
}

//TODO: Test
