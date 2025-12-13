/*
 * @lc app=leetcode.cn id=34 lang=rust
 *
 * [34] 在排序数组中查找元素的第一个和最后一个位置
 */

// @lc code=start
impl Solution {


    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {

        pub fn getRightBorder(nums: &Vec<i32>, target: i32) -> i32 {
            let mut left: i32 = 0;
            let mut right = (nums.len() - 1) as i32;
    
            let mut right_border: i32 = (nums.len() + 1) as i32; 
    
            while left <= right {
                let mid = left + (right - left) / 2 ;
    
                if nums[mid as usize] <= target {
                    left = mid + 1;
                    right_border = left
                }  else {
                    right = mid - 1;
                }
            }
    
            right_border
    
        }
    
    
        pub fn getLeftBorder(nums: &Vec<i32>, target: i32) -> i32 {
            let mut left: i32 = 0;
            let mut right = (nums.len() - 1) as i32;
    
            let mut left_border: i32 = -2; 
    
            while left <= right {
                let mid = left + (right - left) / 2 ;
    
                if nums[mid as usize] < target {
                    left = mid + 1;
    
                }  else {
                    right = mid - 1;
                    left_border = right
                }
            }
    
            left_border
        }

        let left = getLeftBorder(&nums, target);
        let right = getRightBorder(&nums, target);

        

        if left == -2 || right == (nums.len() + 1) as i32 {return vec![-1, -1];}
        else if nums[(left + 1) as usize] != target  {return vec![-1, -1];}
        else {return vec![left + 1, right - 1]};

    }
}
// @lc code=end

