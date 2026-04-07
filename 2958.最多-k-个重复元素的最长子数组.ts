/*
 * @lc app=leetcode.cn id=2958 lang=typescript
 *
 * [2958] 最多 K 个重复元素的最长子数组
 */

// @lc code=start
function maxSubarrayLength(nums: number[], k: number): number {
    let left = 0;
    let ans = 0;

    let cnt = new Map()

    for (let right = 0; right < nums.length; right++) {
        cnt.set(nums[right], (cnt.get(nums[right]) ?? 0) + 1)


        while ((cnt.get(nums[right] ?? 0) > k)) {
            cnt.set(nums[left], cnt.get(nums[left]) - 1)
            left++
        }


        ans = Math.max(ans, right - left + 1)
    }

    return ans;

};
// @lc code=end

