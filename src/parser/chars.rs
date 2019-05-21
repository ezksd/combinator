use super::*;
use crate::*;
pub struct Item<'a>(PhantomData<&'a ()>);
pub fn item<'a>() -> Item<'a> {
    Item(PhantomData)
}
pub fn chr<'a>(c: char) -> impl Parser<Input = &'a str, Output = char> {
    sat(move |x| x == c)
}

pub fn digit<'a>() -> impl Parser<Input = &'a str, Output = u32> {
    sat(|x| x.is_digit(10)).map(|x| x.to_digit(10).unwrap())
}

//start with non_zero digit
pub fn number<'a>() -> impl Parser<Input = &'a str, Output = u32> {
    let head = digit().flat_map(|i| if i == 0 { empty() } else { pure(i) });
    let rest = many(digit());
    and(head, rest).map(|(x, xs)| {
        let mut t = x;
        for i in xs.into_iter() {
            t = t * 10 + i
        }
        t
    })
}

pub fn spaces<'a>() -> impl Parser<Input = &'a str, Output = ()> {
    many(sat(|c| c.is_ascii_whitespace())).throw()
}

pub fn alpha<'a>() -> impl Parser<Input = &'a str, Output = char> {
    sat(char::is_alphabetic)
}

pub fn sat<'a, F>(f: F) -> impl Parser<Input = &'a str, Output = char>
where
    F: Fn(char) -> bool,
{
    item().flat_map(move |x| if f(x) { pure(x) } else { empty() })
}

pub fn var<'a>() -> impl Parser<Input = &'a str, Output = String> {
    many1(any!(chr('_'), sat(char::is_alphanumeric))).map(|x| collect(&x))
}

pub fn letter<'a>(s: &str) -> impl Parser<Input = &'a str, Output = String> {
    let mut v: Vec<Box<Parser<Input = _, Output = _>>> = Vec::new();
    for c in s.chars() {
        v.push(chr(c).boxed());
    }
    All(v).map(|x| collect(&x))
}

pub fn collect(v: &[char]) -> String {
    v.iter().collect()
}

impl<'a> Parser for Item<'a> {
    type Input = &'a str;

    type Output = char;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        let mut chars = input.chars();
        chars.next().map(|x| (x, chars.as_str()))
    }
}
