use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum UnpackErrors {
    IsNumber,
    InvalidInput,
}
impl Display for UnpackErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Inoput data cannot be a number")
    }
}
impl Error for UnpackErrors {}

#[allow(dead_code)]
pub fn unpack(data: &str) -> Result<String, UnpackErrors> {
    let mut result = String::new();
    if data.parse::<i32>().is_ok() {
        return Err(UnpackErrors::IsNumber);
    }

    let mut chars = data.chars().peekable();
    while let Some(char) = chars.next() {
        if char.is_ascii_digit() {
            return Err(UnpackErrors::InvalidInput);
        } else if let Some(next_char) = chars.peek() {
            if let Some(count) = next_char.to_digit(10) {
                (0..count).for_each(|_| result.push(char));
                chars.next();
            } else {
                result.push(char)
            }
        } else {
            result.push(char)
        }
    }

    Ok(result)
}
