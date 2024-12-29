struct Solution {}

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut xy_area = 0;
        let mut yz_area = 0;
        let mut zx_area = 0;

        for i in 0..n {
            let mut max_yz = 0;
            let mut max_zx = 0;
            for j in 0..n {
                if grid[i][j] > 0 {
                    xy_area += 1;
                }
                max_yz = max_yz.max(grid[i][j]);
                max_zx = max_zx.max(grid[j][i]);
            }
            yz_area += max_yz;
            zx_area += max_zx;
        }

        xy_area + yz_area + zx_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection_area() {
        // Test case 1: grid = [[1,2],[3,4]]
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);

        // Test case 2: grid = [[2]]
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);

        // Test case 3: grid = [[1,0],[0,2]]
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
