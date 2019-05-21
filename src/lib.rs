#[macro_use]
pub mod parser;
pub mod json;
#[cfg(test)]
pub mod test;
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
