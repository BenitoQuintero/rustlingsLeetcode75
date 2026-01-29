// 643. Maximum Average Subarray I
//
// You are given an integer array nums consisting of n elements, and an integer k.
//
// Find a contiguous subarray whose length is equal to k that has the maximum average
// value and return this value. Any answer with a calculation error less than 10-5 will
// be accepted.

fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
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
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let ans = find_max_average(nums, k);
        assert_eq!(ans, 12.75);
    }

    #[test]
    fn case2() {
        let nums = vec![5];
        let k = 1;
        let ans = find_max_average(nums, k);
        assert_eq!(ans, 5.0);
    }
}
