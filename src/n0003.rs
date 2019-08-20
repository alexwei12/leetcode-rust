pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::length_of_longest_substring(String::from("abcabcbb")));
        assert_eq!(1, Solution::length_of_longest_substring(String::from("aaaaaa")));
    }
}