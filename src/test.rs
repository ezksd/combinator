use super::parser::chars::*;
use super::parser::*;
#[macro_use]
macro_rules! p_test {
    ($p:expr,$i:expr) => {
        assert_eq!($p.parse($i),None)
    };
    ($p:expr,$i:expr,$t:expr,$o:expr) => {
        assert_eq!($p.parse($i),Some(($t,$o)))
    };
}
#[test]
fn test_char() {
    p_test!(item(),"");
    p_test!(item(),"123",'1', "23");
    p_test!(chr('1'),"123",'1', "23");
    p_test!(chr('1'),"023");
    p_test!(digit(),"123", 1, "23");
    p_test!(digit(),"abc");
    p_test!(number(),"123abc", 123, "abc");
    p_test!(spaces()," \n\r\t",(),"");
    p_test!(alpha(),"abc",'a', "bc");
    p_test!(alpha(),"123");
    p_test!(sat(|x| x == '$'),"$100",'$', "100");
    p_test!(sat(|x| x == '$'),"^100");
    p_test!(var(),"abac_123,,,",String::from("abac_123"), ",,,");
    p_test!(letter("abc123"),"abc123,hehe",String::from("abc123"), ",hehe");
    p_test!(repeat(digit(), 3),"123456",vec![1, 2, 3], "456");
}

#[test]
fn test_json(){
    
}