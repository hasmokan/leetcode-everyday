/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut processDividend = dividend.abs();
        let mut processDivisor = divisor.abs();

        let l = if dividend < 0 {1} else {0};
        let r = if divisor < 0 {1} else {0};


        let mut left = 0;
        let mut right = processDividend;

        let mut ans = processDividend;

        while left <= right {
            let mut mid = left + (right - left) / 2;
            
            if (mid  * processDivisor) < processDividend  {
                left = mid + 1;
                ans = mid;
            } else {
                right = mid - 1;
            }
        }

        if l + r == 2 || l + r == 0  {ans} else {-ans}

        
    }
}
// @lc code=end

