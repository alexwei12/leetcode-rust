struct Solution {

}


impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let lower_bound: i64 = - (2_i64.pow(31));
        if (n as i64) == lower_bound  {
            return false;
        }

        return (n & n - 1) == 0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::is_power_of_two(0));
        assert_eq!(true, Solution::is_power_of_two(2));
        assert_eq!(false, Solution::is_power_of_two(5));
    }
}