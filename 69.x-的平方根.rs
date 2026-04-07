/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根 
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;

         let target = x as i64;

         let mut ans = 0;

    
        while left <= right {
            let mid = left + (right - left) / 2;

            if (mid as i64 * mid as i64) <= target {
                left = mid + 1;
               ans = mid;

            }  else {
               right = mid -1;

            }

        }   

        ans
    }
}
// @lc code=end

