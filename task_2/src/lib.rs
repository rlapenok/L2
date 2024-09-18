mod unpack;

#[cfg(test)]
mod test {

    use crate::unpack::{unpack, UnpackErrors};
    #[test]
    fn test_1() {
        assert_eq!("aaaabccddddde".to_owned(), unpack("a4bc2d5e").unwrap());
        assert_eq!("abcd".to_owned(), unpack("abcd").unwrap());
        assert_eq!("", unpack("").unwrap());
        assert_eq!(Err(UnpackErrors::IsNumber), unpack("45"));
        assert_eq!(Err(UnpackErrors::InvalidInput), unpack("4s5c"));
    }
}
