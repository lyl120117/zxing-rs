#[derive(Debug)]
pub enum Error {
    IllegalArgumentException(String),
    ArithmeticException(String),
    WriterException(String),
}
