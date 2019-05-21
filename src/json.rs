#![allow(dead_code)]
use crate::Either::*;
use crate::*;
use parser::chars::*;
use parser::*;
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

pub fn json_value<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    any!(
        json_object(),
        json_array(),
        json_string().map(JsonString),
        json_number(),
        json_bool(),
        json_null()
    )
}

fn json_object<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    fn entry<'a>() -> impl Parser<Input = &'a str, Output = (String, JsonValue)> {
        prefix!(
            spaces(),
            infix!(json_string(), token!(':'), lazy(json_value))
        )
    }

    fn map<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
        opt(
            and(entry(), many(prefix!(token!(','), entry()))).map(|(x, xs)| {
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
    around!(token!('{'), map(), token!('}'))
}

fn json_array<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    fn value<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
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
        // prefix!(spaces(),)
    }
    around!(token!('['), prefix!(spaces(), value()), token!(']'))
}

fn json_string<'a>() -> impl Parser<Input = &'a str, Output = String> {
    fn control<'a>() -> impl Parser<Input = &'a str, Output = char> {
        prefix!(chr('\\'), or(item(), prefix!(chr('u'), repeat(digit(), 4)))).flat_map(
            |e| match e {
                Left(c) => match c {
                    '"' | '\\' | '/' => pure(c),
                    'b' => pure('\x08'),
                    't' => pure('\x12'),
                    'n' => pure('\x10'),
                    'f' => pure('\x13'),
                    'r' => pure('\x09'),
                    _ => empty(),
                },
                Right(v) => {
                    let mut t = 0;
                    for i in v.iter() {
                        t = t * 10 + i;
                    }
                    pure(std::char::from_u32(t).unwrap())
                }
            },
        )
    }

    fn other<'a>() -> impl Parser<Input = &'a str, Output = char> {
        item().flat_map(|x| match x {
            '"' | '\\' => empty(),
            _ => pure(x),
        })
    }
    around!(token!('"'), many(any!(control(), other())), chr('"')).map(|v| collect(&v))
}

fn json_number<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    fn flag<'a>() -> impl Parser<Input = &'a str, Output = i32> {
        prefix!(spaces(), opt(chr('-'))).map(|x| match x {
            Some(_) => -1,
            None => 1,
        })
    }

    fn int<'a>() -> impl Parser<Input = &'a str, Output = u32> {
        or(chr('0'), number()).map(|x| match x {
            Left(_) => 0,
            Right(i) => i,
        })
    }

    fn float<'a>() -> impl Parser<Input = &'a str, Output = Option<f32>> {
        opt(prefix!(chr('.'), many1(digit())).map(|mut v| {
            let mut f = 0 as f32;
            while let Some(i) = v.pop() {
                f = f / 10.0 + i as f32
            }
            f / 10.0
        }))
    }

    flag().flat_map(|g| {
        int()
            .flat_map(move |i| {
                float()
                    .map(move |o| match o {
                        Some(f) => JsonFloat(g as f32 * (i as f32 + f)),
                        None => JsonInteger(g * i as i32),
                    })
                    .boxed()
            })
            .boxed()
    })
}

fn json_bool<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    prefix!(spaces(), or(letter("true"), letter("false"))).map(|o| match o {
        Left(_) => JsonBool(true),
        Right(_) => JsonBool(false),
    })
}

fn json_null<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    prefix!(spaces(), letter("null")).map(|_| JsonNull)
}

#[test]
fn test() {
    // assert_eq!(json_number().parse(" 123456"), Some((JsonInteger(123456),"")));
    assert_eq!(
        json_value().parse(" \"-128.123\""),
        Some((JsonFloat(-128.123), ""))
    );
}
