use super::parser::chars::*;
use super::parser::*;
#[test]
fn test1() {
    assert_eq!(item().parse("123"), Some(('1', "23")));
    assert_eq!(item().parse(""), None);
    assert_eq!(chr('1').parse("123"), Some(('1', "23")));
    assert_eq!(chr('1').parse("023"), None);
    assert_eq!(digit().parse("123"), Some(('1', "23")));
    assert_eq!(digit().parse("abc"), None);
    assert_eq!(alpha().parse("abc"), Some(('a', "bc")));
    assert_eq!(alpha().parse("123"), None);
    assert_eq!(sat(|x| x == '$').parse("$100"), Some(('$', "100")));
    assert_eq!(sat(|x| x == '$').parse("^100"), None);
    assert_eq!(
        var().parse("abac_123,,,"),
        Some((String::from("abac_123"), ",,,"))
    );
    assert_eq!(
        letter().parse("abc123,hehe"),
        Some((String::from("abc123"), ",hehe"))
    );
}
