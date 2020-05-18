use pest::Parser;
use pest::iterators::Pairs;
use pest::RuleType;
use regex::Regex;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "mib.pest"] // relative to src
struct MibParser;

pub fn parse_mib(mib_text: &str) {
    println!("Parsing mib of size {}", mib_text.len());
    let _ = MibParser::parse(Rule::mib, mib_text);
}

#[allow(dead_code)]
fn get_quoted_string_value<R: RuleType>(mut pairs: Pairs<R>) -> String {
    println!("get_quoted_string_value");

    let raw = pairs.next().unwrap().into_inner().as_str().to_owned();

    // Replace double quotes with single
    let raw = raw.replace("\"\"", "\"");

    // Squelch newlines surrounded by spaces or tabs
    let re = Regex::new(r" *\r?\n *").unwrap();

    re.replace_all(raw.as_str(), "\n").to_string()
}

#[allow(dead_code)]
fn print_pairs<R: RuleType>(pairs: Pairs<R>) {
    print_pairs_helper(pairs, 0)
}

fn print_pairs_helper<R: RuleType>(pairs: Pairs<R>, level: usize) {
    let indent = " ".repeat(level*4);
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("{}{:?} '{}'", indent, pair.as_rule(), pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        print_pairs_helper(pair.into_inner(), level+1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let pair = MibParser::parse(Rule::number_string, "1234XYZ").unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::number_string);
        assert_eq!(pair.as_str(), "1234");

        // Parse result str
        let value = pair.as_str().parse::<i32>().unwrap();
        assert_eq!(value, 1234);
    }

    #[test]
    fn not_a_number() {
        assert!(MibParser::parse(Rule::number_string, "-1234XYZ").is_err(), "Expected an error!");
    }

    #[test]
    fn quoted_string_1() {
        let result = get_quoted_string_value(MibParser::parse(Rule::quoted_string, r#""this is a quoted string""#).unwrap());
        assert_eq!(result, "this is a quoted string");
    }

    #[test]
    fn quoted_string_2() {
        let result = get_quoted_string_value(MibParser::parse(Rule::quoted_string, r#""this is a ""quoted"" string""#).unwrap());
        assert_eq!(result, r#"this is a "quoted" string"#);
    }

    #[test]
    fn quoted_string_3() {
        let result = get_quoted_string_value(MibParser::parse(Rule::quoted_string, "\"this is a    \n   quoted string\"").unwrap());
        assert_eq!(result, "this is a\nquoted string");

        let result = get_quoted_string_value(MibParser::parse(Rule::quoted_string, "\"this is a    \r\n   quoted string\"").unwrap());
        assert_eq!(result, "this is a\nquoted string");
    }

    #[test]
    fn binary_string() {
        let pair = MibParser::parse(Rule::binary_string, "11110000'b").unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::binary_string);
        assert_eq!(pair.as_str(), "11110000'b");
    }

    #[test]
    fn hex_string() {
        let pair = MibParser::parse(Rule::hex_string, "DEADBEEF'H").unwrap().next().unwrap();
        
        assert_eq!(pair.as_rule(), Rule::hex_string);
        assert_eq!(pair.as_str(), "DEADBEEF'H");
    }

    #[test]
    fn identifier_0() {
        let pair = MibParser::parse(Rule::identifier, "ab1ur_d-gh0").unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::identifier);
        assert_eq!(pair.as_str(), "ab1ur_d-gh0");
    }

    #[test]
    fn identifier_1() {
        assert!(MibParser::parse(Rule::identifier, "0abc").is_err(), "Expected an error for identifier 0abc!");
    }

    #[test]
    fn identifier_2() {
        assert!(MibParser::parse(Rule::identifier, "_abc").is_err(), "Expected an error for identifier _abc!");
    }

    #[test]
    fn object_id_0() {
        let pair = MibParser::parse(Rule::obj_id, "synology	 OBJECT 
         IDENTIFIER 
        ::= { enterprises 6574 }").unwrap().next().unwrap();

        assert_eq!(pair.as_rule(), Rule::obj_id);
    }

    #[test]
    fn sequence_1() {
        let _pair = MibParser::parse(Rule::sequence_of_type, "SEQUENCE OF wibble").unwrap().next().unwrap();
    }


}