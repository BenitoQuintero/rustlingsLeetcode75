// 1732. Find the Highest Altitude
//
// There is a biker going on a road trip. The road trip consists of n + 1 points
// at different altitudes. The biker starts his trip on point 0 with altitude equal 0.
//
// You are given an integer array gain of length n where gain[i] is the net gain in
// altitude between points i and i + 1 for all (0 <= i < n).
// Return the highest altitude of a point.

fn largest_altitude(gain: Vec<i32>) -> i32 {
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
        let gain = vec![-5, 1, 5, 0, -7];
        let ans = largest_altitude(gain);
        assert_eq!(ans, 1);
    }

    #[test]
    fn case2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let ans = largest_altitude(gain);
        assert_eq!(ans, 0);
    }
}
