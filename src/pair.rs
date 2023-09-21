#![allow(dead_code)]

mod custom_pair {
    pub struct Pair {
        item: (i32, i32),
    }

    impl Pair {
        pub fn default_pair() -> Self {
            Self { item: (0, 0) }
        }

        pub fn new(x: i32, y: i32) -> Self {
            Self { item: (x, y) }
        }

        pub fn get_value(&self) -> (i32, i32) {
            self.item
        }

        pub fn vector_sum(a: Pair, b: Pair) -> Pair {
            Pair::new(a.item.0 + b.item.0, a.item.1 + b.item.1)
        }

        pub fn scalar_sum(a: Pair, b: Pair) -> i32 {
            a.item.0 + a.item.1 + b.item.0 + b.item.1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pair::custom_pair::Pair;

    #[test]
    fn test_default_pair() {
        assert_eq!((0, 0), Pair::default_pair().get_value());
        assert_eq!((0, 0), Pair::new(0, 0).get_value());
        assert_eq!((-1, 1), Pair::new(-1, 1).get_value());
    }

    #[test]
    fn test_pair_vector_sum() {
        assert_eq!(
            (0, 0),
            Pair::vector_sum(Pair::default_pair(), Pair::default_pair()).get_value()
        );
        assert_eq!(
            (1, 1),
            Pair::vector_sum(Pair::new(0, 1), Pair::new(1, 0)).get_value()
        );
        assert_eq!(
            (2, 2),
            Pair::vector_sum(Pair::new(1, 1), Pair::new(1, 1)).get_value()
        );
    }

    #[test]
    fn test_pair_scalar_sum() {
        assert_eq!(0, Pair::scalar_sum(Pair::new(0, 0), Pair::new(0, 0)));
        assert_eq!(2, Pair::scalar_sum(Pair::new(0, 0), Pair::new(1, 1)));
        assert_eq!(4, Pair::scalar_sum(Pair::new(1, 1), Pair::new(1, 1)));
    }
}
