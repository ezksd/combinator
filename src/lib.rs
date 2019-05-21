pub mod json;
pub mod parser;
#[cfg(test)]
pub mod test;
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
