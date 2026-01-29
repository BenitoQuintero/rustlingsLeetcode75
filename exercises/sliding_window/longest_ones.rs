// 1004. Max Consecutive Ones III
//
// Given a binary array nums and an integer k, return the maximum number of
// consecutive 1's in the array if you can flip at most k 0's.

fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
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
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;
        let ans = longest_ones(nums, k);
        assert_eq!(ans, 6);
    }

    #[test]
    fn case2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;
        let ans = longest_ones(nums, k);
        assert_eq!(ans, 10);
    }
}
