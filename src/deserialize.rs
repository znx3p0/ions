
#![allow(dead_code, unused)]

use serde::{Deserialize, de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor}};
use std::ops::{AddAssign, MulAssign, Neg};
use crate::error;

struct Deserializer<'de> {
    input: &'de str,
    parsed: Object,
}

const A: char = 0 as char;
const B: char = 255 as char;
// const beta: char = 'A' as char;
// const beta_last: char = 'Z' as char;

peg::parser! {
    grammar token_parser() for str {

        rule ignore() = precedence! {
            ("#"+ [' '|'\n'|'\t']* accepted_chars()* [' '|'\n'|'\t']*)+ {}
            ("#"+ [' '|'\n'|'\t']* accepted_chars()* [' '|'\n'|'\t']* "#")+ ignore()* {}
            [' '|'\n' | '\t']* {}
        }

        rule accepted_chars() -> String =
            s:$([
                'a'..='z'|'A'..='Z'|'0'..='9'|'.'|'('|')'|' '| '<' |'>'|'"'
                |'|'|':'|'_' | ',' |'-'|'='|'+'|'!'|'/'|'?' | '['| '\\'| ']'| '^'| '_'| '`'| '{'| '}'| '~'| '¡'| '¢'| '£'| '¤'| '¥'| '¦'| '§'| '¨'| '©'| 'ª'| '«'| '¬'| '®'| '¯'| '°'| '±'| '²'| '³'| '´'| 'µ'| '¶'| '·'| '¸'| '¹'| 'º'| '»'| '¼'| '½'| '¾'| '¿'| 'À'| 'Á'| 'Â'| 'Ã'| 'Ä'| 'Å'| 'Æ'| 'Ç'| 'È'| 'É'| 'Ê'| 'Ë'| 'Ì'| 'Í'| 'Î'| 'Ï'| 'Ð'| 'Ñ'| 'Ò'| 'Ó'| 'Ô'| 'Õ'| 'Ö'| '×'| 'Ø'| 'Ù'| 'Ú'| 'Û'| 'Ü'| 'Ý'| 'Þ'| 'ß'| 'à'| 'á'| 'â'| 'ã'| 'ä'| 'å'| 'æ'| 'ç'| 'è'| 'é'| 'ê'| 'ë'| 'ì'| 'í'| 'î'| 'ï'| 'ð'| 'ñ'|
                'ò'|'ó'|'ô'|'õ'|'ö'|'÷'|'ø'|'ù'|'ú'|'û'|'ü'|'ý'|'þ'|'ÿ'|'Ā'|'ā'|'Ă'|'ă'|'Ą'|'ą'|'Ć'|'ć'|'Ĉ'|'ĉ'|'Ċ'|'ċ'|'Č'|'č'|'Ď'|'ď'|'Đ'|'đ'|'Ē'|'ē'|'Ĕ'|'ĕ'|'Ė'|'ė'|'Ę'|'ę'|'Ě'|'ě'|'Ĝ'|'ĝ'|'Ğ'|'ğ'|'Ġ'|'ġ'|'Ģ'|'ģ'|'Ĥ'|'ĥ'|'Ħ'|'ħ'|'Ĩ'|'ĩ'|'Ī'|'ī'|'Ĭ'|'ĭ'|'Į'|'į'|'İ'|'ı'|'Ĳ'|'ĳ'|'Ĵ'|'ĵ'|'Ķ'|'ķ'|'ĸ'|'Ĺ'|'ĺ'|'Ļ'|'ļ'|'Ľ'|'ľ'|'Ŀ'|'ŀ'|'Ł'|'ł'|'Ń'|'ń'|'Ņ'|'ņ'|'Ň'|'ň'|'ŉ'|'Ŋ'|'ŋ'|'Ō'|'ō'|'Ŏ'|'ŏ'|'Ő'|'ő'|'Œ'|'œ'|'Ŕ'|'ŕ'|
                'Ŗ'|'ŗ'|'Ř'|'ř'|'Ś'|'ś'|'Ŝ'|'ŝ'|'Ş'|'ş'|'Š'|'š'|'Ţ'|'ţ'|'Ť'|'ť'|'Ŧ'|'ŧ'|'Ũ'|'ũ'|'Ū'|'ū'|'Ŭ'|'ŭ'|'Ů'|'ů'|'Ű'|'ű'|'Ų'|'ų'|'Ŵ'|'ŵ'|'Ŷ'|'ŷ'|'Ÿ'|'Ź'|'ź'|'Ż'|'ż'|'Ž'|'ž'|'ſ'|'ƀ'|'Ɓ'|'Ƃ'|'ƃ'|'Ƅ'|'ƅ'|'Ɔ'|'Ƈ'|'ƈ'|'Ɖ'|'Ɗ'|'Ƌ'|'ƌ'|'ƍ'|'Ǝ'|'Ə'|'Ɛ'|'Ƒ'|'ƒ'|'Ɠ'|'Ɣ'|'ƕ'|'Ɩ'|'Ɨ'|'Ƙ'|'ƙ'|'ƚ'|'ƛ'|'Ɯ'|'Ɲ'|
                'ƞ'| 'Ɵ'| 'Ơ'| 'ơ'| 'Ƣ'| 'ƣ'| 'Ƥ'| 'ƥ'| 'Ʀ'| 'Ƨ'| 'ƨ'| 'Ʃ'| 'ƪ'| 'ƫ'| 'Ƭ'| 'ƭ'| 'Ʈ'| 'Ư'| 'ư'| 'Ʊ'| 'Ʋ'| 'Ƴ'| 'ƴ'| 'Ƶ'| 'ƶ'| 'Ʒ'| 'Ƹ'| 'ƹ'| 'ƺ'| 'ƻ'| 'Ƽ'| 'ƽ'| 'ƾ'| 'ƿ'| 'ǀ'| 'ǁ'| 'ǂ'| 'ǃ'| 'Ǆ'| 'ǅ'| 'ǆ'| 'Ǉ'| 'ǈ'| 'ǉ'| 'Ǌ'| 'ǋ'| 'ǌ'| 'Ǎ'| 'ǎ'| 'Ǐ'| 'ǐ'| 'Ǒ'| 'ǒ'| 'Ǔ'| 'ǔ'| 'Ǖ'| 'ǖ'| 'Ǘ'| 'ǘ'|
                'Ǚ'|'ǚ'|'Ǜ'|'ǜ'|'ǝ'|'Ǟ'|'ǟ'|'Ǡ'|'ǡ'|'Ǣ'|'ǣ'|'Ǥ'|'ǥ'|'Ǧ'|'ǧ'|'Ǩ'|'ǩ'|'Ǫ'|'ǫ'|'Ǭ'|'ǭ'|'Ǯ'|'ǯ'|'ǰ'|'Ǳ'|'ǲ'|'ǳ'|'Ǵ'|'ǵ'|'Ƕ'|'Ƿ'|'Ǹ'|'ǹ'|'Ǻ'|'ǻ'|'Ǽ'|'ǽ'|'Ǿ'|'ǿ'|'Ȁ'|'ȁ'|'Ȃ'|
                'ȃ'|'Ȅ'|'ȅ'|'Ȇ'|'ȇ'|'Ȉ'|'ȉ'|'Ȋ'|'ȋ'|'Ȍ'|'ȍ'|'Ȏ'|'ȏ'|'Ȑ'|'ȑ'|'Ȓ'|'ȓ'|'Ȕ'|'ȕ'|'Ȗ'|'ȗ'|'Ș'|'ș'|'Ț'|'ț'|'Ȝ'|'ȝ'|'Ȟ'|'ȟ'|'Ƞ'|'ȡ'|'Ȣ'|'ȣ'|'Ȥ'|'ȥ'|'Ȧ'|'ȧ'|'Ȩ'|'ȩ'|'Ȫ'|'ȫ'|'Ȭ'|'ȭ'|'Ȯ'|'ȯ'|'Ȱ'|'ȱ'|'Ȳ'|'ȳ'|'ȴ'|'ȵ'|'ȶ'|'ȷ'|'ȸ'|'ȹ'|'Ⱥ'|'Ȼ'|'ȼ'|'Ƚ'|'Ⱦ'|'ȿ'|'ɀ'|'Ɂ'|'ɂ'|'Ƀ'|'Ʉ'|'Ʌ'|'Ɇ'|'ɇ'|'Ɉ'|'ɉ'|'Ɋ'|'ɋ'|'Ɍ'|'ɍ'|'Ɏ'|'ɏ'|'ɐ'|'ɑ'|'ɒ'|'ɓ'|'ɔ'|'ɕ'
                ]+) {s.to_string()}

        rule key() -> String = precedence! {

            $ignore() "\"" s:accepted_chars() "\"" {s}
            $ignore() "'" s:accepted_chars() "'" {s}
            $ignore() "`" s:accepted_chars() "`" {s}
            $ignore() s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '.']+) {s.into()}
        }
        
        rule pair() -> Object = precedence! {
            $ignore() s:key() " "+ d:value() $ignore() {Object::Key(s, Box::new(d))}
            $ignore() s:key() " "* ":" " "* $ignore() d:value() $ignore() {Object::Key(s, Box::new(d))}
            $ignore() s:key() " "* "=" " "* $ignore() d:value() $ignore() {Object::Key(s, Box::new(d))}
        }

        pub rule value() -> Object = precedence! {
            s:key() {Object::Value(s.trim_start().to_string())}
            $ignore() "[" $ignore() s:$( $ignore() value() )* $ignore() "]" {
                let s = s.into_iter().map(|mut s| Object::Value(s.to_string().trim_start().to_string())).collect();
                Object::List(s)
            }
            
            $ignore() "{" $ignore() vec_pairs: pair()* "}"? $ignore() {
                Object::List(vec_pairs)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Object {
    List(Vec<Object>),
    Key(String, Box<Object>),
    Value(String)
}

impl Object {
    pub fn parse<T: AsRef<str>>(input: T) -> Result<Self, error::Err> {
        let mut p: Object = token_parser::value(&format!("{{{}}}", input.as_ref())).unwrap();
        p.recurse();
        Ok(p)
    }

    fn recurse(&mut self) {
        match self {
            Object::List(list) => {
                for elem in list {
                    elem.recurse();
                }
            }
            Object::Key(key, obj) => {
                obj.recurse();
                *self = Object::Key(key.clone(), obj.clone());
            }
            Object::Value(val) => {
                let res = token_parser::value(&val.clone());
                if let Ok(s) = res {
                    if s == *self {
                        return
                    }
                    *self = s;
                    self.recurse();
                }
                

    
            }
        }
    }

}

pub fn from_str<'a, T: Deserialize<'a>>(_input: &'a str) -> Result<T, error::Err> {
    let obj = Object::parse(&std::fs::read_to_string("l.ion").unwrap())?;
    println!("{:#?}", obj);
    todo!()
}

pub fn to_json(o: &Object) -> String {
    let mut p = "".to_string();
    match o {
        Object::List(list) => {
            let obj: bool = loop {
                let mut out = false;
                for i in list {
                    if let Object::Key(_, _) = i {
                        out = true;
                    };
                }
                break out
            };
            if obj {
                p.push('{');
            } else {
                p.push('[');
            }
            
            for obj in list {
                p.push_str(&to_json(obj));
                p.push(',');
            }
            p.remove(p.len()-1);
            if obj {
                p.push('}');
            } else {
                p.push(']');
            }
        }
        Object::Key(k, o) => {
            p.push_str(&format!("\"{}\":{}", k, to_json(o)));
        }
        Object::Value(v) => {
            let c = v.chars().collect::<Vec<char>>();

            if v == "true" || c.first().unwrap().to_string().parse::<u16>().is_ok() {
                p.push_str(&format!("{}", v));
            } else if v == "false" {
                p.push_str(&format!("{}", v));
            } else if c.first().unwrap() != &'{' || c.first().unwrap() !=  &'[' {
                p.push_str(&format!("\"{}\"", v));
            } else {
                p.push_str(&format!("{}", v));
            }
        }
    };
    p
}
