// 151. Reverse Words in a String
//
// Given an input string s, reverse the order of the words.
//
// A word is defined as a sequence of non-space characters.
// The words in s will be separated by at least one space.
//
// Return a string of the words in reverse order concatenated by a single space.
//
// Note that s may contain leading or trailing spaces or multiple spaces between two words.
// The returned string should only have a single space separating the words.
// Do not include any extra spaces.

fn reverse_words(s: String) -> String {
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
        let s = String::from("the sky is blue");
        let ans = reverse_words(s);
        assert_eq!(ans, String::from("blue is sky the"));
    }

    #[test]
    fn case2() {
        let s = String::from("  hello world  ");
        let ans = reverse_words(s);
        assert_eq!(ans, String::from("world hello"));
    }

    #[test]
    fn case3() {
        let s = String::from("a good   example");
        let ans = reverse_words(s);
        assert_eq!(ans, String::from("example good a"));
    }
}
