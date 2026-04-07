/*
 * @lc app=leetcode.cn id=2379 lang=typescript
 *
 * [2379] 得到 K 个黑块的最少涂色次数
 */

// @lc code=start
function minimumRecolors(blocks: string, k: number): number {

    let ans = Infinity

    let sum = 0;

    for(let i = 0; i < blocks.length; i++){
        let left = i - k + 1;

        if(blocks[i] === 'W') sum += 1;

        if(left < 0) continue

        ans = Math.min(sum, ans);

        if(blocks[left] === 'W') sum -= 1;

    }

    return ans;
};
// @lc code=end

