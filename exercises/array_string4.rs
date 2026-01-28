// 605. Can Place Flowers
//
// You have a long flowerbed in which some of the plots are planted, and some are not.
// However, flowers cannot be planted in adjacent plots.
//
// Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty,
// and an integer n, return true if n new flowers can be planted in the flowerbed without violating the
// no-adjacent-flowers rule and false otherwise.

// TODO: Complete this function.
fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let ans = can_place_flowers(flowerbed, n);
        assert!(ans)
    }

    #[test]
    fn case2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let ans = can_place_flowers(flowerbed, n);
        assert!(!ans)
    }
}
