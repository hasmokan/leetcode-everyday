/*
 * @lc app=leetcode.cn id=154 lang=rust
 *
 * [154] 寻找旋转排序数组中的最小值 II
 */

// @lc code=start
impl Solution {
    // 开区间 [left, right) 实现
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() ;  



        while left < right {
        
            // if right - left == 1 {
            //     return nums[left];
            // }

            if right - left == 2 {
                return nums[left].min(nums[right - 1]);
            }


            let mid = left + (right - left) / 2;
            
            if nums[mid] > nums[right-1] {
                left = mid + 1;
            } else if nums[mid] < nums[right-1] {
                right = mid + 1; 
            } else {
                right -= 1;
            }
        }
    
        nums[left]
    }
}
// @lc code=end

