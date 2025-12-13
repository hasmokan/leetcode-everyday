/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] 搜索插入位置
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();

        let mut left = 0i32;
        let mut right = (len as i32) - 1;

        while left <= right {
            let mid = (left + (right - left) / 2) as usize;

            if nums[mid] < target {
                left = mid as i32 + 1;
            } else if nums[mid] > target {
                right = mid as i32 - 1;
            } else {
                return mid as i32;
            }
        }

        return left;
    }
}
// @lc code=end

