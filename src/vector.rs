#![allow(dead_code)]

mod custom_vector {
    pub const VECTOR_LENGTH: usize = 3;

    pub struct Vector {
        values: [i32; VECTOR_LENGTH],
    }

    impl Vector {
        pub fn default() -> Self {
            Self {
                values: [0; VECTOR_LENGTH],
            }
        }

        pub fn new(x: [i32; VECTOR_LENGTH]) -> Self {
            Self { values: x }
        }

        pub fn get_value(&self) -> [i32; VECTOR_LENGTH] {
            self.values
        }

        pub fn vector_sum(a: Vector, b: Vector) -> Vector {
            Vector::new([
                a.get_value()[0] + b.get_value()[0],
                a.get_value()[1] + b.get_value()[1],
                a.get_value()[2] + b.get_value()[2],
            ])
        }

        pub fn scalar_sum(a: Vector, b: Vector) -> i32 {
            let mut sum = 0;
            for i in 0..VECTOR_LENGTH {
                sum += a.get_value()[i] + b.get_value()[i];
            }
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::custom_vector::Vector;

    #[test]
    fn default_vector() {
        assert_eq!([0, 0, 0], Vector::default().get_value());
    }

    #[test]
    fn new_vector() {
        assert_eq!([1, 2, 3], Vector::new([1, 2, 3]).get_value());
    }

    #[test]
    fn vector_adding() {
        assert_eq!(
            [0, 0, 0],
            Vector::vector_sum(Vector::default(), Vector::default()).get_value()
        );
        assert_eq!(
            [3, 3, 3],
            Vector::vector_sum(Vector::new([1, 1, 1]), Vector::new([2, 2, 2])).get_value()
        );
        assert_eq!(
            [-3, -1, -5],
            Vector::vector_sum(Vector::new([-10, 0, 5]), Vector::new([7, -1, -10])).get_value()
        );
    }

    #[test]
    fn scalar_adding() {
        assert_eq!(0, Vector::scalar_sum(Vector::default(), Vector::default()));
        assert_eq!(
            0,
            Vector::scalar_sum(Vector::new([1, 2, 3]), Vector::new([-3, -2, -1]))
        );
        assert_eq!(
            3,
            Vector::scalar_sum(Vector::new([0, 0, 1]), Vector::new([1, 1, 0]))
        )
    }
}
