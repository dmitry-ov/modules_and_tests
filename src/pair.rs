#![allow(dead_code)]

mod pair {
    #[derive(Debug, PartialEq)]
    pub struct Pair {
        item: (i32, i32),
    }

    impl Pair {
        pub fn default_pair() -> Self {
            Self { item: (0, 0) }
        }

        pub fn init(source: (i32, i32)) -> Self {
            Self { item: source }
        }

        pub fn get_value(&self) -> (&i32, &i32) {
            (&self.item.0, &self.item.1)
        }

        pub fn vector_sum(a: Pair, b: Pair) -> Pair {
            Pair::init((a.item.0 + b.item.0, a.item.1 + b.item.1))
        }

        pub fn scalar_sum(a: Pair, b: Pair) -> i32 {
            a.item.0 + a.item.1 + b.item.0 + b.item.1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pair::pair::Pair;

    #[test]
    fn test_default_pair() {
        let empty = Pair::default_pair();
        assert_eq!((&0, &0), empty.get_value());
    }

    #[test]
    fn test_pair_vector_sum() {
        assert_eq!(
            Pair::default_pair(),
            Pair::vector_sum(Pair::default_pair(), Pair::default_pair())
        );
        assert_eq!(
            (&1, &1),
            Pair::vector_sum(Pair::default_pair(), Pair::init((1, 1))).get_value()
        );

        let pair1 = Pair::init((1, 1));
        let pair2 = Pair::init((2, 2));
        assert_eq!((&3, &3), Pair::vector_sum(pair1, pair2).get_value());
    }

    #[test]
    fn test_pair_scalar_sum() {
        let zero_zero = (0, 0);
        let one_one = (1, 1);
        assert_eq!(
            0,
            Pair::scalar_sum(Pair::init(zero_zero), Pair::init(zero_zero))
        );
        assert_eq!(
            2,
            Pair::scalar_sum(Pair::init(zero_zero), Pair::init(one_one))
        );
        assert_eq!(
            4,
            Pair::scalar_sum(Pair::init(one_one), Pair::init(one_one))
        );
    }
}
