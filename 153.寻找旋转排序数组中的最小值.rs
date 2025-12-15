/*
 * @lc app=leetcode.cn id=153 lang=rust
 *
 * [153] 寻找旋转排序数组中的最小值
 */

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            println!("{} {} {}", mid, left, right);


            if nums[mid] > nums[0] {
                left = mid + 1;
            } else {
                println!("{} {} {} ttt", mid, left, right);
                if nums[mid] <= nums[nums.len() - 1] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }

        // if nums[left] < nums[right] {nums[left]} else {nums[right]}


        if nums[if left >= nums.len() {left - 1} else {left }] < nums[0] {nums[left]} else {nums[0]}


    }
}
// @lc code=end

