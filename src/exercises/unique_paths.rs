//A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
//
// The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
//
// How many possible unique paths are there?
// Input: m = 3, n = 7
// Output: 28
// Input: m = 7, n = 3
// Output: 28
// Input: m = 3, n = 3
// Output: 6

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut grid = vec![vec![1; n]; m];
    for i in 1..m {
        for j in 1..n {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }
    grid[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(3, 3), 6);
        assert_eq!(unique_paths(7, 3), 28);
    }
}