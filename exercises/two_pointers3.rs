// 11. Container With Most Water
//
// You are given an integer array height of length n. There are n
// vertical lines drawn such that the two endpoints of the ith line
// are (i, 0) and (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such
// that the container contains the most water.
//
// Return the maximum amount of water a container can store.
//
// Notice that you may not slant the container.

// TODO: Complete this function.
fn max_area(height: Vec<i32>) -> i32 {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let ans = max_area(height);
        assert_eq!(ans, 49)
    }

    #[test]
    fn case2() {
        let height = vec![1, 1];
        let ans = max_area(height);
        assert_eq!(ans, 1)
    }
}
