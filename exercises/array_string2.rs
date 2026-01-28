// 1071. Greatest Common Divisor of Strings
//
// For two strings s and t, we say "t divides s" if and only if
// s = t + t + t + ... + t + t (i.e., t is concatenated with itself one or more times).
//
// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

// TODO: Complete this function.
fn gcd_of_strings(str1: String, str2: String) -> String {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test1() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from("ABC"));
    }

    #[test]
    fn simple_test2() {
        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from("AB"));
    }

    #[test]
    fn no_substring_test() {
        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from(""));
    }

    #[test]
    fn failed_test2() {
        let str1 = String::from("AAAAAB");
        let str2 = String::from("AAA");
        let ans = gcd_of_strings(str1, str2);
        assert_eq!(ans, String::from(""));
    }
}
