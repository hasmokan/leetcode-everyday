/*
 * @lc app=leetcode.cn id=209 lang=typescript
 *
 * [209] 长度最小的子数组
 */

// @lc code=start
function minSubArrayLen(target: number, nums: number[]): number {
    let left = 0;
    let ans = Infinity;
    let sum = 0;

    for(let right = 0; right < nums.length; right++){
        sum += nums[right];

        while(sum >= target){
            ans = Math.min(ans, right - left + 1)
            sum -= nums[left]
            left += 1;  
        }
    }

    return ans;
};
// @lc code=end

