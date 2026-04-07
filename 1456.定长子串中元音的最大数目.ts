/*
 * @lc app=leetcode.cn id=1456 lang=typescript
 *
 * [1456] 定长子串中元音的最大数目
 */

// @lc code=start
function maxVowels(s: string, k: number): number {
    let cnt = 0;

    let max = -Infinity;

    let arr = ['a', 'e', 'i', 'o', 'u']

    for (let i = 0; i < s.length; i++) {
        let left = i - k + 1;

        if (arr.includes(s[i])) { cnt++; }
        if (left < 0) continue
        max = Math.max(cnt, max);

        if (arr.includes(s[left])) { cnt--; }
    }

    return max;
};
// @lc code=end
