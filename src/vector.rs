#![allow(dead_code)]

mod vector {
    pub const VECTOR_LENGTH: usize = 3;

    #[derive(Debug, PartialEq)]
    pub struct Vector {
        values: [i32; VECTOR_LENGTH],
    }

    impl Vector {
        pub fn get_values(&self) -> [i32; VECTOR_LENGTH] {
            self.values
        }

        pub fn set_values(&mut self, a: [i32; VECTOR_LENGTH]) {
            self.values = a
        }

        pub fn new() -> Self {
            Self {
                values: [0; VECTOR_LENGTH],
            }
        }

        pub fn init(a: [i32; VECTOR_LENGTH]) -> Self {
            Self { values: a }
        }

        pub fn vector_sum(&self, x: &Vector) -> Vector {
            let mut result = Vector::new();
            for i in 0..VECTOR_LENGTH {
                result.values[i] = self.values[i] + x.get_values()[i];
            }
            result
        }

        pub fn scalar_sum(&self, x: &Vector) -> i32 {
            let mut sum = 0;
            for i in 0..VECTOR_LENGTH {
                sum += self.values[i] + x.get_values()[i];
            }
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::vector::Vector;

    #[test]
    fn new_vector() {
        assert_eq!([0, 0, 0], Vector::new().get_values());
    }

    #[test]
    fn empty_vector_adding() {
        let a = Vector::new();
        let b = a.vector_sum(&Vector::new());

        assert_eq!(Vector::new(), b);
        assert_eq!(Vector::new().get_values(), b.get_values());
    }

    #[test]
    fn one_vector_adding() {
        let x = Vector::init([1, 1, 1]);
        let y = Vector::init([2, 2, 2]);
        let result = x.vector_sum(&y);

        assert_eq!([3, 3, 3], result.get_values());
    }

    #[test]
    fn one_scalar_adding() {
        assert_eq!(0, Vector::new().scalar_sum(&Vector::new()));
        assert_eq!(3, Vector::init([1, 1, 1]).scalar_sum(&Vector::new()));
    }
}
