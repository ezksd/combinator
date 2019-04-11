#![allow(dead_code)]
use std::marker::PhantomData;
mod chars;
pub trait Parser<I: ?Sized, O>
where
    Self: Sized,
{
    fn parse<'a>(&self, i: &'a I) -> Option<(O, &'a I)>;
    fn map<B, F>(self, f: F) -> Map<I, O, B, Self, F>
    where
        F: Fn(O) -> B,
    {
        Map(self, f, PhantomData)
    }

    fn flat_map<B, F, Q>(self, f: F) -> FlatMap<I, O, B, Self, Q, F>
    where
        Q: Parser<I, B>,
        F: Fn(O) -> Q,
    {
        FlatMap(self, f, PhantomData)
    }

    fn filter<F>(self, f: F) -> Filter<I, O, Self, F>
    where
        F: Fn(&O) -> bool,
    {
        Filter(self, f, PhantomData)
    }

    fn many(self) -> Many<I, O, Self> {
        Many(self, PhantomData)
    }

    // fn Some<F>(self) -> Filter<I, O, Many<I,O,Self>, F>
    // where F: Fn(&O) -> bool
    // {
    //     self.many().filter(|v| v.len != 0)
    // }
}

pub struct Map<I: ?Sized, O, B, P, F>(P, F, PhantomData<(Box<I>, O, B)>)
where
    P: Parser<I, O>,
    F: Fn(O) -> B;

impl<I, O, B, P, F> Parser<I, B> for Map<I, O, B, P, F>
where
    P: Parser<I, O>,
    F: Fn(O) -> B,
{
    fn parse<'a>(&self, i: &'a I) -> Option<(B, &'a I)> {
        self.0.parse(i).map(|(o, i)| ((self.1)(o), i))
    }
}

pub struct FlatMap<I: ?Sized, O, B, P, Q, F>(P, F, PhantomData<(Box<I>, O, B, Q)>)
where
    P: Parser<I, O>,
    Q: Parser<I, B>,
    F: Fn(O) -> Q;

impl<I, O, B, P, Q, F> Parser<I, B> for FlatMap<I, O, B, P, Q, F>
where
    P: Parser<I, O>,
    Q: Parser<I, B>,
    F: Fn(O) -> Q,
{
    fn parse<'a>(&self, i: &'a I) -> Option<(B, &'a I)> {
        self.0.parse(i).and_then(|(o, i1)| {
            let p = (self.1)(o);
            p.parse(i1)
        })
    }
}

pub struct Filter<I: ?Sized, O, P, F>(P, F, PhantomData<(Box<I>, O)>)
where
    P: Parser<I, O>,
    F: Fn(&O) -> bool;

impl<I, O, P, F> Parser<I, O> for Filter<I, O, P, F>
where
    P: Parser<I, O>,
    F: Fn(&O) -> bool,
{
    fn parse<'a>(&self, i: &'a I) -> Option<(O, &'a I)> {
        self.0
            .parse(i)
            .and_then(|(o, i)| if self.1(&o) { Some((o, i)) } else { None })
    }
}

pub struct Many<I: ?Sized, O, P>(P, PhantomData<(Box<I>, O)>)
where
    P: Parser<I, O>;

impl<I, O, P> Parser<I, Vec<O>> for Many<I, O, P>
where
    P: Parser<I, O>,
{
    fn parse<'a>(&self, i: &'a I) -> Option<(Vec<O>, &'a I)> {
        let mut v = Vec::new();
        let mut t = i;
        while let Some((o, i)) = self.0.parse(t) {
            v.push(o);
            t = i;
        }
        Some((v, t))
    }
}
