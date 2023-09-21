#![allow(dead_code)]

mod custom_counters {
    pub struct SignedCounter {
        item: isize,
    }

    pub struct UnsignedCounter {
        item: usize,
    }

    impl SignedCounter {
        pub fn default_signed() -> Self {
            Self { item: 0 }
        }

        pub fn new(x: isize) -> Self {
            Self { item: x }
        }

        pub fn get_value(&self) -> isize {
            self.item
        }

        pub fn next_signed(&mut self) {
            self.item += 1;
        }

        pub fn prev_signed(&mut self) {
            self.item -= 1;
        }
    }

    impl UnsignedCounter {
        pub fn default_unsigned() -> UnsignedCounter {
            Self { item: 0 }
        }

        pub fn new(x: usize) -> Self {
            Self { item: x }
        }

        pub fn get_value(&self) -> usize {
            self.item
        }

        pub fn next_unsigned(&mut self) {
            self.item += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::counter::custom_counters::{SignedCounter, UnsignedCounter};

    #[test]
    fn signed_default() {
        assert_eq!(0, SignedCounter::default_signed().get_value());
    }

    #[test]
    fn unsigned_default() {
        assert_eq!(0, UnsignedCounter::default_unsigned().get_value());
    }

    #[test]
    fn signed_new() {
        assert_eq!(-1, SignedCounter::new(-1).get_value());
        assert_eq!(0, SignedCounter::new(0).get_value());
        assert_eq!(5, SignedCounter::new(5).get_value());
    }

    #[test]
    fn unsigned_new() {
        assert_eq!(0, UnsignedCounter::new(0).get_value());
        assert_eq!(1, UnsignedCounter::new(1).get_value());
    }

    #[test]
    fn next_signed() {
        let mut signed_counter = SignedCounter::default_signed();
        signed_counter.next_signed();
        assert_eq!(1, signed_counter.get_value());

        let mut counter = SignedCounter::new(-1);
        counter.next_signed();
        assert_eq!(0, counter.get_value());
    }

    #[test]
    fn prev_signed() {
        let mut signed_counter = SignedCounter::default_signed();
        signed_counter.prev_signed();
        assert_eq!(-1, signed_counter.get_value());

        let mut counter = SignedCounter::new(1);
        counter.prev_signed();
        assert_eq!(0, counter.get_value());
    }

    #[test]
    fn next_unsigned() {
        let mut unsigned_counter = UnsignedCounter::default_unsigned();
        unsigned_counter.next_unsigned();
        assert_eq!(1, unsigned_counter.get_value());

        let mut counter = UnsignedCounter::new(1);
        counter.next_unsigned();
        assert_eq!(2, counter.get_value());
    }
}
