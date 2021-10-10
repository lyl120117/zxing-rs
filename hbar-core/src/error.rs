#[derive(Debug)]
pub enum Error {
    IllegalArgumentException(String),
    ArithmeticException(String),
    WriterException(String),
    ImageErrorException(String),
    UnsupportedOperationException(String),
}
