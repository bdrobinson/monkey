#[cfg(test)]
mod test {
    use crate::code;
    #[test]
    fn test_to_bytes() {
        let bytes = code::OpCodeAndOperands::Constant(65534).to_bytes();
        assert_eq!(bytes, vec![0, 0xFF, 0xFE]);
    }
}
