// 238. Product of Array Except Self
//
// Given an integer array nums, return an array answer such that answer[i] is equal to the product of
// all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using the division operation.

// TODO: Complete this function.
fn product_except_self(nums: Vec<i32>) -> Vec<i32> {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test1() {
        let nums = vec![1, 2, 3, 4];
        let ans = product_except_self(nums);
        assert_eq(ans, vec![24, 12, 8, 6])
    }

    #[test]
    fn simple_test2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let ans = product_except_self(nums);
        assert_eq(ans, vec![0, 0, 9, 0, 0])
    }
}
