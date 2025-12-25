/*
 * @lc app=leetcode.cn id=2090 lang=typescript
 *
 * [2090] 半径为 k 的子数组平均值
 */

// @lc code=start
function getAverages(nums: number[], k: number): number[] {
    let ans: number[] = new Array(nums.length).fill(-1);
    let sum = 0;

    let len = 1 + 2 * k;

    for(let i = 0; i < nums.length; i++){
        let left = i - len + 1;

        sum += nums[i];

        if(left < 0) {
            continue
        }

        ans[i - k] = Math.floor(sum / len);

        sum -= nums[i];

    }

    return ans;
};
// @lc code=end

