use pest::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "mib.pest"] // relative to src
struct MibParser;

pub fn parse_mib(mib_text: &str) {
    println!("Parsing mib of size {}", mib_text.len());
    let _ = MibParser::parse(Rule::mib, mib_text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let pair = MibParser::parse(Rule::num, "1234XYZ").unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::num);
        assert_eq!(pair.as_str(), "1234");

        // Parse result str
        let value = pair.as_str().parse::<i32>().unwrap();
        assert_eq!(value, 1234);
    }

    #[test]
    fn not_a_number() {
        assert!(MibParser::parse(Rule::num, "-1234XYZ").is_err(), "Expected and error!");
    }

    #[test]
    fn quoted_1() {
        let pair = MibParser::parse(Rule::quoted, r#""this is a quoted string""#).unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::quoted);
        let inner = pair.into_inner().next().unwrap();
        assert_eq!(inner.as_rule(), Rule::inner);
        assert_eq!(inner.as_str(), "this is a quoted string");
    }

    #[test]
    fn quoted_2() {
        let pair = MibParser::parse(Rule::quoted, r#""this is a ""quoted"" string""#).unwrap().next().unwrap();
        let inner = pair.into_inner().next().unwrap();
        assert_eq!(inner.as_rule(), Rule::inner);
        assert_eq!(inner.as_str(), r#"this is a ""quoted"" string"#);
    }

    #[test]
    fn quoted_3() {
        let pair = MibParser::parse(Rule::quoted, r#""this is a 
        quoted string""#).unwrap().next().unwrap();
        let inner = pair.into_inner().next().unwrap();
        assert_eq!(inner.as_rule(), Rule::inner);
        assert_eq!(inner.as_str(), r#"this is a 
        quoted string"#);
    }
}