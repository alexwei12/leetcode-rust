struct Solution {

}


impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::from("");
        if strs.len() == 0 {
            return result
        }
        for i in 1..=strs[0].len() {
            let w = &strs[0][0..i];
            let mut is_match = false;
            for j in 1..strs.len() {
                if i > strs[j].len() || w != &strs[j][0..i] {
                    is_match = false;
                    return result;
                }
            }
            result = String::from(w);
        }
        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("fl".to_string(), Solution::longest_common_prefix(vec!("flower".to_string(),
                                                           "flow".to_string(),
                                                           "flight".to_string())));
    }

    #[test]
    fn test_2() {
        assert_eq!("".to_string(), Solution::longest_common_prefix(vec!("dog".to_string(),
                                                                          "racecar".to_string(),
                                                                          "car".to_string())));
        }

    #[test]
    fn test_3() {
        assert_eq!("flower".to_string(), Solution::longest_common_prefix(vec!("flower".to_string())));
    }

    #[test]
    fn test_4() {
        assert_eq!("a".to_string(), Solution::longest_common_prefix(vec!("a".to_string())));
    }
}