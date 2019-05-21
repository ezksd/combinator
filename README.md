# simple rust implement parsercombinator library

# HOW TO USE
1. build a parser
2. call the `parse()` method
3. done

more detail are in the module `parser` and module `parser::char`

# example
```rust
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
    assert_eq!(letter().parse("abc123,hehe"),Some((String::from("abc123"),",hehe")));
```

# more complicate example
```rust
pub enum JsonValue {
    JsonObject(HashMap<String, Box<JsonValue>>), // the first parameter is JsonString
    JsonArray(Vec<JsonValue>),
    JsonString(String),
    JsonInteger(i32),
    JsonFloat(f32),
    JsonBool(bool),
    JsonNull
}
pub fn json_value<'a>() -> impl Parser<Input = &'a str,Output = JsonValue>{
    any!(json_object(),json_array(),json_string().map(JsonString),json_number(),json_bool(),json_null())
} 

fn json_object<'a>() -> impl Parser<Input = &'a str,Output = JsonValue>{
    fn entry<'a> () -> impl Parser<Input = &'a str,Output = (String,JsonValue)>{
        prefix!(spaces(),infix!(json_string(),prefix!(spaces(),chr(':')),lazy(json_value)))
    }

    fn map<'a>() -> impl Parser<Input = &'a str, Output = JsonValue>{
        // opt(and(entry(),many(prefix!(prefix!(spaces(),chr(',')),entry()))).map(|(x,xs)| {
        opt(and(entry(),many(prefix!(chr(','),entry()))).map(|(x,xs)| {
            let mut m = HashMap::new();
            m.insert(x.0, Box::new(x.1));
            for p in xs {
                m.insert(p.0,Box::new(p.1));
            }
            m
        })).map(|o| match o {
            Some(m) =>  JsonObject(m),
            None => JsonObject(HashMap::new())
        })
    }
    around!(prefix!(spaces(),chr('{')),map(),prefix!(spaces(),chr('}')))
}

fn json_array<'a>() -> impl Parser<Input = &'a str,Output = JsonValue>{
    fn value<'a>() -> impl Parser<Input = &'a str,Output = JsonValue>{
        opt(and(lazy(json_value), many(prefix!(prefix!(spaces(),chr(',')),lazy(json_value))))).map(|o| match o{
            Some((x,xs)) => {
                let mut v = vec![x];
                for x in xs {
                    v.push(x)
                }
                JsonArray(v)
            },
            None => JsonArray(Vec::new())
        })
        // prefix!(spaces(),)
    }
    around!(prefix!(spaces(),chr('[')),prefix!(spaces(),value()),prefix!(spaces(),chr(']')))
}

fn json_string<'a>() -> impl Parser<Input = &'a str, Output = String> {
    fn control<'a>() -> impl Parser<Input = &'a str, Output = char>{
        chr('\\').flat_map(|_|{
            any!(item().flat_map(|x|{
                match x {
                    '"' | '\\' | '/'  => pure(x),
                    'b' => pure('\x08'),
                    't' => pure('\x12'),
                    'n' => pure('\x10'),
                    'f' => pure('\x13'),
                    'r' => pure('\x09'),
                    _ => empty()
                }
            }),chr('u').flat_map(|_|{
                repeat(digit(), 4).map(|v|{
                    let mut t = 0;
                    for i in v.iter(){
                        t = t * 10 + i;
                    }
                    std::char::from_u32(t).unwrap()
                }).boxed()
            })).boxed()
        })
    }

    fn other<'a>() -> impl Parser<Input  = &'a str,Output = char>{
        item().flat_map(|x| match x {
            '"' | '\\' => empty(),
            _ => pure(x)
        })
    }
    prefix!(spaces(),chr('"')).flat_map(|_|{
        many(any!(control(),other())).boxed()
    }).flat_map(|v|{
        chr('"').map(move|_| collect(&v)).boxed()
    })
}

fn json_number<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    fn flag<'a>() -> impl Parser<Input = &'a str, Output = i32> {
        opt(chr('-')).map(|x| match x {
            Some(_) => -1,
            None => 1,
        })
    }

    fn int<'a>() -> impl Parser<Input = &'a str, Output = u32> {
        or(chr('0'), number()).map(|x| match x {
            Left(_) => 0,
            Right(i) => i,
        })
    }

    fn float<'a>() -> impl Parser<Input = &'a str, Output = Option<f32>> {
        opt(chr('.').flat_map(|_| {
            many1(digit())
                .map(|mut v| {
                    let mut f: f32 = 0 as f32;
                    while let Some(i) = v.pop() {
                        f = f / 10.0 + i as f32
                    }
                    f / 10.0
                })
                .boxed()
        }))
    }

    prefix!(spaces(),flag().flat_map(|g| {
        int()
            .flat_map(move |i| {
                float()
                    .map(move |o| match o {
                        Some(f) => JsonFloat(g as f32 * (i as f32 + f)),
                        None => JsonInteger(g * i as i32),
                    })
                    .boxed()
            })
            .boxed()
    }))
}

fn json_bool<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    prefix!(spaces(),or(letter("true"),letter("false")).map(|o| match o {
        Left(_) => JsonBool(true),
        Right(_) => JsonBool(false)
    }))
}

fn json_null<'a>() -> impl Parser<Input = &'a str, Output = JsonValue> {
    prefix!(spaces(),letter("null")).map(|_| JsonNull)
}
```
