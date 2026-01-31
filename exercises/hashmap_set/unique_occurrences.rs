// 1207. Unique Number of Occurrences
//
// Given an array of integers arr, return true if the number of occurrences
// of each value in the array is unique or false otherwise.

fn unique_occurrences(arr: Vec<i32>) -> bool {
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
        let arr = vec![1, 2, 2, 1, 1, 3];
        let ans = unique_occurrences(arr);
        assert!(ans);
    }

    #[test]
    fn case2() {
        let arr = vec![1, 2];
        let ans = unique_occurrences(arr);
        assert!(!ans);
    }

    #[test]
    fn case3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let ans = unique_occurrences(arr);
        assert!(ans);
    }
}
