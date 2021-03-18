

use std::vec::IntoIter;

use pest::{Parser, error::Error};


#[grammar = "grammar.pest"]
#[derive(Parser)]
struct IONParser;

#[derive(Debug)]
pub(crate) enum IONValue<'a> {
    Object(Vec<(&'a str, IONValue<'a>)>),
    Array(Vec<IONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

impl IONValue<'_> {
    fn to_json(&self) -> String {
        match self {
            IONValue::Object(o) => {
                let mut buf = "{".to_string();
                for (key, val) in o {
                    buf.push_str(&format!("\"{}\"", key));
                    buf.push(':');
                    buf.push_str(&val.to_json());
                    buf.push(',');
                }
                buf.remove(buf.len()-1);
                buf.push('}');
                buf
            }
            IONValue::Array(a) => {
                let mut buf = "[".to_string();
                for val in a {
                    buf.push_str(&val.to_json());
                    buf.push(',');
                }
                buf.remove(buf.len()-1);
                buf.push(']');
                buf
            }
            IONValue::String(a) => {
                let mut a = a.to_string();
                if a.as_bytes()[0] as char == '"' {
                    a
                } else if a.as_bytes()[0] as char == '\'' {
                    a.remove(0);
                    a.remove(a.len() - 1);
                    format!("\"{}\"", a)
                } else {
                    format!("\"{}\"", a)
                }
            },
            IONValue::Number(a) => a.to_string(),
            IONValue::Boolean(a) => a.to_string(),
            IONValue::Null => "null".to_string(),
        }
    }
}

pub fn ion_to_json(file: &str) -> Result<String, anyhow::Error> {
    let p = match IONParser::parse(Rule::ion, file)?.next() {
        Some(s) => s,
        None => return Err(anyhow::anyhow!("not a valid file"))
    };
    let p = parse_value(p);
    Ok(p.to_json())
}

use pest::iterators::Pair;

use crate::error;
fn parse_value(pair: Pair<Rule>) -> IONValue {
    match pair.as_rule() {
        Rule::ion |
        Rule::object => {

            IONValue::Object(
                pair.into_inner().map(|s| {
                    let mut inner = s.into_inner();
                    let name = inner.next().unwrap().as_span().as_str();
                    let value = inner.next().unwrap();
                    let value = parse_value(value);
                    (name, value)
                }).collect()
            )
        },
        Rule::array => IONValue::Array(pair.into_inner().map(parse_value).collect()),
        Rule::string => IONValue::String(pair.as_str()),
        Rule::number => IONValue::Number(pair.as_str().parse().unwrap()),
        Rule::boolean => IONValue::Boolean(pair.as_str().parse().unwrap()),
        Rule::null => IONValue::Null,
        | Rule::key_val
        | Rule::value
        | Rule::qchar
        | Rule::sqchar
        | Rule::COMMENT
        | Rule::WHITESPACE => unreachable!(),
    }
}

