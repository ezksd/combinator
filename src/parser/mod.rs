#![allow(dead_code)]
use std::marker::PhantomData;
trait Parser {
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

    fn flat_map<B,F>(&'_ self,f: F) -> FlatMap<Self,B,F>
    where Self: Sized, F: Fn(Self::Output) -> Box<dyn Parser<Input=Self::Input,Output=B>>,{
        FlatMap(self,f,PhantomData)
    }
}

impl <I,O: Clone> Parser<Input=I,Output=O> where Self:Sized{
    fn pure(a: O) -> Pure<I,O>{
        Pure(Box::new(a.clone()),PhantomData)
    }
}

impl <I,O> Parser<Input = I,Output=O> where Self: Sized{
    fn empty() -> Empty<I,O>{
        Empty(PhantomData)
    }
}

struct Map<'a,P, B, F>(&'a P, F, PhantomData<B>)
where
    P: Parser,
    F: Fn(P::Output) -> B;

impl<P, B, F> Parser for Map<'_,P, B, F>
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

struct Pure<I, A>(Box<A>, PhantomData<I>);

impl<I, A: Clone> Parser for Pure<I, A> {
    type Input = I;
    type Output = A;
    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        Some(((*self.0).clone(), input))
    }
}

fn pure<A: Clone, I>(a: A) -> Pure<I, A> {
    Pure(Box::new(a.clone()), PhantomData)
}

struct Empty<I, O>(PhantomData<(I, O)>);
impl<I, O> Parser for Empty<I, O> {
    type Input = I;
    type Output = O;
    fn parse(&self, _: Self::Input) -> Option<(Self::Output, Self::Input)> {
        None
    }
}

fn empty<I, O>() -> Empty<I, O> {
    Empty(PhantomData)
}


struct FlatMap<'a,P, B, F>(&'a P, F, PhantomData<B>)
where
    P: Parser,
    F: Fn(P::Output) -> Box<dyn Parser<Input = P::Input, Output = B>>;

impl<P,B,F> Parser for FlatMap<'_,P,B,F>
where P: Parser, F: Fn(P::Output) -> Box<dyn Parser<Input = P::Input,Output = B>>
{
    type Input = P::Input;
    type Output = B;
    fn parse(&self,input: Self::Input) -> Option<(Self::Output,Self::Input)>{
        self.0.parse(input).and_then(|(o1,i1)| (self.1)(o1).parse(i1))
    }
}