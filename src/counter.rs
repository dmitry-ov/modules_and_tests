#[allow(dead_code)]

pub mod counter {
    pub type SignedCounter = isize;
    pub type UnsignedCounter = usize;

    pub fn default_signed_counter() -> SignedCounter {
        0
    }

    pub fn default_unsigned_counter() -> UnsignedCounter {
        0
    }

    pub fn next_signed(counter: SignedCounter) -> SignedCounter {
        counter + 1
    }

    pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
        counter + 1
    }

    pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
        counter - 1
    }
}

#[cfg(test)]
mod tests {

    use crate::counter::counter::{
        default_signed_counter, default_unsigned_counter, next_signed, next_unsigned, prev_signed,
    };

    #[test]
    fn test_prev_signed() {
        assert_eq!(-1001, prev_signed(-1000));
        assert_eq!(-2, prev_signed(-1));
        assert_eq!(-1, prev_signed(0));
        assert_eq!(0, prev_signed(1));
        assert_eq!(1, prev_signed(2));
        assert_eq!(99, prev_signed(100));
    }

    #[test]
    fn test_next_unsigned() {
        assert_eq!(1, next_unsigned(0));
        assert_eq!(2, next_unsigned(1));
        assert_eq!(99, next_unsigned(98));
    }

    #[test]
    fn test_next_signed() {
        assert_eq!(1, next_signed(0));
        assert_eq!(2, next_signed(1));
        assert_eq!(0, next_signed(-1));
    }

    #[test]
    fn test_counter_default() {
        assert_eq!(0, default_signed_counter());
        assert_eq!(0, default_unsigned_counter());
    }
}
