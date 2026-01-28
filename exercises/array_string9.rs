// 443. String Compression
//
// Given an array of characters chars, compress it using the following algorithm:
// Begin with an empty string s. For each group of consecutive repeating characters in chars:
// - If the group's length is 1, append the character to s.
// - Otherwise, append the character followed by the group's length.
//
// The compressed string s should not be returned separately, but instead, be stored in the input
// character array chars. Note that group lengths that are 10 or longer will be split into multiple
// characters in chars.
//
// After you are done modifying the input array, return the new length of the array.
//
// You must write an algorithm that uses only constant extra space.
//
// Note: The characters in the array beyond the returned length do not matter and should be ignored.

// TODO: Complete this function.
fn compress(chars: &mut Vec<char>) -> i32 {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test1() {
        let mut input = vec!["a", "a", "b", "b", "c", "c", "c"];
        let ans = compress(input);
        assert_eq(ans, 6)
    }

    #[test]
    fn simple_test2() {
        let mut input = vec!["a"];
        let ans = compress(input);
        assert_eq(ans, 1)
    }

    #[test]
    fn simple_test2() {
        let mut input = vec![
            "a", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b", "b",
        ];
        let ans = compress(input);
        assert_eq(ans, 4)
    }
}
