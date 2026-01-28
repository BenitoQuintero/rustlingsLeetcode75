// 334. Increasing Triplet Subsequence
//
// Given an integer array nums, return true if there exists a triple of indices
// (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k].
// If no such indices exists, return false.

// TODO: Complete this function.
fn increasing_triplet(nums: Vec<i32>) -> bool {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 4, 5];
        let ans = increasing_triplet(nums);
        assert!(ans)
    }

    #[test]
    fn case2() {
        let nums = vec![5, 4, 3, 2, 1];
        let ans = increasing_triplet(nums);
        assert!(!ans)
    }

    #[test]
    fn case3() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let ans = increasing_triplet(nums);
        assert!(ans)
    }
}
