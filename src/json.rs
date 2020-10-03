use crate::parser::chars::*;
use crate::parser::*;
use std::collections::HashMap;
use JsonValue::*;
#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    JsonObject(HashMap<String, Box<JsonValue>>), // the first parameter is JsonString
    JsonArray(Vec<JsonValue>),
    JsonString(String),
    JsonInteger(i32),
    JsonFloat(f32),
    JsonBool(bool),
    JsonNull,
}

#[macro_export]
macro_rules! token {
    ($x:tt) => {
        prefix!(spaces(), chr($x))
    };
}

pub fn json_value<'a>() -> impl Parser<&'a str, JsonValue> {
    any!(
        json_object,
        json_array,
        json_string.map(JsonString), // json_number(),
        json_number,
        json_bool,
        json_null
    )
}

fn json_object(input: &str) -> Option<(JsonValue, &str)> {
    fn entry(input: &str) -> Option<((String, JsonValue), &str)> {
        prefix!(spaces(), infix!(json_string, token!(':'), lazy(json_value))).parse(input)
    }

    fn map<'a>() -> impl Parser<&'a str, JsonValue> {
        opt(
            and(entry, many(prefix!(token!(','), entry))).map(|(x, xs)| {
                let mut m = HashMap::new();
                m.insert(x.0, Box::new(x.1));
                for p in xs {
                    m.insert(p.0, Box::new(p.1));
                }
                m
            }),
        )
        .map(|o| match o {
            Some(m) => JsonObject(m),
            None => JsonObject(HashMap::new()),
        })
    }
    around!(token!('{'), map(), token!('}')).parse(input)
}

pub fn json_array(input: &str) -> Option<(JsonValue, &str)> {
    fn value<'a>() -> impl Parser<&'a str, JsonValue> {
        opt(and(
            lazy(json_value),
            many(prefix!(token!(','), lazy(json_value))),
        ))
        .map(|o| match o {
            Some((x, xs)) => {
                let mut v = vec![x];
                for x in xs {
                    v.push(x)
                }
                JsonArray(v)
            }
            None => JsonArray(Vec::new()),
        })
    }
    around!(token!('['), prefix!(spaces(), value()), token!(']')).parse(input)
}

fn json_string(input: &str) -> Option<(String, &str)> {
    fn control(input: &str) -> Option<(char, &str)> {
        prefix!(
            chr('\\'),
            or::<&str, char, Vec<u32>, _, _>(item, prefix!(chr('u'), repeat(digit(), 4)))
        )
        .map(|e| match e {
            Either::Left(c) => match c {
                '"' | '\\' | '/' => c,
                'b' => '\x08',
                't' => '\x12',
                'n' => '\x10',
                'f' => '\x13',
                'r' => '\x09',
                _ => c, // _ => empty(),
            },
            Either::Right(v) => {
                let mut t = 0;
                for i in v.iter() {
                    t = t * 10 + i;
                }
                std::char::from_u32(t).unwrap()
            }
        })
        .parse(input)
    }

    fn other(input: &str) -> Option<(char, &str)> {
        item.filter(|x| *x != '"' && *x != '\\').parse(input)
    }
    around!(token!('"'), many(any!(control, other)), chr('"'))
        .map(|v| collect(&v))
        .parse(input)
}

fn json_number(input: &str) -> Option<(JsonValue, &str)> {
    fn flag<'a>() -> impl Parser<&'a str, i32> {
        prefix!(spaces(), opt(chr('-'))).map(|x| match x {
            Some(_) => -1,
            None => 1,
        })
    }

    fn int<'a>() -> impl Parser<&'a str, u32> {
        or::<&str, char, u32, _, _>(chr('0'), number()).map(|x| match x {
            Either::Left(_) => 0,
            Either::Right(i) => i,
        })
    }

    fn float<'a>() -> impl Parser<&'a str, Option<f32>> {
        opt(prefix!(chr('.'), many1(digit())).map(|mut v| {
            let mut f = 0 as f32;
            while let Some(i) = v.pop() {
                f = f / 10.0 + i as f32
            }
            f / 10.0
        }))
    }

    flag()
        .flat_map(|g| {
            int().flat_map(move |i| {
                float().map(move |o| match o {
                    Some(f) => JsonFloat(g as f32 * (i as f32 + f)),
                    None => JsonInteger(g * i as i32),
                })
            })
        })
        .parse(input)
}

fn json_bool(input: &str) -> Option<(JsonValue, &str)> {
    prefix!(
        spaces(),
        or::<&str, String, String, _, _>(letter("true"), letter("false"))
    )
    .map(|o| match o {
        Either::Left(_) => JsonBool(true),
        Either::Right(_) => JsonBool(false),
    })
    .parse(input)
}

fn json_null(input: &str) -> Option<(JsonValue, &str)> {
    prefix!(spaces(), letter("null"))
        .map(|_| JsonNull)
        .parse(input)
}
