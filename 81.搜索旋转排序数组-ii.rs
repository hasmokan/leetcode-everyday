/*
 * @lc app=leetcode.cn id=81 lang=rust
 *
 * [81] 搜索旋转排序数组 II
 */

// @lc code=start
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() - 1;  // 使用闭区间，避免越界

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return true;
            }

            // 处理重复元素
            if nums[left] == nums[mid] && nums[mid] == nums[right] {
                left += 1;
                if right > 0 {
                    right -= 1;
                }
                continue;
            }

            // 判断左半部分是否有序
            if nums[mid] >= nums[left] {
                // 左半部分有序
                if target >= nums[left] && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // 右半部分有序
                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        false
    }
}
// @lc code=end

