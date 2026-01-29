// 1493. Longest Subarray of 1's After Deleting One Element
//
// Given a binary array nums, you should delete one element from it.
//
// Return the size of the longest non-empty subarray containing
// only 1's in the resulting array.
// Return 0 if there is no such subarray.

fn longest_subarray(nums: Vec<i32>) -> i32 {
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
        let nums = vec![1, 1, 0, 1];
        let ans = longest_subarray(nums);
        assert_eq!(ans, 3);
    }

    #[test]
    fn case2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let ans = longest_subarray(nums);
        assert_eq!(ans, 5);
    }

    #[test]
    fn case3() {
        let nums = vec![1, 1, 1];
        let ans = longest_subarray(nums);
        assert_eq!(ans, 2);
    }
}
