use super::*;

pub struct Item<'a>(PhantomData<&'a ()>);

pub static ITEM: Item<'static> = Item(PhantomData);

pub fn item<'a>() -> &'static Item<'a> {
    &ITEM
}
pub fn char<'a>(
    c: char,
) -> FlatMap<
    'static,
    Item<'a>,
    char,
    impl Fn(char) -> Box<dyn Parser<Input = &'a str, Output = char>>,
> {
    item().flat_map(move |x| {
        if x == c {
            Box::new(pure(x))
        } else {
            Box::new(empty())
        }
    })
}

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
    use crate::parser::chars::item;
    use crate::parser::Parser;

    #[test]
    fn test() {
        let x = item().parse("123");
        assert_eq!(x, Some(('1', "23")));
    }
}
