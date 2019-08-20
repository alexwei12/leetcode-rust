pub struct Alg {}

impl Alg {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Alg::add(1, 2));
        assert_ne!(4, Alg::add(1, 2));
    }
}