// 1679. Max Number of K-Sum Pairs
//
// You are given an integer array nums and an integer k.
//
// In one operation, you can pick two numbers from the array
// whose sum equals k and remove them from the array.
//
// Return the maximum number of operations you can perform on the array.

fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
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
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        let ans = max_operations(nums, k);
        assert_eq!(ans, 2)
    }

    #[test]
    fn case2() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;
        let ans = max_operations(nums, k);
        assert_eq!(ans, 1)
    }
}
