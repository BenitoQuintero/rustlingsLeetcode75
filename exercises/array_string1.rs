// You are given two strings word1 and word2.
// Merge the strings by adding letters in alternating order, starting with word1.
// If a string is longer than the other, append the additional letters onto the end of the merged string.
//
// Return the merged string.

// TODO: Complete this function.
fn merge_alternately(word1: String, word2: String) -> String {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_length_words() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        let ans = merge_alternately(word1, word2);
        assert_eq!(ans, String::from("apbqcr"));
    }

    #[test]
    fn first_word_longer() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        let ans = merge_alternately(word1, word2);
        assert_eq!(ans, String::from("apbqrs"));
    }

    #[test]
    fn second_word_longer() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        let ans = merge_alternately(word1, word2);
        assert_eq!(ans, String::from("apbqcd"));
    }
}
