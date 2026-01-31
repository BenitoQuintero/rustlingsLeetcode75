// 2352. Equal Row and Column Pairs
//
// Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such
// that row ri and column cj are equal.
//
// A row and column pair is considered equal if they contain the same elements in the same
// order (i.e., an equal array).

fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
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
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        let ans = equal_pairs(grid);
        assert_eq!(ans, 1);
    }

    #[test]
    fn case2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        let ans = equal_pairs(grid);
        assert_eq!(ans, 3);
    }
}
