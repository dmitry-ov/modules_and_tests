#![allow(dead_code)]

pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {

    use crate::pair::{default_pair, pair_scalar_sum, pair_vector_sum};

    #[test]
    fn test_default_pair() {
        assert_eq!((0, 0), default_pair());
    }

    #[test]
    fn test_pair_vector_sum() {
        assert_eq!(
            default_pair(),
            pair_vector_sum(default_pair(), default_pair())
        );
        let one_one = (1, 1);
        assert_eq!(one_one, pair_vector_sum(default_pair(), one_one));
        assert_eq!((2, 2), pair_vector_sum(one_one, one_one));
    }

    #[test]
    fn test_pair_scalar_sum() {
        let zero_zero = (0, 0);
        let one_one = (1, 1);
        assert_eq!(0, pair_scalar_sum(zero_zero, zero_zero));
        assert_eq!(2, pair_scalar_sum(zero_zero, one_one));
        assert_eq!(4, pair_scalar_sum(one_one, one_one));
    }
}
