/*
 * @lc app=leetcode.cn id=2389 lang=rust
 *
 * [2389] 和有限的最长子序列
 */

// @lc code=start
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        let mut prefix: Vec<i32> = vec![0; nums.len()];

        let mut ans = vec![];

        prefix[0] = nums[0];

        for i in 1..=nums.len()-1 {
            prefix[i] = prefix[i-1] + nums[i];
        }


        println!("{:?}", prefix);

        queries.iter().for_each(|item| {
            let mut left = 0;
            let mut right = prefix.len();
            

            while left < right {
                let mid = left + (right - left) / 2;
                println!("{} {} {}", left, right , mid);
    
                if prefix[mid] <= *item {
                    left = mid + 1;
                } else  {
                    right = mid;
                }
                //  else {
                //     flag = true;
                //     ans.push(mid as i32 + 1);
                // }
            }
            
                ans.push(left as i32);
        });


        ans

    }
}
// @lc code=end

