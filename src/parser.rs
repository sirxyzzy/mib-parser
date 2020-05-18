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
    use pest::iterators::Pairs;
    use pest::RuleType;

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
        assert!(MibParser::parse(Rule::num, "-1234XYZ").is_err(), "Expected an error!");
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

    #[test]
    fn ident_0() {
        let pair = MibParser::parse(Rule::ident, "ab1ur_d-gh0").unwrap().next().unwrap();
        assert_eq!(pair.as_rule(), Rule::ident);
        assert_eq!(pair.as_str(), "ab1ur_d-gh0");
    }

    #[test]
    fn ident_1() {
        assert!(MibParser::parse(Rule::ident, "0abc").is_err(), "Expected an error for ident 0abc!");
    }

    #[test]
    fn ident_2() {
        assert!(MibParser::parse(Rule::ident, "_abc").is_err(), "Expected an error for ident _abc!");
    }

    #[test]
    fn object_id_0() {
        let mut pairs = MibParser::parse(Rule::obj_id, "synology	 OBJECT 
         IDENTIFIER 
        ::= { enterprises 6574 }").unwrap();

        // Useful for debugging, remember to run as: cargo test -- --nocapture
        // print_pairs(&pairs);
        
        let pair = pairs.next().unwrap();

        assert_eq!(pair.as_rule(), Rule::obj_id);
    }

    #[test]
    fn sequence_1() {
        let mut pairs = MibParser::parse(Rule::sequence_of_type, "SEQUENCE OF wibble").unwrap();
        print_pairs(&pairs);

        let _pair = pairs.next().unwrap();
    }

    fn print_pairs<R: RuleType>(pairs: &Pairs<R>) {
        print_pairs_helper(pairs.clone(), 0)
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
}