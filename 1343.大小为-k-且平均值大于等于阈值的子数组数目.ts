/*
 * @lc app=leetcode.cn id=1343 lang=typescript
 *
 * [1343] 大小为 K 且平均值大于等于阈值的子数组数目
 */

// @lc code=start
function numOfSubarrays(arr: number[], k: number, threshold: number): number {
    let ans = 0;

    let sum = 0;

    for(let i = 0; i < arr.length; i++){
        let left = i - k + 1;

        sum += arr[i];

        if(left < 0) continue;

        if(sum / k >= threshold) ans += 1;

        sum -= arr[left];
    }

    return ans
};
// @lc code=end

