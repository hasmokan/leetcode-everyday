/*
 * @lc app=leetcode.cn id=2300 lang=rust
 *
 * [2300] 咒语和药水的成功对数
 */

// @lc code=start
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions; 
        let mut ans: Vec<i32> = vec![];
                
        potions.sort_unstable_by(|a, b| a.cmp(b));

        spells.iter().for_each(|item| {
            let mut left = 0;
            let mut right = potions.len();

            while left < right {
                let mid = left + (right - left) / 2;

                if (((potions[mid] as i64) * (*item as i64)) as i64) < success {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            ans.push((potions.len() as i32 - left as i32) as i32);
        });


        ans
    }
}
// @lc code=end

