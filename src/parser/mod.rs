#![allow(dead_code)]
use std::marker::PhantomData;

mod chars;
pub trait Parser {
    type Input;
    type Output;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)>;
    fn map<B, F>(&'_ self, f: F) -> Map<Self, B, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
    {
        Map(self, f, PhantomData)
    }

    fn flat_map<B, F>(&'_ self, f: F) -> FlatMap<Self, B, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> Box<dyn Parser<Input = Self::Input, Output = B>>,
    {
        FlatMap(self, f, PhantomData)
    }

    fn many(&self) -> Many<Self>
    where
        Self: Sized,
    {
        Many(self)
    }
}

pub struct Map<'a, P, B, F>(&'a P, F, PhantomData<B>);

impl<P, B, F> Parser for Map<'_, P, B, F>
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

pub struct Pure<I, A>(Box<A>, PhantomData<I>);
pub fn pure<I, A: Clone>(a: A) -> Pure<I, A> {
    Pure(Box::new(a.clone()), PhantomData)
}

impl<I, A: Clone> Parser for Pure<I, A> {
    type Input = I;
    type Output = A;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        Some(((*self.0).clone(), input))
    }
}

pub struct Empty<I, O>(PhantomData<(I, O)>);
pub fn empty<I, O>() -> Empty<I, O> {
    Empty(PhantomData)
}
impl<I, O> Parser for Empty<I, O> {
    type Input = I;
    type Output = O;
    fn parse(&self, _: Self::Input) -> Option<(Self::Output, Self::Input)> {
        None
    }
}

pub struct FlatMap<'a, P, B, F>(&'a P, F, PhantomData<B>);

impl<P, B, F> Parser for FlatMap<'_, P, B, F>
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

pub struct Put<I>(I);
impl<I: Clone> Parser for Put<I> {
    type Input = I;
    type Output = ();
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        Some(((), input))
    }
}

pub struct Get<I>(PhantomData<I>);
impl<I: Clone> Parser for Get<I> {
    type Input = I;
    type Output = I;

    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        Some((input.clone(), input))
    }
}

pub fn put<I>(i: I) -> Put<I> {
    Put(i)
}

pub fn get<I>() -> Get<I> {
    Get(PhantomData)
}

pub struct Many<'a, P>(&'a P);

impl<'a, P, I: Clone, O> Parser for Many<'a, P>
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
        return Some((v, t.clone()));
    }
}

//pub fn many<p>
