#![allow(dead_code)]
use std::marker::PhantomData;
pub mod chars;
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
}

#[macro_export]
macro_rules! pure {
    ($x:expr) => {
        Box::new(Pure(Box::new($x.clone()), PhantomData))
    };
}

#[macro_export]
macro_rules! empty {
    () => {
        Box::new(Empty(PhantomData))
    };
}

#[macro_export]
macro_rules! many {
    ($e:expr) => {
        Box::new(Many($e))
    };
}

#[macro_export]
macro_rules! some {
    ($e:expr) => {
        Box::new(Many($e).flat_map(|v| if v.is_empty() { empty!() } else { pure!(v) }))
    };
}

#[macro_export]
macro_rules! all {
    ($($e:expr),+) => {
        {
            let mut v:Vec<Box<dyn Parser<Input=_,Output=_>>> = Vec::new();
            $(v.push(Box::new($e));)*
            Box::new(All(v))
        }
    };
}

#[macro_export]
macro_rules! any {
    ($($e:expr),+) => {
        {
            let mut v:Vec<Box<dyn Parser<Input=_,Output=_>>> = Vec::new();
            $(v.push(Box::new($e));)*
            // Box::new(Any(v))
            Any(v)
        }
    };
}

#[macro_export]
macro_rules! and {
    ($e1:expr,$e2:expr) => {
        Box::new(And(e1, e2))
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

pub struct All<I, O>(Vec<Box<dyn Parser<Input = I, Output = O>>>);
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

pub struct Any<I, O>(Vec<Box<dyn Parser<Input = I, Output = O>>>);
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

use Either::{Left, Right};
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

pub enum Either<A, B> {
    Left(A),
    Right(B),
}
