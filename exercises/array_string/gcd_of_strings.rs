// 1071. Greatest Common Divisor of Strings
//
// For two strings s and t, we say "t divides s" if and only if
// s = t + t + t + ... + t + t (i.e., t is concatenated with itself one or more times).
//
// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

fn gcd_of_strings(str1: String, str2: String) -> String {
    // TODO: Complete this function.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from("ABC"));
    }

    #[test]
    fn case2() {
        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from("AB"));
    }

    #[test]
    fn case3() {
        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from(""));
    }

    #[test]
    fn case4() {
        let str1 = String::from("AAAAAB");
        let str2 = String::from("AAA");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from(""));
    }
}
