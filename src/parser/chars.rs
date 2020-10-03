use crate::parser::*;

pub fn item(input: &str) -> Option<(char,&str)> {
    let mut chars = input.chars();
    chars.next().map(|c| (c,chars.as_str()))
}
pub fn chr<'a>(c: char) -> impl Parser<&'a str, char> {
    item.filter(move |x| *x == c)
}

pub fn digit<'a>() -> impl Parser<&'a str, u32> {
    item.filter(move |x| x.is_digit(10)).map(|x| x.to_digit(10).unwrap())
}

//start with non_zero digit
pub fn number<'a>() -> impl Parser<&'a str, u32> {
    let head = digit().filter(|i| *i != 0);
    let rest = many(digit());
    and(head, rest).map(|(x, xs)| {
        let mut t = x;
        for i in xs.into_iter() {
            t = t * 10 + i
        }
        t
    })
}

pub fn spaces<'a>() -> impl Parser<&'a str, ()> {
    throw(many(item.filter(|c| c.is_ascii_whitespace())))
}

pub fn alpha<'a>() -> impl Parser<&'a str, char> {
    item.filter(|c| c.is_alphabetic())
}

pub fn var<'a>() -> impl Parser<&'a str, String> {
    many1(item.filter(|c| c.is_alphanumeric() || *c == '_')).map(|x| collect(&x))
}

pub fn letter<'a>(s: &str) -> impl Parser<&'a str, String> {
    let mut v:Vec<Box<dyn Parser<&'a str,char>>> = Vec::new();
    for c in s.chars() {
        v.push(Box::new(item.filter(move |x| *x == c)));
    }
    All(v).map(|x| collect(&x))
}

pub fn collect(v: &[char]) -> String {
    v.iter().collect()
}
