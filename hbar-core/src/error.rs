#[derive(Debug)]
pub enum Error {
    IllegalArgumentException(String),
    ArithmeticException(String),
    WriterException(String),
    ImageErrorException(String),
    UnsupportedOperationException(String),
    TryFromIntError(String),
    Infallible(String),
    NotFoundException(String),
    ReedSolomonException(String),
    IllegalStateException(String),
    FormatException(String),
}
pub type ResultError<T> = Result<T, Error>;
