/*
 * @lc app=leetcode.cn id=643 lang=typescript
 *
 * [643] 子数组最大平均数 I
 */

// @lc code=start
function findMaxAverage(nums: number[], k: number): number {
    let ans = 0;
    let sum = 0;

    for(let i = 0; i < nums.length; i++){
        console.log(ans, sum, k)
        let left = i - k + 1;

        sum += nums[i];

        if(left < 0) continue

        ans = Math.max(ans, sum / k)



        sum -= nums[left]; 
    }

    return ans
};
// @lc code=end

