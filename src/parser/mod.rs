#![allow(dead_code)]
use std::marker::PhantomData;

mod chars;
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
        Box::new(Pure(Box::new($x.clone()),PhantomData))
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
        Box::new(Many($e).flat_map(|v|{
            if v.is_empty() {
                empty!()
            }else{
                pure!(v)
            }
        }))
    };
}



pub fn and<'a,I,O>(p:&'a dyn Parser<Input=I,Output=O>,q: &'a dyn Parser<Input=I,Output=O>) -> And<'a,I,O>{
    And(p,q)
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

pub struct And<'a,I,O>(&'a dyn Parser<Input=I,Output=O>,&'a dyn Parser<Input=I,Output=O>);
impl <'a,I: Clone,O> Parser for And<'a,I,O>
{
    type Input = I;
    type Output = O;

    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0.parse(input.clone()).and(self.1.parse(input))
    }
}

pub struct Or<'a,I,O>(&'a dyn Parser<Input=I,Output=O>,&'a dyn Parser<Input=I,Output=O>);
impl <'a,I: Clone,O> Parser for Or<'a,I,O>
{
    type Input = I;
    type Output = O;

    fn parse(&self, input: Self::Input) -> Option<(Self::Output, Self::Input)> {
        self.0.parse(input.clone()).or_else(|| self.1.parse(input))
    }
}

pub struct All<'a,I,O>(Vec<&'a dyn Parser<Input=I,Output=O>>);
impl <'a,I: Clone,O> Parser for All<'a,I,O>{
    type Input = I;
    type Output = Vec<O>;
    fn parse(&self,input: Self::Input) -> Option<(Self::Output,Self::Input)>{
        let mut v = Vec::new();
        let mut t = input;
        for x in self.0.iter(){
            match x.parse(t){
                Some((o,i)) => {
                    v.push(o);
                    t = i;
                },
                None => return None
            }
        }
        Some((v,t.clone()))
    }
}

pub struct Any<'a,I,O>(Vec<&'a dyn Parser<Input=I,Output=O>>);
impl <'a,I: Clone,O> Parser for Any<'a,I,O>{
    type Input = I;
    type Output = O;
    fn parse(&self,input: Self::Input) -> Option<(Self::Output,Self::Input)>{
        for x in self.0.iter(){
            match x.parse(input.clone()){
                Some((o,i)) => {
                    return Some((o,i))
                },
                None => return None
            }
        }
        None
    }
}
