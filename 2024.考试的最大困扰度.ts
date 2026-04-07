/*
 * @lc app=leetcode.cn id=2024 lang=typescript
 *
 * [2024] 考试的最大困扰度
 */

// @lc code=start
function maxConsecutiveAnswers(answerKey: string, k: number): number {
    let left = 0;
    let cnt: Record<string, number> = { "T": 0, "F": 0 };
    let ans = 0;

    for (let right = 0; right < answerKey.length; right++) {

        cnt[answerKey[right]]++

        while (cnt['T'] > k && cnt['F'] > k) {
            cnt[answerKey[left]]--
            left++
        }

        ans = Math.max(right - left + 1, ans)
    }

    return ans
};
// @lc code=end

