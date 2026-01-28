// 1431. Kids With the Greatest Number of Candies
//
// There are n kids with candies. You are given an integer array candies, where each candies[i] represents the number of candies the ith kid has,
// and an integer extraCandies, denoting the number of extra candies that you have.
//
// Return a boolean array result of length n, where result[i] is true if, after giving the ith kid all the extraCandies,
// they will have the greatest number of candies among all the kids, or false otherwise.
//
// Note that multiple kids can have the greatest number of candies.

// TODO: Complete this function.
fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test1() {
        let candies = vec![2, 3, 5, 11, 3];
        let extraCandies = 3;
        let ans = kids_with_candies(candies, extra_candies);
        assert_eq(ans, vec![true, true, true, false, true])
    }

    #[test]
    fn simple_test2() {
        let candies = vec![4, 2, 1, 1, 2];
        let extraCandies = 1;
        let ans = kids_with_candies(candies, extra_candies);
        assert_eq(ans, vec![true, false, false, false, false])
    }

    #[test]
    fn simple_test3() {
        let candies = vec![12, 1, 12];
        let extraCandies = 10;
        let ans = kids_with_candies(candies, extra_candies);
        assert_eq(ans, vec![true, false, true])
    }
}
