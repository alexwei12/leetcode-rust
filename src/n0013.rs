struct Solution {

}



fn char_to_int(c: &str) ->i32 {
    match c {
        "I" => {return 1;}
        "V" => {return 5;}
        "X" => {return 10;}
        "L" => {return 50;}
        "C" => {return 100;}
        "D" => {return 500;}
        "M" => {return 1000;}
        _ => panic!("invalid char = {}", c)
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut result: i32 = char_to_int(&s[0..1]);

        for i in 1..s.len() {
            let pre: i32 = char_to_int(&s[i-1..i]);
            let cur: i32 = char_to_int(&s[i..i+1]);
            if pre < cur {
                result = result - pre + (cur - pre);
            } else {
                result += cur;
            }

        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::roman_to_int(String::from("III")));
        assert_eq!(4, Solution::roman_to_int(String::from("IV")));
        assert_eq!(9, Solution::roman_to_int(String::from("IX")));
        assert_eq!(58, Solution::roman_to_int(String::from("LVIII")));
        assert_eq!(1994, Solution::roman_to_int(String::from("MCMXCIV")));
    }
}
//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000
// 示例 1:
//
// 输入: "III"
//输出: 3
//
// 示例 2:
//
// 输入: "IV"
//输出: 4
//
// 示例 3:
//
// 输入: "IX"
//输出: 9
//
// 示例 4:
//
// 输入: "LVIII"
//输出: 58
//解释: L = 50, V= 5, III = 3.
//
//
// 示例 5:
//
// 输入: "MCMXCIV"
//输出: 1994