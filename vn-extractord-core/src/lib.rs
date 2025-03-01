//! Crate to extract trade fill events from solana using the phoenix dex software development kit

/// Add some numbers
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[cfg(test)]
mod test {
    use super::add;

    #[test]
    fn simple_test() {
        assert_eq!(add(2, 2), 4);
    }
}
