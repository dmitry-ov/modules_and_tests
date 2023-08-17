#[allow(dead_code)]

pub mod vec3 {
    pub const VEC3_LEN: usize = 3;
    pub type Vec3 = [i32; VEC3_LEN];

    pub fn default_vec3() -> Vec3 {
        [0; 3]
    }

    pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
        let mut c = default_vec3();
        for i in 0..3 {
            c[i] = a[i] + b[i];
        }
        c
    }

    pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += a[i] + b[i];
        }
        c
    }
}

#[cfg(test)]
mod tests {

    use crate::vec3::vec3::{default_vec3, vec3_scalar_sum, vec3_vector_sum};

    #[test]
    fn test_default_vec3() {
        let vec3 = default_vec3();
        assert_eq!([0, 0, 0], vec3);
    }

    #[test]
    fn test_vec3_vector_sum() {
        assert_eq!(
            default_vec3(),
            vec3_vector_sum(default_vec3(), default_vec3())
        );
        let one = [1, 1, 1];
        assert_eq!(one, vec3_vector_sum(default_vec3(), one));
        assert_eq!([3, 3, 3], vec3_vector_sum(one, [2, 2, 2]));
        assert_eq!([-1, -1, 2], vec3_vector_sum([0, -1, 1], [-1, 0, 1]));
    }

    #[test]
    fn test_vec3_scalar_sum() {
        assert_eq!(0, vec3_scalar_sum(default_vec3(), default_vec3()));
        let one = [1, 1, 1];
        assert_eq!(3, vec3_scalar_sum(default_vec3(), one));
        assert_eq!(6, vec3_scalar_sum(one, one));
    }
}
