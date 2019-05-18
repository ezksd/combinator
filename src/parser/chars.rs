use super::*;
pub struct Item<'a>(PhantomData<&'a ()>);

pub static ITEM: Item<'static> = Item(PhantomData);
pub fn item<'a>() -> &'static Item<'a> {
    &ITEM
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
            Box::new(pure(x))
        } else {
            Box::new(empty())
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
    use crate::parser::chars::*;

    #[test]
    fn test() {
        let x = item().parse("123");
        assert_eq!(x, Some(('1', "23")));
    }

    #[test]
    fn test1(){
        let x = many(&chr('a')).map(collect).parse("aaabc");
        assert_eq!(x,Some((String::from("aaa"),"bc")));
    }
}
