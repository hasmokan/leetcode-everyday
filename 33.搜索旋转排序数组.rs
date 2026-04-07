/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            // println!("{} {} {}", left, right , mid);

            if nums[mid] == target {
                return mid as i32;
            } else {
                if nums[mid] > nums[left] {
                    if nums[left] <= target && nums[mid] > target {
                        right = mid;
                    } else {
                        left = mid + 1
                    }
                } else {
                    if nums[mid] < target && nums[right - 1] >= target {
                        left = mid + 1;
                    } else {
                        print!("{} {} {} {}", nums[mid] , target, nums[right-1], 2);
                        right = mid;
                    }
                }
            }
        }


        -1
    }
}
// @lc code=end

