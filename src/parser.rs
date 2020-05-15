use nom::{
    IResult,
    error::ErrorKind,
    combinator::map_res,
    sequence::tuple,
    character::complete::*,
    regexp::str::re_match
};

use regex::Regex;

#[allow(dead_code)]
fn from_decimal(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 10)
}

#[allow(dead_code)]
pub fn number(input:&str) -> IResult<&str, u32> {
    map_res(
        //take_while1(is_digit),
        digit1,
        from_decimal
    )(input)
}

#[allow(dead_code)]
pub fn identifier(input:&str) -> IResult<&str, String> {
    let (input,t) = tuple((
        alpha1,
        alphanumeric0
    ))(input)?;

    let identifier = format!("{}{}", t.0, t.1);

    Ok((input, identifier))
}

#[allow(dead_code)]
pub fn quoted_string(input:&str) -> IResult<&str, &str> {
    let re = Regex::new(r#""([^"]|"")*""#).unwrap();
    let parser = re_match::<(&str, ErrorKind)>(re);
    parser(input)

    // TODO: Annoyingly we need to post-process, to collapse any
    // spaces either side of any newline characters, this often happens
    // as it is common to indent successive lines of text in a multi-line
    // quoted string
}

pub fn why() {
    println!("Why!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_test1() {
        let (remaining, result) = number("3").unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(result, 3)
    }

    #[test]
    fn number_test2() {
        let (remaining, result) = number("123").unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(result, 123)
    }

    #[test]
    fn number_test3() {
        let (remaining, result) = number("123XY2").unwrap();
        assert_eq!(remaining, "XY2");
        assert_eq!(result, 123)
    }

    #[test]
    fn id_test1() {
        let (remaining, result) = identifier("a123XY2!").unwrap();
        assert_eq!(remaining, "!");
        assert_eq!(result, "a123XY2")
    }

    #[test]
    fn id_test2() {
        let result = identifier("1a123XY2!");

        if let Ok(_) = result {
            panic!("Should not have parsed!");
        }
    }

    #[test]
    fn string_test1() {
        let test_string = "\"Just a comment\"";
        let (remaining, result) = quoted_string(test_string).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(result, test_string)
    }

    #[test]
    fn string_test2() {
        let test_string = "\"Just a comment\nover two lines\"";
        let (remaining, result) = quoted_string(test_string).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(result, test_string)
    }
}