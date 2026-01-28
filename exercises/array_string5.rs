// 345. Reverse Vowels of a String
//
// Given a string s, reverse only all the vowels in the string and return it.
//
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

// TODO: Complete this function.
fn reverse_vowels(s: String) -> String {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = String::from("IceCreAm");
        let ans = reverse_vowels(s);
        assert_eq!(ans, String::from("AceCreIm"))
    }

    #[test]
    fn case2() {
        let s = String::from("leetcode");
        let ans = reverse_vowels(s);
        assert_eq!(ans, String::from("leotcede"))
    }
}
