// 2390. Removing Stars From a String
//
// You are given a string s, which contains stars *.
// In one operation, you can:
// - Choose a star in s.
// - Remove the closest non-star character to its left, as well as remove the star itself.
//
// Return the string after all stars have been removed.
//
// Note:
// - The input will be generated such that the operation is always possible.
// - It can be shown that the resulting string will always be unique.

fn remove_stars(s: String) -> String {
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
        let s = String::from("leet**cod*e");
        let ans = remove_stars(s);
        assert_eq!(ans, String::from("lecoe"));
    }

    #[test]
    fn case2() {
        let s = String::from("erase*****");
        let ans = remove_stars(s);
        assert_eq!(ans, String::from(""));
    }
}
