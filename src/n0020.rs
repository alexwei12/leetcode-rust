struct Solution {

}


impl Solution {
    pub fn is_valid(s: String) -> bool {
        return false;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_valid(String::from("()")),  true);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")),  true);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::is_valid(String::from("(]")),  false);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::is_valid(String::from("([)]")),  false);
    }
    #[test]
    fn test_5() {
        assert_eq!(Solution::is_valid(String::from("{[]}")),  true);
    }
}