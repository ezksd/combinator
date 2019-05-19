// #[allow(dead_code)]
// use parser::chars::*;
// use parser::*;
// use crate::*;
// use JsonValue::*;
// enum JsonValue{
//     JsonObject(Box<JsonValue>,Box<JsonValue>), // the first parameter is JsonString
//     JsonArray(Vec<JsonValue>),
//     JsonString(String),
//     JsonInteger(i32),
//     JsonFloat(f32),
//     JsonBool(bool),
//     JsonNull(),
// }

// // fn json_float<'a>() -> impl Parser<Input = &'a str, Output = JsonValue>{

// // }

// fn json_bool<'a>() -> impl Parser<Input = &'a str,Output=JsonValue>{
//     letter("true").map(|_| JsonBool(true))
// }

// fn json_null<'a>() -> impl Parser<Input = &'a str,Output=JsonValue>{
//     letter("null").map(|_| JsonNull())
// }
