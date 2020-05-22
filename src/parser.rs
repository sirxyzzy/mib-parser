use pest::Parser;
use pest::iterators::{Pairs,Pair};
use regex::Regex;
use indextree::Arena;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "mib.pest"] // relative to src
struct MibParser;

#[allow(dead_code)]
/// When a MIB is loaded, the parsed information ends up in a tree of these
pub struct ObjectIdentifierNode {
    pub id: u64,
    pub name: String
}

pub fn parse_mib(mib_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _arena = &mut Arena::<ObjectIdentifierNode>::new();
    MibParser::parse(Rule::main, mib_text)?;
    Ok(())
}

#[allow(dead_code)]
fn get_quoted_string(pair: Pair<Rule>) -> String {
    let raw = pair.into_inner().as_str().to_owned();

    // Replace double quotes with single
    let raw = raw.replace("\"\"", "\"");

    // Squelch newlines surrounded by spaces or tabs
    let re = Regex::new(r" *\r?\n *").unwrap();

    re.replace_all(raw.as_str(), "\n").to_string()
}

#[allow(dead_code)]
fn get_number(pair: Pair<Rule>) -> u64 {
    pair.as_str().parse::<u64>().unwrap()
}

#[allow(dead_code)]
fn get_hex_number(pair: Pair<Rule>) -> u64 {
    let s = pair.as_str();
    let len = s.len();
    // skip prefix and suffix
    u64::from_str_radix(&s[1..len-2], 16).unwrap()
}

#[allow(dead_code)]
fn get_bin_number(pair: Pair<Rule>) -> u64 {
    let s = pair.as_str();
    let len = s.len();
    // skip prefix and suffix
    u64::from_str_radix(&s[1..len-2], 2).unwrap()
}

#[allow(dead_code)]
fn get_identifier(pair: Pair<Rule>) -> String {
    pair.as_str().to_owned()
}

#[allow(dead_code)]
fn print_pair(pair: Pair<Rule>) {
    println!("<<{:?}>> '{}'", pair.as_rule(), pair.as_str());
    print_pairs(pair.into_inner(), 1)
}

fn print_pairs(pairs: Pairs<Rule>, level: usize) {
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("{:indent$}<<{:?}>> '{}'", "", pair.as_rule(), pair.as_str(), indent=level*2);

        // A pair can be converted to an iterator of the tokens which make it up:
        print_pairs(pair.into_inner(), level+1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn number() {
        let number = get_number(parse(Rule::number_string, "12345678"));
        assert_eq!(number, 12345678);
    }

    #[test]
    fn not_a_number() {
        parse_fail(Rule::number_string, "A1234");
    }

    #[test]
    fn quoted_string_1() {
        let result = get_quoted_string(parse(Rule::quoted_string, r#""this is a quoted string""#));
        assert_eq!(result, "this is a quoted string");
    }

    #[test]
    fn quoted_string_2() {
        let result = get_quoted_string(parse(Rule::quoted_string, r#""this is a ""quoted"" string""#));
        assert_eq!(result, r#"this is a "quoted" string"#);
    }

    #[test]
    fn quoted_string_3() {
        let result = get_quoted_string(parse(Rule::quoted_string, "\"this is a    \n   quoted string\""));
        assert_eq!(result, "this is a\nquoted string");

        let result = get_quoted_string(parse(Rule::quoted_string, "\"this is a    \r\n   quoted string\""));
        assert_eq!(result, "this is a\nquoted string");
    }

    #[test]
    fn binary_string() {
        let result = get_bin_number(parse(Rule::binary_string, "'11110000'b"));
        assert_eq!(result, 0b11110000);
    }

    #[test]
    fn hex_string() {
        let result = get_hex_number(parse(Rule::hex_string, "'DEADBEEF'H"));
        assert_eq!(result, 0xDEADBEEF);
    }

    #[test]
    fn identifier_0() {
        let identifier = get_identifier(parse(Rule::identifier, "ab1ur_d-gh0"));
        assert_eq!(identifier, "ab1ur_d-gh0");
    }

    #[test]
    fn identifier_1() {
        parse_fail(Rule::identifier, "0abc");
    }

    #[test]
    fn identifier_2() {
        parse_fail(Rule::identifier, "_abc");
    }

    #[test]
    fn object_id_0() {
        let pair = parse(Rule::value_assignment, "synology OBJECT IDENTIFIER ::= { enterprises 6574 }");
        print_pair(pair)
    }

    #[test]
    fn sequence_1() {
        let pair = parse(Rule::sequence_of_type, "SEQUENCE OF wibble");
        print_pair(pair)
    }

    #[test]
    fn snmp_update() {
        let input = r#"LAST-UPDATED "201309110000Z""#;
        let pair = parse(Rule::snmp_update_part, input);
        print_pair(pair)
    }
    #[test]
    fn x_identifier()
    {
        let input = "synoDisk";
        let pair = parse(Rule::identifier, input);
        print_pair(pair)
    }

    #[test]
    fn some_type() {
        let input = r#"MODULE-IDENTITY
        LAST-UPDATED "201309110000Z"
        ORGANIZATION "www.synology.com"
        CONTACT-INFO
             "postal:   Jay Pan
              email:    jaypan@synology.com"
        DESCRIPTION
            "Characteristics of the disk information"
        REVISION     "201309110000Z"
        DESCRIPTION
            "Second draft.""#;

        let pair = parse(Rule::some_type, input);
        print_pair(pair)
    }

    #[test]
    fn value() {
        let input = "{ synology 2 }";

        let pair = parse(Rule::value, input);
        print_pair(pair)
    }

    #[test]
    fn constraint_list() {
        let input = "( SIZE (0..63) )";
        let pair = parse(Rule::constraint_list, input);
        print_pair(pair)        
    }

    #[test]
    fn value_assignment() {
        let input = r#"synoDisk MODULE-IDENTITY
            LAST-UPDATED "201309110000Z"
            ORGANIZATION "www.synology.com"
            CONTACT-INFO
                 "postal:   Jay Pan
                  email:    jaypan@synology.com"
            DESCRIPTION
                "Characteristics of the disk information"
            REVISION     "201309110000Z"
            DESCRIPTION
                "Second draft."
            ::= { synology 2 }"#;

        let pair = parse(Rule::value_assignment, input);
        print_pair(pair)        
    }

    #[test]
    fn import_list() {
        let input = r#"IMPORTS
        OBJECT-GROUP, MODULE-COMPLIANCE
                    FROM SNMPv2-CONF
        enterprises, MODULE-IDENTITY, OBJECT-TYPE, Integer32
                    FROM SNMPv2-SMI;"#;

        let pair = parse(Rule::import_list, input);
        print_pair(pair)
    }

    #[test]
    fn assignment() {
        let input = r#"synoDisk MODULE-IDENTITY
            LAST-UPDATED "201309110000Z"
            ORGANIZATION "www.synology.com"
            CONTACT-INFO
                 "postal:   Jay Pan
                  email:    jaypan@synology.com"
            DESCRIPTION
                "Characteristics of the disk information"
            REVISION     "201309110000Z"
            DESCRIPTION
                "Second draft."
            ::= { synology 2 }"#;

        let pair = parse(Rule::value_assignment, input);
        print_pair(pair)        
    }

    #[test]
    fn module_body() {
        let input = r#"IMPORTS
            OBJECT-GROUP, MODULE-COMPLIANCE
                        FROM SNMPv2-CONF
            enterprises, MODULE-IDENTITY, OBJECT-TYPE, Integer32
                        FROM SNMPv2-SMI;
        
        synoDisk MODULE-IDENTITY
            LAST-UPDATED "201309110000Z"
            ORGANIZATION "www.synology.com"
            CONTACT-INFO
                 "postal:   Jay Pan
                  email:    jaypan@synology.com"
            DESCRIPTION
                "Characteristics of the disk information"
            REVISION     "201309110000Z"
            DESCRIPTION
                "Second draft."
            ::= { synology 2 }"#;

        let pair = parse(Rule::module_body, input);
        print_pair(pair)
    }

    #[test]
    fn value_test1() {
        // A very simple value, for example, used in groups
        let input = "rmonEtherStatsGroup";
        let pair = parse(Rule::value, input);
        print_pair(pair)
    }

    #[test]
    fn compliance_group() {
        let input = r#"GROUP rmonEtherStatsGroup
        DESCRIPTION
            "The RMON Ethernet Statistics Group is optional.""#;
        let pair = parse(Rule::compliance_group, input);
        print_pair(pair)
    }

    #[test]
    fn snmp_module_part() {
        let input = r#"MODULE -- this module       
              GROUP rmonEtherStatsGroup
                  DESCRIPTION
                      "The RMON Ethernet Statistics Group is optional.""#;
        let pair = parse(Rule::snmp_module_part, input);
        print_pair(pair)
    }

    //
    // helpers
    //

    fn parse(rule: Rule, input: &str) -> Pair<Rule> {
        let result = MibParser::parse(rule, input);

        match result {
            Err(e) => panic!("Failed parse: {}", e),
            Ok(mut pairs) => {
                let pair = pairs.next().unwrap();
                assert_eq!(pair.as_rule(), rule);
                if pair.as_str() != input {
                    println!("Expected rule({:?}) to fully consume '{}'", rule, input);
                    print_pair(pair);
                    panic!("Failed test");
                }
                pair
            }
        }
    }

    fn parse_fail(rule: Rule, input: &str) {
        assert!(MibParser::parse(rule, input).is_err(), "Expected rule({:?}) to fail to parse '{}'", rule, input);
    }
}