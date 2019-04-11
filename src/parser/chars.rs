use super::*;
pub struct Item();
impl Parser<str, char> for Item {
    fn parse<'a>(&self, i: &'a str) -> Option<(char, &'a str)> {
        let mut x = i.chars();
        x.next().map(|c| (c, x.as_str()))
    }
}

pub fn item() -> Item {
    Item()
}
