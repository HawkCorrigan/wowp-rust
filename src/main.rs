extern crate nom;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::string::String;
use std::time::Instant;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, digit0, space1},
    combinator::map,
    error::{ErrorKind, ParseError},
    multi::separated_list,
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

fn parse_digit_int(input: &str) -> IResult<&str, u16> {
    map(digit0, |s: &str| s.parse::<u16>().unwrap())(input)
}

#[test]
fn parse_digit_int_works() {
    let num = parse_digit_int("69.420");
    assert_eq!(num, Ok((".420", 69)));
}

#[test]
fn parse_digit_int_edge() {
    let num = parse_digit_int("999");
    assert_eq!(num, Ok(("", 999)));
}

fn time_delimiter(i: &str) -> IResult<&str, char> {
    return alt((char(':'), char('.')))(i);
}

fn parse_time(input: &str) -> IResult<&str, Time> {
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
    let test = parse_time("00:46:03.895");
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

fn parse_date(input: &str) -> IResult<&str, Date> {
    let (input, (month, day)) = separated_pair(parse_digit_int, tag("/"), parse_digit_int)(input)?;

    Ok((input, Date { month, day }))
}

#[test]
fn parse_date_works() {
    let test = parse_date("10/16");

    assert_eq!(test, Ok(("", Date { month: 10, day: 16 })))
}

fn parse_line_values(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list(tag(","), is_not(","))(s)
}

#[test]
fn parse_line_values_works() {
    let test = parse_line_values("a,a,a");

    assert_eq!(test, Ok(("", vec!["a", "a", "a"])));
}

fn parse_log_line(input: &str) -> IResult<&str, (Date, Time, Vec<&str>)> {
    let (input, (date, _, time, _, vals)) =
        tuple((parse_date, space1, parse_time, space1, parse_line_values))(input).unwrap();

    Ok((input, (date, time, vals)))
}

#[test]
fn parse_log_line_test() {
    let test = parse_log_line("10/17 01:00:29.037 HELLO_WORLD");

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
                },
                vec!["HELLO_WORLD"]
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

        let _logLine = parse_log_line(test.as_str());
        // let _date = parse_date(test.as_str());
        // let _test = parse_time("00:46:03.895");

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
