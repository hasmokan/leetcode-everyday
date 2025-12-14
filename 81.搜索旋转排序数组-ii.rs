/*
 * @lc app=leetcode.cn id=81 lang=rust
 *
 * [81] 搜索旋转排序数组 II
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            println!("{} {} {}", left, right, mid);

            if nums[mid] == target {
                return true;
            }
            if nums[left] == nums[mid] {
                left += 1;
                continue;
            }
            else if nums[mid] > nums[left] {
                if target >= nums[left] && target < nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            else {
                if target > nums[mid] && target <= nums[right - 1] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }


        false
    }
}
// @lc code=end

