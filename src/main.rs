extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::string::String;
use std::time::Instant;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit0, space1},
    combinator::map,
    error::{ErrorKind, ParseError},
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Time {
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
    pub millisecond: u16,
}

#[derive(Debug, PartialEq)]
struct Date {
    pub month: u16,
    pub day: u16,
}

fn parse_digit_int<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, u16, E> {
    map(digit0, |s: &str| s.parse::<u16>().unwrap())(input)
}

#[test]
fn parse_digit_int_works() {
    let num = parse_digit_int::<(&str, ErrorKind)>("69.420");
    assert_eq!(num, Ok((".420", 69)));
}

#[test]
fn parse_digit_int_edge() {
    let num = parse_digit_int::<(&str, ErrorKind)>("999");
    assert_eq!(num, Ok(("", 999)));
}

fn time_delimiter<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, char, E> {
    return alt((char(':'), char('.')))(i);
}

fn parse_time<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Time, E> {
    let (input, (hour, _, minute, _, second, _, millisecond)) = tuple((
        parse_digit_int,
        time_delimiter,
        parse_digit_int,
        time_delimiter,
        parse_digit_int,
        time_delimiter,
        parse_digit_int,
    ))(input)?;

    Ok((
        input,
        Time {
            hour,
            minute,
            second,
            millisecond,
        },
    ))
}

#[test]
fn parse_time_fragment_works() {
    let test = parse_time::<(&str, ErrorKind)>("00:46:03.895");
    assert_eq!(
        test,
        Ok((
            "",
            Time {
                hour: 0,
                minute: 46,
                second: 3,
                millisecond: 895
            }
        ))
    )
}

fn parse_date<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Date, E> {
    let (input, (month, day)) = separated_pair(parse_digit_int, tag("/"), parse_digit_int)(input)?;

    Ok((input, Date { month, day }))
}

#[test]
fn parse_date_works() {
    let test = parse_date::<(&str, ErrorKind)>("10/16");

    assert_eq!(test, Ok(("", Date { month: 10, day: 16 })))
}

fn parse_log_line<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (Date, Time), E> {
    let (input, (date, _, time)) = tuple((parse_date, space1, parse_time))(input)?;

    Ok((input, (date, time)))
}

#[test]
fn parse_log_line_test() {
    let test = parse_log_line::<(&str, ErrorKind)>("10/17 01:00:29.037");

    assert_eq!(
        test,
        Ok((
            "",
            (
                Date { month: 10, day: 17 },
                Time {
                    hour: 1,
                    minute: 0,
                    second: 29,
                    millisecond: 37
                }
            )
        ))
    )
}

fn open_file(path: &str) -> Result<(), Error> {
    let file = File::open(path)?;
    let buffer = BufReader::new(file);
    let start = Instant::now();

    for line in buffer.lines() {
        let test: String = line?;

        let _logLine = parse_log_line::<(&str, ErrorKind)>(test.as_str());
        // let _date = parse_date::<(&str, ErrorKind)>(test.as_str());
        // let _test = parse_time::<(&str, ErrorKind)>("00:46:03.895");

        // println!("{:#?}", _logLine);
        // println!("{:#?}", test);
    }

    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
    return Ok(());
}

fn main() {
    open_file("WoWCombatLog.txt");
}
