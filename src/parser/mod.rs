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
}

pub fn pure<I, A: Clone>(a: A) -> Pure<I, A> {
    Pure(Box::new(a.clone()), PhantomData)
}

pub fn empty<I, O>() -> Empty<I, O> {
    Empty(PhantomData)
}

pub fn many<P>(p: &P) -> Many1<P> {
    Many1(p)
}

pub fn some<P>(p: &P) -> Many<P>{
    Many(p)
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
        Some((v, t.clone()))
    }
}

pub struct Many1<'a,P>(&'a P);
impl <'a,P,I: Clone,O> Parser for Many1<'a,P>
where P: Parser<Input=I,Output = O>,
{
    type Input = P::Input;
    type Output = Vec<P::Output>;
    fn parse(&self,input: Self::Input) -> Option<(Self::Output,Self::Input)>{
        let mut v = Vec::new();
        let mut t = input;
        while let Some((o, i)) = self.0.parse(t.clone()) {
            v.push(o);
            t = i
        }
        if v.len() ==0 {
            None
        }else {
            Some((v,t.clone()))
        }
    }
}