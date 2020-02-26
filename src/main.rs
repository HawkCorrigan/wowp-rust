extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::string::String;
use std::time::Instant;

use nom::{
    bytes::complete::tag,
    error::{ErrorKind, ParseError},
    number::complete::float,
    sequence::separated_pair,
    IResult,
};

fn parse_digit<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, i32, E> {
    let num = float(input)?;

    Ok((num.0, num.1 as i32))
}

fn parse_date<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (i32, i32), E> {
    separated_pair(parse_digit, tag("/"), parse_digit)(input)
}

fn open_file(path: &str) -> Result<(), Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let start = Instant::now();

    for line in buffer.lines() {
        let test: String = line?;

        let date = parse_date::<(&str, ErrorKind)>(test.as_str());

        // println!("{:#?}", date);
    }

    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
    return Ok(());
}

fn main() {
    open_file("WoWCombatLog.txt");
}
