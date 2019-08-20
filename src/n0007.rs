pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input: i64 = x as i64;
        let mut n: i64 = 0;
        let mut y: i64 = 0;
        let upper_bound: i64 = 2_i64.pow(31) - 1;
        let lower_bound: i64 = - (2_i64.pow(31));
        while input != 0 {
            y = input % 10;
            n = n * 10 + y;
            input = input / 10;
        }
        if n > upper_bound || n < lower_bound {
            return 0;
        }
        n as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(23, Solution::reverse(320));
        assert_eq!(0, Solution::reverse(1056389759));
    }
}