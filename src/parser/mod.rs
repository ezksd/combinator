use std::marker::PhantomData;
pub trait Parser<I, O> {
    fn parse(&self, input: I) -> Option<(O, I)>;
    fn map<B, F>(self, f: F) -> Map<Self, F, O>
    where
        Self: Sized,
        F: Fn(O) -> B,
    {
        Map(self, f, PhantomData)
    }

    fn flat_map<B, F, Q>(self, f: F) -> FlatMap<Self, F, O>
    where
        Self: Sized,
        F: Fn(O) -> Q,
        Q: Parser<I, B>,
    {
        FlatMap(self, f, PhantomData)
    }

    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        F: Fn(&O) -> bool,
    {
        Filter(self, f)
    }
}

pub struct Map<P, F, A>(P, F, PhantomData<A>);

impl<I, O, B, P: Parser<I, O>, F: Fn(O) -> B> Parser<I, B> for Map<P, F, O> {
    fn parse(&self, input: I) -> Option<(B, I)> {
        self.0.parse(input).map(|(o, i)| ((self.1)(o), i))
    }
}

pub struct FlatMap<P, F, A>(P, F, PhantomData<A>);

impl<I, O, B, P: Parser<I, O>, Q: Parser<I, B>, F: Fn(O) -> Q> Parser<I, B> for FlatMap<P, F, O> {
    fn parse(&self, input: I) -> Option<(B, I)> {
        self.0
            .parse(input)
            .and_then(|(o1, i1)| (self.1)(o1).parse(i1))
    }
}

pub struct Filter<P, F>(P, F);

impl<I, A, P: Parser<I, A>, F: Fn(&A) -> bool> Parser<I, A> for Filter<P, F> {
    fn parse(&self, input: I) -> Option<(A, I)> {
        self.0.parse(input).filter(|(a, _)| self.1(a))
    }
}

impl<I, O, F> Parser<I, O> for F
where
    F: Fn(I) -> Option<(O, I)>,
{
    fn parse(&self, input: I) -> Option<(O, I)> {
        self(input)
    }
}

pub struct Pure<A>(A);

impl<I, A: Clone> Parser<I, A> for Pure<A> {
    fn parse(&self, input: I) -> Option<(A, I)> {
        Some((self.0.clone(), input))
    }
}

pub struct Empty;
impl<I, O> Parser<I, O> for () {
    fn parse(&self, _: I) -> Option<(O, I)> {
        None
    }
}

pub fn throw<I, O, P>(p: P) -> impl Parser<I, ()>
where
    P: Parser<I, O>,
{
    p.map(|_| ())
}

pub fn and<I, A, B, P, Q>(p: P, q: Q) -> And<P, Q>
where
    P: Parser<I, A>,
    Q: Parser<I, B>,
{
    And(p, q)
}

pub struct And<P, Q>(P, Q);
impl<I, A, B, P, Q> Parser<I, (A, B)> for And<P, Q>
where
    P: Parser<I, A>,
    Q: Parser<I, B>,
{
    fn parse(&self, input: I) -> Option<((A, B), I)> {
        self.0
            .parse(input)
            .and_then(|(a, i)| self.1.parse(i).map(|(b, i1)| ((a, b), i1)))
    }
}

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub fn or<I, A, B, P, Q>(p: P, q: Q) -> Or<P, Q> {
    Or(p, q)
}

pub struct Or<P, Q>(P, Q);
impl<I: Clone, A, B, P, Q> Parser<I, Either<A, B>> for Or<P, Q>
where
    P: Parser<I, A>,
    Q: Parser<I, B>,
{
    fn parse(&self, input: I) -> Option<(Either<A, B>, I)> {
        self.0
            .parse(input.clone())
            .map(|(o, i)| (Either::Left(o), i))
            .or_else(|| self.1.parse(input).map(|(o, i)| (Either::Right(o), i)))
    }
}

pub fn opt<I: Clone, O, P>(p: P) -> impl Parser<I, Option<O>>
where
    P: Parser<I, O>,
{
    Or(p, Pure(())).map(|e| match e {
        Either::Left(e) => Some(e),
        Either::Right(()) => None,
    })
}

pub fn many<I, A, P>(p: P) -> Many<P>
where
    P: Parser<I, A>,
{
    Many(p)
}

pub fn many1<I: Clone, A, P>(p: P) -> impl Parser<I, Vec<A>>
where
    P: Parser<I, A>,
{
    Many(p).filter(|v| v.len() > 0)
}

pub struct Many<P>(P);
impl<P, I: Clone, O> Parser<I, Vec<O>> for Many<P>
where
    P: Parser<I, O>,
{
    fn parse(&self, input: I) -> Option<(Vec<O>, I)> {
        let mut v = Vec::new();
        let mut t = input;
        while let Some((o, i)) = self.0.parse(t.clone()) {
            v.push(o);
            t = i
        }
        Some((v, t))
    }
}

pub fn lazy<P, F>(f: F) -> Lazy<F>
where
    F: Fn() -> P,
{
    Lazy(f)
}

#[macro_export]
macro_rules! all {
    ($($e:expr),+) => {
        {
            let mut v:Vec<Box<dyn Parser<Input=_,Output=_>>> = Vec::new();
            $(v.push(Box::new($e));)*
            All(v)
        }
    };
}

#[macro_export]
macro_rules! any {
    ($($e:expr),+) => {
        {
            let mut v = Vec::new();
            $(
                let p: Box<dyn Parser<_,_>> = Box::new($e);
                v.push(p);
            )*
            Any(v)
        }
    };
}

pub fn repeat<P>(p: P, i: i32) -> Repeat<P> {
    Repeat(p, i)
}

pub struct All<I, O>(pub Vec<Box<dyn Parser<I, O>>>);
impl<I, O> Parser<I, Vec<O>> for All<I, O> {
    fn parse(&self, input: I) -> Option<(Vec<O>, I)> {
        let mut v = Vec::new();
        let mut t = input;
        for x in self.0.iter() {
            match x.parse(t) {
                Some((o, i)) => {
                    v.push(o);
                    t = i;
                }
                None => return None,
            }
        }
        Some((v, t))
    }
}

pub struct Any<I, O>(pub Vec<Box<dyn Parser<I, O>>>);
impl<I: Clone, O> Parser<I, O> for Any<I, O> {
    fn parse(&self, input: I) -> Option<(O, I)> {
        for x in self.0.iter() {
            if let Some((o, i)) = x.parse(input.clone()) {
                return Some((o, i));
            }
        }
        None
    }
}

pub struct Opt<P>(P);
impl<I: Clone, O, P> Parser<I, Option<O>> for Opt<P>
where
    P: Parser<I, O>,
{
    fn parse(&self, input: I) -> Option<(Option<O>, I)> {
        match self.0.parse(input.clone()) {
            Some((o, i)) => Some((Some(o), i)),
            None => Some((None, input)),
        }
    }
}

pub struct Repeat<P>(P, i32);
impl<I, O, P> Parser<I, Vec<O>> for Repeat<P>
where
    P: Parser<I, O>,
{
    fn parse(&self, input: I) -> Option<(Vec<O>, I)> {
        let mut n = self.1;
        let mut results = Vec::new();
        let mut t = input;
        while n > 0 {
            match self.0.parse(t) {
                Some((o, i)) => {
                    results.push(o);
                    t = i;
                    n -= 1;
                }
                None => return None,
            }
        }
        Some((results, t))
    }
}

pub struct Throw<O, P>(P, PhantomData<O>);
impl<I, O, P> Parser<I, ()> for Throw<O, P>
where
    P: Parser<I, O>,
{
    fn parse(&self, input: I) -> Option<((), I)> {
        self.0.parse(input).map(|(_, i)| ((), i))
    }
}

pub struct Lazy<F>(F);
impl<I, O, F, P> Parser<I, O> for Lazy<F>
where
    P: Parser<I, O>,
    F: Fn() -> P,
{
    fn parse(&self, input: I) -> Option<(O, I)> {
        let p = self.0();
        p.parse(input)
    }
}

#[macro_export]
macro_rules! prefix {
    ($p:expr,$q:expr) => {
        and($p, $q).map(|(_, x)| x)
    };
    ($p1:expr, $p2:expr, $x:expr) => {
        prefix!(and($p1, $p2), $x)
    };
}

#[macro_export]
macro_rules! suffix {
    ($p:expr,$q:expr) => {
        and($p, $q).map(|(x, _)| x)
    };
}

#[macro_export]
macro_rules! infix {
    ($p:expr, $i:expr, $q:expr) => {
        and($p, prefix!($i, $q))
    };
}

#[macro_export]
macro_rules! around {
    ($p:expr,$x:expr,$q:expr) => {
        prefix!($p, suffix!($x, $q))
    };
}
pub mod chars;
