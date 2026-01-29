// 392. Is Subsequence
//
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by
// deleting some (can be none) of the characters without disturbing the relative positions
// of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

fn is_subsequence(s: String, t: String) -> bool {
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
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let ans = is_subsequence(s, t);
        assert!(ans);
    }

    #[test]
    fn case2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let ans = is_subsequence(s, t);
        assert!(!ans);
    }
}
