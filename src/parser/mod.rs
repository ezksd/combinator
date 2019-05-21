use std::marker::PhantomData;
pub trait Parser {
    type Input;
    type Output;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)>;
    fn map<B, F>(self, f: F) -> Map<Self, B, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
    {
        Map(self, f, PhantomData)
    }

    fn flat_map<B, F>(self, f: F) -> FlatMap<Self, B, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> Box<dyn Parser<Input = Self::Input, Output = B>>,
    {
        FlatMap(self, f, PhantomData)
    }

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn throw(self) -> Throw<Self>
    where
        Self: Sized,
    {
        Throw(self)
    }
}

pub fn pure<I, O: Clone>(o: O) -> Box<Pure<I, O>> {
    Pure(Box::new(o.clone()), PhantomData).boxed()
}

pub fn empty<I, O>() -> Box<Empty<I, O>> {
    Empty(PhantomData).boxed()
}

pub fn many<P>(p: P) -> Many<P> {
    Many(p)
}

pub fn many1<P>(p: P) -> Many1<P> {
    Many1(p)
}

pub fn lazy<P>(f: fn() -> P) -> Lazy<P> {
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
            let mut v:Vec<Box<dyn Parser<Input=_,Output=_>>> = Vec::new();
            $(v.push(Box::new($e));)*
            Any(v)
        }
    };
}

pub fn and<P, Q>(p: P, q: Q) -> And<P, Q> {
    And(p, q)
}

pub fn or<P, Q>(p: P, q: Q) -> Or<P, Q> {
    Or(p, q)
}

pub fn opt<P>(p: P) -> Opt<P> {
    Opt(p)
}

pub fn repeat<P>(p: P, i: i32) -> Repeat<P> {
    Repeat(p, i)
}

#[macro_export]
macro_rules! prefix {
    ($p:expr,$q:expr) => {
        and($p, $q).map(|(_, x)| x)
    };
    ($p1:expr, $p2:expr, $x:expr) => {
        and(and($p1, $p2), $x).map(|(_, x)| x)
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
pub struct Map<P, B, F>(P, F, PhantomData<B>);
impl<P, B, F> Parser for Map<P, B, F>
where
    P: Parser,
    F: Fn(P::Output) -> B,
{
    type Input = P::Input;
    type Output = B;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0.parse(input).map(|(o, i)| ((self.1)(o), i))
    }
}

pub struct FlatMap<P, B, F>(P, F, PhantomData<B>);
impl<P, B, F> Parser for FlatMap<P, B, F>
where
    P: Parser,
    F: Fn(P::Output) -> Box<dyn Parser<Input = P::Input, Output = B>>,
{
    type Input = P::Input;
    type Output = B;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0
            .parse(input)
            .and_then(|(o1, i1)| (self.1)(o1).parse(i1))
    }
}

pub struct Pure<I, A>(Box<A>, PhantomData<I>);

impl<I, A: Clone> Parser for Pure<I, A> {
    type Input = I;
    type Output = A;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        Some(((*self.0).clone(), input))
    }
}

pub struct Empty<I, O>(PhantomData<(I, O)>);

impl<I, O> Parser for Empty<I, O> {
    type Input = I;
    type Output = O;
    fn parse(&self, _: Self::Input) -> Option<(Self::Output, Self::Input)> {
        None
    }
}

pub struct Many<P>(P);
impl<P, I: Clone, O> Parser for Many<P>
where
    P: Parser<Input = I, Output = O>,
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        let mut v = Vec::new();
        let mut t = input;
        while let Some((o, i)) = self.0.parse(t.clone()) {
            v.push(o);
            t = i
        }
        Some((v, t.clone()))
    }
}

pub struct Many1<P>(P);
impl<P, I: Clone, O> Parser for Many1<P>
where
    P: Parser<Input = I, Output = O>,
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        let mut v = Vec::new();
        let mut t = input;
        while let Some((o, i)) = self.0.parse(t.clone()) {
            v.push(o);
            t = i
        }

        if v.is_empty() {
            None
        } else {
            Some((v, t.clone()))
        }
    }
}

pub struct All<I, O>(pub Vec<Box<dyn Parser<Input = I, Output = O>>>);
impl<'a, I: Clone, O> Parser for All<I, O> {
    type Input = I;
    type Output = Vec<O>;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
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
        Some((v, t.clone()))
    }
}

pub struct Any<I, O>(pub Vec<Box<dyn Parser<Input = I, Output = O>>>);
impl<'a, I: Clone, O> Parser for Any<I, O> {
    type Input = I;
    type Output = O;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        for x in self.0.iter() {
            if let Some((o, i)) = x.parse(input.clone()) {
                return Some((o, i));
            }
        }
        None
    }
}

pub struct And<P, Q>(P, Q);
impl<I, A, B, P, Q> Parser for And<P, Q>
where
    P: Parser<Input = I, Output = A>,
    Q: Parser<Input = I, Output = B>,
{
    type Input = I;
    type Output = (A, B);
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0
            .parse(input)
            .and_then(|(o1, i)| self.1.parse(i).map(|(o2, i1)| ((o1, o2), i1)))
    }
}
use crate::Either;
use crate::Either::{Left, Right};
pub struct Or<P, Q>(P, Q);
impl<I: Clone, A, B, P, Q> Parser for Or<P, Q>
where
    P: Parser<Input = I, Output = A>,
    Q: Parser<Input = I, Output = B>,
{
    type Input = I;
    type Output = Either<A, B>;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0
            .parse(input.clone())
            .map(|(o, i)| (Left(o), i))
            .or_else(|| self.1.parse(input).map(|(o, i)| (Right(o), i)))
    }
}

pub struct Opt<P>(P);
impl<P> Parser for Opt<P>
where
    P: Parser,
    P::Input: Clone,
{
    type Input = P::Input;
    type Output = Option<P::Output>;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        match self.0.parse(input.clone()) {
            Some((o, i)) => Some((Some(o), i)),
            None => Some((None, input)),
        }
    }
}

pub struct Repeat<P>(P, i32);
impl<P> Parser for Repeat<P>
where
    P: Parser,
    P::Input: Clone,
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
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

pub struct Throw<P>(P);
impl<P> Parser for Throw<P>
where
    P: Parser,
{
    type Input = P::Input;
    type Output = ();
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0.parse(input).map(|(_, i)| ((), i))
    }
}

pub struct Lazy<P>(fn() -> P);
impl<P> Parser for Lazy<P>
where
    P: Parser,
{
    type Input = P::Input;
    type Output = P::Output;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        let p = self.0();
        p.parse(input)
    }
}

pub mod chars;
