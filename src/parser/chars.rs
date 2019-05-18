use super::*;
use crate::{pure,empty};
pub struct Item<'a>(PhantomData<&'a ()>);

pub fn item<'a>() -> Item<'a> {
    Item(PhantomData)
}
pub fn chr<'a>(c: char) -> impl Parser<Input=&'a str,Output=char> {
    sat(move |x| *x == c)
}

pub fn digit<'a>() -> impl Parser<Input=&'a str,Output=char>{
    sat(move |x| x.is_digit(10))
}

pub fn sat<'a, F>(f: F) -> impl Parser<Input=&'a str,Output=char>
where
    F: Fn(&char) -> bool,
{
    item().flat_map(move |x| {
        if f(&x) {
            pure!(x)
            // Box::new(pure(x))
        } else {
            empty!()
        }
    })
}

pub fn collect(v: Vec<char>) -> String{
    v.into_iter().collect()
}

// pub fn letter()

impl<'a> Parser for Item<'a> {
    type Input = &'a str;

    type Output = char;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        let mut chars = input.chars();
        chars.next().map(|x| (x, chars.as_str()))
    }
}

#[cfg(test)]
mod test {

    use crate::*;
    use super::*;
    #[test]
    fn test() {
        let x = some!(chr('a')).map(|x| x).parse("abc");
        assert_eq!(x, Some((vec!['a'],"bc")));
    }
}
