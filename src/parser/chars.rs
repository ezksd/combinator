use super::*;
use crate::*;
pub struct Item<'a>(PhantomData<&'a ()>);

pub fn item<'a>() -> Item<'a> {
    Item(PhantomData)
}
pub fn chr<'a>(c: char) -> impl Parser<Input = &'a str, Output = char> {
    sat(move |x| x == c)
}

pub fn digit<'a>() -> impl Parser<Input = &'a str, Output = char> {
    sat(|x| x.is_digit(10))
}

pub fn alpha<'a>() -> impl Parser<Input = &'a str, Output = char> {
    sat(char::is_alphabetic)
}

pub fn sat<'a, F>(f: F) -> impl Parser<Input = &'a str, Output = char>
where
    F: Fn(char) -> bool,
{
    item().flat_map(move |x| if f(x) { pure!(x) } else { empty!() })
}

pub fn var<'a>() -> impl Parser<Input = &'a str, Output = String> {
    some!(any!(chr('_'), sat(char::is_alphanumeric))).map(collect)
}

pub fn letter<'a>() -> impl Parser<Input = &'a str, Output = String> {
    some!(sat(char::is_alphanumeric)).map(collect)
}

pub fn collect(v: Vec<char>) -> String {
    v.into_iter().collect()
}

impl<'a> Parser for Item<'a> {
    type Input = &'a str;

    type Output = char;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        let mut chars = input.chars();
        chars.next().map(|x| (x, chars.as_str()))
    }
}
