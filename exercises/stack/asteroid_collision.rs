// 735. Asteroid Collision
//
// We are given an array asteroids of integers representing asteroids in a row.
// The indices of the asteroid in the array represent their relative position in space.
//
// For each asteroid, the absolute value represents its size, and the sign represents
// its direction (positive meaning right, negative meaning left).
// Each asteroid moves at the same speed.
//
// Find out the state of the asteroids after all collisions. If two asteroids meet,
// the smaller one will explode. If both are the same size, both will explode.
// Two asteroids moving in the same direction will never meet.

fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
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
        let asteroids = vec![5, 10, -5];
        let ans = asteroid_collision(asteroids);
        assert_eq!(ans, vec![5, 10]);
    }

    #[test]
    fn case2() {
        let asteroids = vec![8, -8];
        let ans = asteroid_collision(asteroids);
        assert_eq!(ans, vec![]);
    }

    #[test]
    fn case3() {
        let asteroids = vec![10, 2, -5];
        let ans = asteroid_collision(asteroids);
        assert_eq!(ans, vec![10]);
    }

    #[test]
    fn case4() {
        let asteroids = vec![3, 5, -6, 2, -1, 4];
        let ans = asteroid_collision(asteroids);
        assert_eq!(ans, vec![-6, 2, 4]);
    }
}
