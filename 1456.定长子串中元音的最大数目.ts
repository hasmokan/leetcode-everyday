/*
 * @lc app=leetcode.cn id=1456 lang=typescript
 *
 * [1456] 定长子串中元音的最大数目
 */

// @lc code=start
function maxVowels(s: string, k: number): number {
    const arr = s.split('');

    let ans = 0;

    let vowel = 0;
    for(let i = 0; i < arr.length; i++){
        if(arr[i] === 'a' || arr[i] === 'e' || arr[i] === 'i' || arr[i] === 'o' || arr[i] === 'u'){
            vowel++
        }

        let left = i - k + 1;

        if(left < 0) continue

        ans = Math.max(ans, vowel);

        if(arr[left] === 'a' || arr[left] === 'e' || arr[left] === 'i' || arr[left] === 'o' || arr[left] === 'u'){
            vowel--;
        }
    }

    return  ans
};
// @lc code=end

