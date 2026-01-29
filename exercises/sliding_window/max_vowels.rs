// 1456. Maximum Number of Vowels in a Substring of Given Length
//
// Given a string s and an integer k, return the maximum number of vowel letters
// in any substring of s with length k.
//
// Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.

fn max_vowels(s: String, k: i32) -> i32 {
    // TODO: Complete this function.
}

fn is_vowel(ch: char) -> bool {
    ch == 'a'
        || ch == 'e'
        || ch == 'i'
        || ch == 'o'
        || ch == 'u'
        || ch == 'A'
        || ch == 'E'
        || ch == 'I'
        || ch == 'O'
        || ch == 'U'
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let s = String::from("abciiidef");
        let k = 3;
        let ans = max_vowels(s, k);
        assert_eq!(ans, 3);
    }

    #[test]
    fn case2() {
        let s = String::from("aeiou");
        let k = 2;
        let ans = max_vowels(s, k);
        assert_eq!(ans, 2);
    }

    #[test]
    fn case3() {
        let s = String::from("leetcode");
        let k = 3;
        let ans = max_vowels(s, k);
        assert_eq!(ans, 2);
    }
}
