/*
 * @lc app=leetcode.cn id=74 lang=rust
 *
 * [74] 搜索二维矩阵
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left: i32 = 0;
        let mut right: i32 = matrix.len() as i32 - 1;

        let mut ftarget = -2;
        while left <= right {
            let mid = left + (right - left) / 2;
            println!("{}", mid);
            if matrix[mid as usize][0] > target {
                right = mid - 1;
            } else if matrix[mid as usize][0] < target {
                left = mid + 1;
            } else {
                return true;
            }

            println!("{} {}", left, right)

        }

        println!("{} {}", left, right);

        right = if right < 0 {0} else {right};

        let mut sleft: i32 = 0;
        let mut sright: i32 = matrix[0].len() as i32 - 1;

        while sleft <= sright {
            let mid = sleft + (sright - sleft) / 2;

            if matrix[right as usize][mid as usize] > target {
                sright = mid - 1;
            } else if matrix[right as usize][mid as usize] < target {
                sleft = mid + 1;
            } else {
                return true;
            }
        }

        return false
    }
}
// @lc code=end

