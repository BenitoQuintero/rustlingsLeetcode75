// 394. Decode String
//
// Given an encoded string, return its decoded string.
//
// The encoding rule is: k[encoded_string], where the encoded_string inside the
// square brackets is being repeated exactly k times. Note that k is guaranteed
// to be a positive integer.
//
// You may assume that the input string is always valid; there are no extra white
// spaces, square brackets are well-formed, etc. Furthermore, you may assume that
// the original data does not contain any digits and that digits are only for those
// repeat numbers, k. For example, there will not be input like 3a or 2[4].
//
// The test cases are generated so that the length of the output will never exceed 10^5.

fn decode_string(s: String) -> String {
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
        let s = String::from("3[a]2[bc]");
        let ans = decode_string(s);
        assert_eq!(ans, String::from("aaabcbc"));
    }

    #[test]
    fn case2() {
        let s = String::from("3[a2[c]]");
        let ans = decode_string(s);
        assert_eq!(ans, String::from("accaccacc"));
    }

    #[test]
    fn case3() {
        let s = String::from("2[abc]3[cd]ef");
        let ans = decode_string(s);
        assert_eq!(ans, String::from("abcabccdcdcdef"));
    }
}
