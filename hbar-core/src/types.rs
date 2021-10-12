use crate::{Error, ResultError};
use std::convert::TryFrom;

pub type Int = i32;
pub type Char = char;
pub type Byte = u8;
pub type Float = f64;

pub fn u32_to_u8(i: u32) -> ResultError<u8> {
    match u8::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting u32: {} to u8!",
            i
        ))),
    }
}

pub fn u32_to_usize(i: u32) -> ResultError<usize> {
    match usize::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting u32: {} to usize!",
            i
        ))),
    }
}

pub fn u32_to_i32(i: u32) -> ResultError<i32> {
    match i32::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting u32: {} to i32!",
            i
        ))),
    }
}

pub fn i32_to_u32(i: i32) -> ResultError<u32> {
    match u32::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting i32: {} to u32!",
            i
        ))),
    }
}

pub fn i32_to_u8(i: i32) -> ResultError<u8> {
    match u8::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting i32: {} to u8!",
            i
        ))),
    }
}

pub fn i32_to_usize(i: i32) -> ResultError<usize> {
    match usize::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting i32: {} to usize!",
            i
        ))),
    }
}

pub fn i32_to_f32(i: i32) -> ResultError<f32> {
    let i = (match i16::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting i32: {} to i16 for f32!",
            i
        ))),
    })?;
    match f32::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting i32: {} to f32!",
            i
        ))),
    }
}

pub fn usize_to_u32(i: usize) -> ResultError<u32> {
    match u32::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting usize: {} to u32!",
            i
        ))),
    }
}

pub fn usize_to_i32(i: usize) -> ResultError<i32> {
    match i32::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::TryFromIntError(format!(
            "Error converting usize: {} to i32!",
            i
        ))),
    }
}

pub fn char_to_u32(i: char) -> ResultError<u32> {
    match u32::try_from(i) {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::Infallible(format!(
            "Error converting char: {} to u32!",
            i
        ))),
    }
}
