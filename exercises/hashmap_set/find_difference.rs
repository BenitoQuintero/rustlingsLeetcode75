// 2215. Find the Difference of Two Arrays
//
// Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:
// - answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
// - answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
//
// Note that the integers in the lists may be returned in any order.

fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    // TODO: Complete this function.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    use super::*;

    fn relaxed_inner_eq(a: &[Vec<i32>], b: &[Vec<i32>]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        a.iter().zip(b).all(|(x, y)| {
            let mut x = x.clone();
            let mut y = y.clone();
            x.sort();
            y.sort();
            x == y
        })
    }

    #[test]
    fn case1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let ans = find_difference(nums1, nums2);
        let t = relaxed_inner_eq(&ans, &[vec![1, 3], vec![4, 6]]);
        assert!(t);
    }

    #[test]
    fn case2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let ans = find_difference(nums1, nums2);
        let t = relaxed_inner_eq(&ans, &[vec![3], vec![]]);
        assert!(t);
    }
}
