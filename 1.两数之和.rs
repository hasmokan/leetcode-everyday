/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */


struct Solution;

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![map[&complement], i as i32];
            }
            map.insert(num, i as i32);
        }
        let mut map 
        vec![]
    }
}
// @lc code=end